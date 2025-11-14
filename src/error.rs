use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration file (.shelf.yaml) not found.")]
    ConfigNotFound,
    #[error("Profile '{0}' not found in .shelf.yaml.")]
    ProfileNotFound(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Failed to parse .shelf.yaml: {0}")]
    YamlParse(#[from] serde_yaml::Error),
}
