use clap::{Parser, Subcommand};

mod commands;
mod config;
mod error;

use crate::error::AppError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists all available profiles
    List,
    /// Shows the currently active profile
    Status,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("✖ Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            commands::list::list_profiles()?;
        }
        Commands::Status => {
            commands::status::run_status()?;
        }
    }

    Ok(())
}
