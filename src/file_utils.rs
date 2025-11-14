use crate::error::AppError;
use std::env;
use std::path::PathBuf;

pub const SHELF_START_MARKER: &str = "# --- SHELF START ---";
pub const SHELF_END_MARKER: &str = "# --- SHELF END ---";

pub fn find_file_upwards(filename: &str) -> Result<Option<PathBuf>, AppError> {
    let current_dir = env::current_dir()?;
    for ancestor in current_dir.ancestors() {
        let file_path = ancestor.join(filename);
        if file_path.exists() {
            return Ok(Some(file_path));
        }
    }
    Ok(None)
}

pub fn update_gemini_ignore(original_content: &str, shelf_block: &str) -> String {
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
