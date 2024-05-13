use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    /// The Github URL of the repository you want to fetch
    pub github_url: String,
    /// Exclude pattern
    #[clap(short = 'e', long)]
    pub exclude: Option<String>,
    /// Include pattern
    #[clap(short = 'p', long)]
    pub include: Option<String>,
    /// Output format
    /// Can be one of, json, markdown
    #[clap(
        short = 'f',
        long,
        value_enum,
        rename_all = "kebab-case",
        default_value = "markdown"
    )]
    pub format: OutputFormat,
    /// Include hidden files
    /// By default, hidden files are not included
    /// If you want to include hidden files, set this flag to true
    #[clap(short = 'i', long, default_value = "false")]
    pub hidden: Option<bool>,
    /// The output file
    #[clap(short, long, default_value = "output")]
    pub output_file: String,
    // Output style
    // Can be one of, folder, one-file
    // #[clap(short='s', long, value_enum, rename_all="kebab-case", default_value_t = OutputStyle::OneFile)]
    // output_style: OutputStyle,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputStyle {
    Folder,
    OneFile,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Json,
    Markdown,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}