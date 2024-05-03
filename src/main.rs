use clap::{Parser, ValueEnum};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

#[derive(Parser, Debug)]
struct Cli {
    /// The Github URL of the repository you want to fetch
    github_url: String,
    /// Exclude pattern
    #[clap(short = 'e', long)]
    exclude: Option<String>,
    /// Include pattern
    #[clap(short = 'p', long)]
    include: Option<String>,
    /// Output format
    /// Can be one of, json, markdown
    #[clap(
        short = 'f',
        long,
        value_enum,
        rename_all = "kebab-case",
        default_value = "markdown"
    )]
    format: OutputFormat,
    /// Include hidden files
    /// By default, hidden files are not included
    /// If you want to include hidden files, set this flag to true
    #[clap(short = 'i', long, default_value = "false")]
    hidden: Option<bool>,
    /// The output file
    #[clap(short, long, default_value = "output")]
    output_file: String,
    // Output style
    // Can be one of, folder, one-file
    // #[clap(short='s', long, value_enum, rename_all="kebab-case", default_value_t = OutputStyle::OneFile)]
    // output_style: OutputStyle,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputStyle {
    Folder,
    OneFile,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Markdown,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileEntry {
    location: String,
    content: String,
}

fn does_repo_exist(github_url: &str) -> bool {
    let command = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg(github_url)
        .output()
        .expect("Failed to execute command");

    command.status.success()
}

fn clone_repo(github_url: &str) -> PathBuf {
    // Create a temporary directory to clone the repository to
    // Don't precisely know what happens with these strings yet
    // Got it from: https://stackoverflow.com/a/76378247
    let temp_dir: PathBuf = (env::temp_dir().to_string_lossy().to_string() + "repo").into();

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).expect("Failed to remove the temporary directory");
    }

    let command = Command::new("gh")
        .arg("repo")
        .arg("clone")
        .arg(github_url)
        .arg(&temp_dir)
        .output()
        .expect("Failed to execute command");

    if !command.status.success() {
        println!("Status: {:?}", command);
        panic!("Failed to clone the repository");
    }

    temp_dir
}

fn create_markdown_output(filtered_files: Vec<&PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) {
    output_file.set_extension("md");

    let mut file =
        File::create(&output_file).expect("Error creating or opening output file.");
                
    let entries: Vec<FileEntry> = filtered_files
        .iter()
        .map(|path| {
            let content = fs::read_to_string(path)
                .unwrap_or_else(|_| "Error reading file".to_string());
            FileEntry {
                location: path.display().to_string().replace(&(repo_path.to_string_lossy().to_string() + "/"), ""),
                content,
            }
        })
        .collect();
    for entry in entries {
        let markdown = format!("## {}\n{}\n", entry.location, entry.content);
        file.write_all(markdown.as_bytes())
            .expect("Error writing file contents to output file.");
    }
}

fn create_json_output(filtered_files: Vec<&PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) {
    
    output_file.set_extension("json");

    let file = File::create(&output_file).expect("Error creating or opening output file.");
    let entries: Vec<FileEntry> = filtered_files
        .iter()
        .map(|path| {
            let content = fs::read_to_string(path)
                .unwrap_or_else(|_| "Error reading file".to_string());
            FileEntry {
                location: path.display().to_string().replace(&(repo_path.to_string_lossy().to_string() + "/"), ""),
                content,
            }
        })
        .collect();
    serde_json::to_writer(&file, &entries)
        .expect("Error writing file contents to output file.");
}

fn main() {
    let args = Cli::parse();

    if !does_repo_exist(&args.github_url) {
        println!("The repository does not exist");
        return;
    }

    let repo_path = clone_repo(&args.github_url);

    let include_pattern = args.include.as_deref().unwrap_or("**/*");

    let included_file_paths = glob(&(repo_path.to_string_lossy().to_string() + "/" + include_pattern))
        .expect("Error reading temporary cloned directory.")
        .filter_map(Result::ok)
        .collect::<HashSet<PathBuf>>();

    let excluded_file_paths = match &args.exclude {
        Some(pattern) => glob(&(repo_path.to_string_lossy().to_string() + "/" + pattern))
            .expect("Failed to read exclude glob pattern")
            .filter_map(Result::ok)
            .collect::<HashSet<PathBuf>>(),
        None => HashSet::new(), // Default to an empty set when exclude is None
    };

    let filtered_files = included_file_paths
        .difference(&excluded_file_paths)
        .collect::<Vec<&PathBuf>>();

    let mut output_file = PathBuf::new();
    output_file.push(&args.output_file);

    match &args.format {
        OutputFormat::Json => {
            create_json_output(filtered_files, &repo_path, output_file);
        }
        OutputFormat::Markdown => {
            create_markdown_output(filtered_files, &repo_path, output_file);
        }
    };
    
}
