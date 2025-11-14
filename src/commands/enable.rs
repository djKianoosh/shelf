use crate::config::{self, Profile};
use crate::error::AppError;
use crate::file_utils;
use std::fs;

const SHELF_START_MARKER: &str = "# --- SHELF START ---";
const SHELF_END_MARKER: &str = "# --- SHELF END ---";

pub fn enable_profile(profile_name: &str) -> Result<(), AppError> {
    let config = config::find_and_parse()?;
    let profile = config
        .profiles
        .get(profile_name)
        .ok_or_else(|| AppError::ProfileNotFound(profile_name.to_string()))?;

    let gemini_ignore_path = file_utils::find_file_upwards(".geminiignore")?
        .unwrap_or_else(|| ".geminiignore".into());

    let original_content = if gemini_ignore_path.exists() {
        fs::read_to_string(&gemini_ignore_path)?
    } else {
        String::new()
    };

    let global_excludes = config.profiles.get("global").map(|p| &p.excludes);
    let shelf_block = generate_shelf_block(profile_name, profile, global_excludes);
    let new_content = update_gemini_ignore(&original_content, &shelf_block);

    fs::write(&gemini_ignore_path, new_content)?;

    println!(
        "✔ Activated profile '{}'. .geminiignore updated.",
        profile_name
    );

    Ok(())
}

fn generate_shelf_block(
    profile_name: &str,
    profile: &Profile,
    global_excludes: Option<&Vec<String>>,
) -> String {
    let mut block = vec![
        SHELF_START_MARKER.to_string(),
        format!("# Profile: {}", profile_name),
        "**/*".to_string(),
    ];

    for include in &profile.includes {
        block.push(format!("!{}", include));
    }

    block.extend(profile.excludes.iter().cloned());

    if let Some(excludes) = global_excludes {
        block.extend(excludes.iter().cloned());
    }

    block.push(SHELF_END_MARKER.to_string());

    block.join("\n")
}

fn update_gemini_ignore(original_content: &str, shelf_block: &str) -> String {
    let mut new_content = String::new();
    let mut in_shelf_block = false;
    let mut shelf_block_written = false;

    for line in original_content.lines() {
        if line.trim() == SHELF_START_MARKER {
            in_shelf_block = true;
            if !shelf_block_written {
                new_content.push_str(shelf_block);
                new_content.push('\n');
                shelf_block_written = true;
            }
        } else if line.trim() == SHELF_END_MARKER {
            in_shelf_block = false;
        } else if !in_shelf_block {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    if !shelf_block_written {
        new_content.push_str(shelf_block);
        new_content.push('\n');
    }

    new_content
}
