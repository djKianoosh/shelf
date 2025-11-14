use crate::error::AppError;
use crate::file_utils::{self, SHELF_END_MARKER, SHELF_START_MARKER, update_gemini_ignore};
use std::fs;

pub fn disable_profile() -> Result<(), AppError> {
    let gemini_ignore_path = match file_utils::find_file_upwards(".geminiignore")? {
        Some(path) => path,
        None => {
            // If the file doesn't exist, there's nothing to disable.
            return Ok(());
        }
    };

    let original_content = fs::read_to_string(&gemini_ignore_path)?;
    let empty_shelf_block = format!("{}\n{}", SHELF_START_MARKER, SHELF_END_MARKER);
    let new_content = update_gemini_ignore(&original_content, &empty_shelf_block);

    fs::write(&gemini_ignore_path, new_content)?;

    println!("✔ All shelf profiles disabled. .geminiignore updated.");

    Ok(())
}
