mod cli;

use glob::glob;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet, env, fs::{self, File}, io::Write, path::PathBuf, process::Command
};



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

fn clone_repo(github_url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Create a temporary directory to clone the repository to
    // Don't precisely know what happens with these strings yet
    // Got it from: https://stackoverflow.com/a/76378247
    let temp_dir = env::temp_dir().join("repo");

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    let output = Command::new("gh")
        .arg("repo")
        .arg("clone")
        .arg(github_url)
        .arg(&temp_dir)
        .output()?;

    if !output.status.success() {
        println!("Status: {:?}", output);
        return Err(From::from("Failed to clone repository"));
    }

    Ok(temp_dir)
}

fn create_markdown_output(filtered_files: Vec<PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) {
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

fn create_json_output(filtered_files: Vec<PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) {
    
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

fn get_filtered_files(
    repo_path: &PathBuf,
    include_pattern: Option<String>,
    exclude_pattern: Option<String>,
) -> Vec<PathBuf> {

    let included_file_paths = match &include_pattern {
        Some(pattern) => glob(&(repo_path.to_string_lossy().to_string() + "/" + pattern))
            .expect("Failed to read include glob pattern")
            .filter_map(Result::ok)
            .collect::<HashSet<PathBuf>>(),
        None => glob(&(repo_path.to_string_lossy().to_string() + "/**/*"))
            .expect("Error reading temporary cloned directory.")
            .filter_map(Result::ok)
            .collect::<HashSet<PathBuf>>(),
    };

    let excluded_file_paths = match &exclude_pattern {
        Some(pattern) => glob(&(repo_path.to_string_lossy().to_string() + "/" + pattern))
            .expect("Failed to read exclude glob pattern")
            .filter_map(Result::ok)
            .collect::<HashSet<PathBuf>>(),
        None => HashSet::new(), // Default to an empty set when exclude is None
    };

    let filtered_files = included_file_paths
        .difference(&excluded_file_paths)
        .cloned()
        .collect::<Vec<PathBuf>>();

    return filtered_files;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();

    if !does_repo_exist(&args.github_url) {
        println!("The repository does not exist");
        return Ok(());
    }

    let repo_path = clone_repo(&args.github_url)?;

    let filtered_files = get_filtered_files(
        &repo_path, 
        args.include, 
        args.exclude);

    let output_file = PathBuf::from(&args.output_file);

    match &args.format {
        cli::OutputFormat::Json => {
            create_json_output(filtered_files, &repo_path, output_file);
        }
        cli::OutputFormat::Markdown => {
            create_markdown_output(filtered_files, &repo_path, output_file);
        }
    };

    Ok(())
    
}
