use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author, about, version, long_about = None)]
pub struct Cli {
    /// The Github URL of the repository you want to fetch.
    #[clap(help = "Specify the Github repository URL.")]
    pub github_url: String,

    /// Glob pattern to exclude files or directories
    #[clap(short = 'e', long = "exclude", help = "Exclude files matching the specified glob pattern.")]
    pub exclude: Option<String>,

    /// Include pattern
    #[clap(short = 'p', long = "include", help = "Include files matching the specified glob pattern.")]
    pub include: Option<String>,

    /// Specifies the output format of the result
    #[clap(
        short = 'f',
        long = "format",
        value_enum,
        rename_all = "kebab-case",
        default_value_t = OutputFormat::Markdown,
        help = "Choose an output format: json or markdown."
    )]
    pub format: OutputFormat,

    /// Include hidden files in the output
    #[clap(short = 'i', long = "include-hidden", default_value = "false", help = "Include hidden files in the output.")]
    pub hidden: bool,

    /// Specifies the output file name.
    #[clap(short = 'o', long = "output", default_value = "output", help = "Set the output file path.")]
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