use std::{env, fs, path::PathBuf, process::Command};

pub fn does_repo_exist(github_url: &str) -> bool {
    let command = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg(github_url)
        .output()
        .expect("Failed to execute command");

    command.status.success()
}

pub fn clone_repo(github_url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
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