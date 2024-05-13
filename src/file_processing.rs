use std::{collections::HashSet, fs::{self, File}, io::Write, path::PathBuf};
use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileEntry {
    location: String,
    content: String,
}


pub fn create_markdown_output(filtered_files: Vec<PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) {
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

pub fn create_json_output(filtered_files: Vec<PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) {
    
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

pub fn get_filtered_files(
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