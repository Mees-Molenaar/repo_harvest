use std::{env, fs, path::PathBuf, process::Command};
use anyhow::{Context, Result};

pub fn does_repo_exist(github_url: &str) -> Result<bool> {
    let output = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg(github_url)
        .output()
        .context("Failed to execute Github CLI view command")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Github CLI failed, {}", error_message);
    }

    Ok(output.status.success())

}

pub fn clone_repo(github_url: &str) -> Result<PathBuf> {
    let temp_dir = env::temp_dir().join("repo");

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).context("Failed to remove existing temporary directory.")?;
    }

    let output = Command::new("gh")
        .arg("repo")
        .arg("clone")
        .arg(github_url)
        .arg(&temp_dir)
        .output()
        .context("Failed to execute Github CLI clone command")?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to clone repository: {}", error_message);
    }

    Ok(temp_dir)
}