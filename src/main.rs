use clap::{Parser, Subcommand};

mod commands;
mod config;
mod error;
mod file_utils;

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
    /// Activates a profile
    Enable {
        /// The name of the profile to activate
        profile_name: String,
    },
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
        Commands::Enable { profile_name } => {
            commands::enable::enable_profile(&profile_name)?;
        }
    }

    Ok(())
}
