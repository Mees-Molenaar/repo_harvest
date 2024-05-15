use anyhow::{Context, Result};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileEntry {
    location: String,
    content: String,
}

pub fn create_markdown_output(
    filtered_files: Vec<PathBuf>,
    repo_path: &PathBuf,
    mut output_file: PathBuf,
) -> Result<()> {
    output_file.set_extension("md");
    let mut file = File::create(&output_file).context("Error creating or opening output file.")?;

    let entries = build_file_entries(&filtered_files, repo_path)?;

    for entry in entries {
        let markdown = format!("## {}\n{}\n", entry.location, entry.content);
        file.write_all(markdown.as_bytes())
            .context("Error writing file contents to output file")?;
    }
    Ok(())
}

pub fn create_json_output(
    filtered_files: Vec<PathBuf>,
    repo_path: &PathBuf,
    mut output_file: PathBuf,
) -> Result<()> {
    output_file.set_extension("json");
    let file = File::create(&output_file).context("Error creating or opening output file.")?;

    let entries: Vec<FileEntry> = build_file_entries(&filtered_files, repo_path)?;
    serde_json::to_writer(&file, &entries).context("Error writing file contents to output file")
}

fn build_file_entries(filtered_files: &[PathBuf], repo_path: &PathBuf) -> Result<Vec<FileEntry>> {
    filtered_files
        .iter()
        .map(|path| {
            println!("{:?}", path);
            let content = fs::read_to_string(path).unwrap();
            let location = path
                .strip_prefix(repo_path)
                .unwrap_or(path)
                .display()
                .to_string();
            Ok(FileEntry { location, content })
        })
        .collect()
}

pub fn get_filtered_files(
    repo_path: &PathBuf,
    include_pattern: Option<String>,
    exclude_pattern: Option<String>,
    include_hidden: bool,
) -> Result<Vec<PathBuf>> {
    let include_pattern = include_pattern.as_deref().unwrap_or("**/*");
    let include_path = format!("{}/{}", repo_path.display(), include_pattern);
    let included_file_paths = glob(&include_path)
        .context("Failed to read include glob pattern")?
        .filter_map(Result::ok)
        .filter(|path| include_hidden || !is_hidden(&path))
        .map(|path| path.strip_prefix(repo_path).unwrap().to_path_buf())
        .collect::<HashSet<_>>();

    let excluded_file_paths = if let Some(pattern) = exclude_pattern {
       let exclude_path = format!("{}/{}", repo_path.display(), pattern);
        glob(&exclude_path)
            .context("Failed to read exclude glob pattern")?
            .filter_map(Result::ok)
            .filter(|path| include_hidden || !is_hidden(&path))
            .map(|path: PathBuf| path.strip_prefix(repo_path).unwrap().to_path_buf())
            .collect::<HashSet<_>>()
    } else {
        HashSet::new()
    };
  
    let filtered_files = included_file_paths
    .difference(&excluded_file_paths)
    .cloned()
    .collect();

    Ok(filtered_files)
}

fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use serde_json::{json, Value};
    use std::io::Read;

    #[allow(dead_code)]
    struct RepoSetup {
        temp_dir: assert_fs::TempDir,
        repo_path: PathBuf,
        output_path: PathBuf,
        filtered_files: Vec<PathBuf>,
    }

    fn setup_repo(extension: &str) -> RepoSetup {
        let temp_dir = assert_fs::TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("repo");
        let mut output_path = temp_dir.path().join("output");
        output_path.set_extension(extension);

        let file1 = temp_dir.child("repo/file1.md");
        file1.write_str("# File 1 Content").unwrap();
        let file2 = temp_dir.child("repo/file2.md");
        file2.write_str("# File 2 Content").unwrap();

        let filtered_files = vec![file1.to_path_buf(), file2.to_path_buf()];

        RepoSetup {
            temp_dir, // If this is not passed, it will be removed
            repo_path,
            output_path,
            filtered_files,
        }
    }

    #[test]
    fn test_create_markdown_output() {
        let repo = setup_repo("md");

        let result = create_markdown_output(
            repo.filtered_files,
            &repo.repo_path,
            repo.output_path.clone(),
        )
        .unwrap();

        let mut output_content = String::new();
        std::fs::File::open(repo.output_path)
            .unwrap()
            .read_to_string(&mut output_content)
            .unwrap();

        assert!(output_content.contains("# File 1 Content"));
        assert!(output_content.contains("# File 2 Content"));
        assert_eq!(result, ());
    }

    #[test]
    fn test_create_json_output() {
        let repo = setup_repo("json");
        let expected_json = json!([
            {
                "location": "file1.md",
                "content": "# File 1 Content"
            },
            {
                "location": "file2.md",
                "content": "# File 2 Content"
            }
        ]);

        let result = create_json_output(
            repo.filtered_files,
            &repo.repo_path,
            repo.output_path.clone(),
        )
        .unwrap();

        let mut output_content = String::new();
        std::fs::File::open(repo.output_path)
            .unwrap()
            .read_to_string(&mut output_content)
            .unwrap();

        let json: Value = serde_json::from_str(&output_content).unwrap();

        assert_eq!(json, expected_json);
        assert_eq!(result, ());

    }

    #[test]
    fn test_include_pattern() -> Result<(), anyhow::Error> {
        let temp = assert_fs::TempDir::new().unwrap();
        let repo_path = temp.path().join("repo");
        temp.child("repo/file1.txt").touch()?;
        temp.child("repo/subdir/file2.txt").touch()?;
        temp.child("repo/subdir/file2.log").touch()?;

        let filtered_files = get_filtered_files(&repo_path, Some("**/*.txt".to_string()), None, false)?;

        assert_eq!(filtered_files.len(), 2);
        assert!(filtered_files.contains(&PathBuf::from("file1.txt")));
        assert!(filtered_files.contains(&PathBuf::from("subdir/file2.txt")));

        Ok(())
    }

    #[test]
    fn test_include_pattern_subdir_txt_files() -> Result<(), anyhow::Error> {
        let temp = assert_fs::TempDir::new().unwrap();
        let repo_path = temp.path().join("repo");

        // Create files in the root directory
        temp.child("repo/file1.txt").touch()?;
        temp.child("repo/file1.log").touch()?;

        // Create files in the subdir directory
        let subdir = temp.child("repo/subdir");
        subdir.child("file2.txt").touch()?;
        subdir.child("file3.log").touch()?;
        subdir.child("file4.txt").touch()?;

        // Include pattern specifically for txt files in the subdir directory
        let included_files = get_filtered_files(&repo_path.into(), Some("subdir/*.txt".to_string()), None, false)?;

        // Validate the result
        assert_eq!(included_files.len(), 2, "Should include exactly two txt files from subdir.");
        assert!(included_files.contains(&PathBuf::from("subdir/file2.txt")), "Should include file2.txt");
        assert!(included_files.contains(&PathBuf::from("subdir/file4.txt")), "Should include file4.txt");

        Ok(())
    }

    #[test]
    fn test_exclude_pattern() -> Result<(), anyhow::Error> {
        let temp = assert_fs::TempDir::new().unwrap();
        let repo_path = temp.path().join("repo");
        temp.child("repo/file1.txt").touch()?;
        temp.child("repo/file2.log").touch()?;
        temp.child("repo/file3.txt").touch()?;

        let filtered_files = get_filtered_files(&repo_path, None, Some("**/*.log".to_string()), false)?;

        assert_eq!(filtered_files.len(), 2);
        assert!(filtered_files.contains(&PathBuf::from("file1.txt")));
        assert!(filtered_files.contains(&PathBuf::from("file3.txt")));

        Ok(())
    }

    #[test]
    fn test_include_and_exclude_pattern() -> Result<(), anyhow::Error> {
        let temp = assert_fs::TempDir::new().unwrap();
        let repo_path: PathBuf = temp.path().join("repo");
        temp.child("repo/file1.txt").touch()?;
        temp.child("repo/subdir/file2.txt").touch()?;
        temp.child("repo/subdir/file2.log").touch()?;

        let filtered_files = get_filtered_files(&repo_path, Some("**/*.txt".to_string()), Some("subdir/**/*".to_string()), false)?;

        assert_eq!(filtered_files.len(), 1);
        assert!(filtered_files.contains(&PathBuf::from("file1.txt")));

        Ok(())
    }

    #[test]
    fn test_exclude_hidden_files() -> Result<(), anyhow::Error> {
        let temp = assert_fs::TempDir::new()?;
        let repo_path: PathBuf = temp.path().join("repo");

        temp.child("repo/visible.txt").touch()?;
        temp.child("repo/.hidden.txt").touch()?;
        temp.child("repo/another_visible.txt").touch()?;

        let filtered_files = get_filtered_files(&repo_path, Some("**/*".to_string()), None, false)?;

        assert_eq!(filtered_files.len(), 2, "Should only include visible files, excluding the hidden file.");
        assert!(filtered_files.contains(&PathBuf::from("visible.txt")), "Should include visible.txt");
        assert!(filtered_files.contains(&PathBuf::from("another_visible.txt")), "Should include another_visible.txt");
        assert!(!filtered_files.contains(&PathBuf::from(".hidden.txt")), "Should not include .hidden.txt");

        Ok(())
    }

    #[test]
    fn test_include_hidden_files() -> Result<(), anyhow::Error> {
        let temp = assert_fs::TempDir::new()?;
        let repo_path: PathBuf = temp.path().join("repo");

        temp.child("repo/visible.txt").touch()?;
        temp.child("repo/.hidden.txt").touch()?;
        temp.child("repo/another_visible.txt").touch()?;

        let filtered_files = get_filtered_files(&repo_path, Some("**/*".to_string()), None, true)?;

        assert_eq!(filtered_files.len(), 3, "Should include all files, including the hidden file.");
        assert!(filtered_files.contains(&PathBuf::from("visible.txt")), "Should include visible.txt");
        assert!(filtered_files.contains(&PathBuf::from(".hidden.txt")), "Should include .hidden.txt");
        assert!(filtered_files.contains(&PathBuf::from("another_visible.txt")), "Should include another_visible.txt");

        Ok(())
    }
}
