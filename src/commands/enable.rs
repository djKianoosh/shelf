use crate::config::{self, Profile};
use crate::error::AppError;
use crate::file_utils::{self, SHELF_END_MARKER, SHELF_START_MARKER, update_gemini_ignore};
use std::fs;

pub fn enable_profile(profile_name: &str) -> Result<(), AppError> {
    let config = config::find_and_parse()?;
    let profile = config
        .profiles
        .get(profile_name)
        .ok_or_else(|| AppError::ProfileNotFound(profile_name.to_string()))?;

    let gemini_ignore_path =
        file_utils::find_file_upwards(".geminiignore")?.unwrap_or_else(|| ".geminiignore".into());

    let original_content = if gemini_ignore_path.exists() {
        fs::read_to_string(&gemini_ignore_path)?
    } else {
        String::new()
    };

    let global_profile = config.profiles.get("global");
    let global_includes = global_profile.map(|p| &p.includes);
    let global_excludes = global_profile.map(|p| &p.excludes);
    let shelf_block = generate_shelf_block(profile_name, profile, global_includes, global_excludes);
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
    global_includes: Option<&Vec<String>>,
    global_excludes: Option<&Vec<String>>,
) -> String {
    let mut block = vec![
        SHELF_START_MARKER.to_string(),
        format!("# Profile: {}", profile_name),
        "*".to_string(),
    ];

    // Helper closure to avoid repetition
    let mut add_includes = |includes: &Vec<String>| {
        for include in includes {
            let is_dir = include.ends_with('/');
            let base_path = include.trim_end_matches('/');
            let clean_path = base_path.trim_start_matches('/');

            if is_dir {
                block.push(format!("!/{}", clean_path));
                block.push(format!("!/{}/**", clean_path));
            } else {
                block.push(format!("!/{}", clean_path));
            }
        }
    };

    // Process profile includes
    add_includes(&profile.includes);

    // Process global includes
    if let Some(includes) = global_includes {
        add_includes(includes);
    }

    block.extend(profile.excludes.iter().cloned());

    if let Some(excludes) = global_excludes {
        block.extend(excludes.iter().cloned());
    }

    block.push(SHELF_END_MARKER.to_string());

    block.join("\n")
}
