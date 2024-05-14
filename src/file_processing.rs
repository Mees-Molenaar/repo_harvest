use std::{collections::HashSet, fs::{self, File}, io::Write, path::PathBuf};
use glob::glob;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileEntry {
    location: String,
    content: String,
}


pub fn create_markdown_output(filtered_files: Vec<PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) -> Result<()>{
    output_file.set_extension("md");
    let mut file =
        File::create(&output_file).context("Error creating or opening output file.")?;
                
    let entries = build_file_entries(&filtered_files, repo_path)?;
    
    for entry in entries {
        let markdown = format!("## {}\n{}\n", entry.location, entry.content);
        file.write_all(markdown.as_bytes()).context("Error writing file contents to output file")?;
    }
    Ok(())
}


pub fn create_json_output(filtered_files: Vec<PathBuf>, repo_path: &PathBuf, mut output_file: PathBuf) -> Result<()> {
    output_file.set_extension("json");
    let file = File::create(&output_file).expect("Error creating or opening output file.");
    
    let entries: Vec<FileEntry> = build_file_entries(&filtered_files, repo_path)?;
    serde_json::to_writer(&file, &entries).context("Error writing file contents to output file")
}

fn build_file_entries(filtered_files: &[PathBuf], repo_path: &PathBuf) -> Result<Vec<FileEntry>> {
    filtered_files.iter().map(|path| {
        let content = fs::read_to_string(path).unwrap_or_else(|_| "Error reading file".to_string());
        let location = path.strip_prefix(repo_path)
            .unwrap_or(path)
            .display()
            .to_string();
        Ok(FileEntry { location, content })
    }).collect()
}

pub fn get_filtered_files(
    repo_path: &PathBuf,
    include_pattern: Option<String>,
    exclude_pattern: Option<String>,
) -> Result<Vec<PathBuf>> {

    let include_pattern = include_pattern.as_deref().unwrap_or("/**/*");
    let included_file_paths = glob(&format!("{}/{}", repo_path.display(), include_pattern))
        .context("Failed to read include glob pattern")?
        .filter_map(Result::ok)
        .collect::<HashSet<_>>();

    let excluded_file_paths = if let Some(pattern) = exclude_pattern {
        glob(&format!("{}/{}", repo_path.display(), pattern))
            .context("Failed to read exclude glob pattern")?
            .filter_map(Result::ok)
            .collect()
    } else {
        HashSet::new()
    };

    Ok(included_file_paths.difference(&excluded_file_paths).cloned().collect())
}
