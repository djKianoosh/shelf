use crate::error::AppError;
use crate::file_utils;
use std::fs;

const SHELF_START_MARKER: &str = "# --- SHELF START ---";
const SHELF_END_MARKER: &str = "# --- SHELF END ---";

pub fn run_status() -> Result<(), AppError> {
    let gemini_ignore_path = match file_utils::find_file_upwards(".geminiignore")? {
        Some(path) => path,
        None => {
            println!("No .geminiignore file found.");
            return Ok(());
        }
    };

    let content = fs::read_to_string(gemini_ignore_path)?;
    let lines = content.lines();

    let mut active_profile: Option<String> = None;
    let mut user_patterns: Vec<String> = Vec::new();
    let mut in_shelf_block = false;

    for line in lines {
        let trimmed_line = line.trim();

        if trimmed_line == SHELF_START_MARKER {
            in_shelf_block = true;
            continue;
        }

        if trimmed_line == SHELF_END_MARKER {
            in_shelf_block = false;
            continue;
        }

        if in_shelf_block {
            if let Some(profile_name) = trimmed_line.strip_prefix("# Profile: ") {
                active_profile = Some(profile_name.to_string());
            }
        } else if !trimmed_line.is_empty() && !trimmed_line.starts_with('#') {
            user_patterns.push(trimmed_line.to_string());
        }
    }

    if let Some(profile) = active_profile {
        println!("Profile '{}' is active.", profile);
    } else {
        println!("No shelf profile is active.");
    }

    if !user_patterns.is_empty() {
        println!("\nUser-defined patterns:");
        for pattern in user_patterns {
            println!("• {}", pattern);
        }
    }

    Ok(())
}


