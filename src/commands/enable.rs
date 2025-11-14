use crate::config::{self, Profile};
use crate::error::AppError;
use crate::file_utils::{
    self, update_gemini_ignore, SHELF_END_MARKER, SHELF_START_MARKER,
};
use std::fs;

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
