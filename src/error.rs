use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(".shelf.yaml not found in this directory or any parent directories.")]
    ConfigNotFound,

    #[error("Failed to read file: {0}")]
    FileReadError(#[from] io::Error),

    #[error("Failed to parse .shelf.yaml on line {line}: {message}")]
    YamlParseError { line: usize, message: String },

    // This is a catch-all for other errors that we don't handle specifically yet.
    #[error("An unexpected error occurred: {0}")]
    Anyhow(#[from] anyhow::Error),
}

// Custom conversion from serde_yaml::Error to our AppError
impl From<serde_yaml::Error> for AppError {
    fn from(e: serde_yaml::Error) -> Self {
        if let Some(location) = e.location() {
            AppError::YamlParseError {
                line: location.line(),
                message: e.to_string(),
            }
        } else {
            // If there's no location info, we create a generic message.
            AppError::YamlParseError {
                line: 0, // 0 or 1? Let's use 0 for "unknown".
                message: e.to_string(),
            }
        }
    }
}
