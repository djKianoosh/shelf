use crate::error::AppError;
use std::env;
use std::path::PathBuf;

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
