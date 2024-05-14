use std::path::PathBuf;

mod cli;
mod github;
mod file_processing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();

    if !github::does_repo_exist(&args.github_url)? {
        println!("The repository does not exist");
        return Ok(());
    }

    let repo_path = github::clone_repo(&args.github_url)?;

    let filtered_files = file_processing::get_filtered_files(
        &repo_path, 
        args.include, 
        args.exclude)?;

    let output_file = PathBuf::from(&args.output_file);

    match &args.format {
        cli::OutputFormat::Json => {
            file_processing::create_json_output(filtered_files, &repo_path, output_file)?;
        }
        cli::OutputFormat::Markdown => {
            file_processing::create_markdown_output(filtered_files, &repo_path, output_file)?;
        }
    };

    Ok(())
    
}
