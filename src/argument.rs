use clap::Parser;

/// Easily to backup your SQLite database
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Argument {
    /// The project name
    #[arg(long, default_value = "default")]
    pub project_name: String,

    /// The path to the database to backup
    #[arg(long)]
    pub db: String,
}
