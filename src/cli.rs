use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Csv,
    Json,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input_file: String,
    #[arg(short, long)]
    pub loinc_column: String,
    #[arg(short, long)]
    pub unit_column: String,
    #[arg(short, long)]
    pub output_file: Option<String>,
    #[arg(short, long, default_value_t = false)]
    pub strict: bool,
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Csv)]
    pub format: OutputFormat,
    #[arg(long, default_value_t = false)]
    pub no_progress: bool,
}
