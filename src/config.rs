use crate::error::AppError;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: BTreeMap<String, Profile>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Profile {
    pub description: Option<String>,
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(default)]
    pub excludes: Vec<String>,
}

pub fn find_and_parse() -> Result<Config, AppError> {
    let config_path = find_config_file()?;
    let file_content = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&file_content)?;
    Ok(config)
}

fn find_config_file() -> Result<PathBuf, AppError> {
    let current_dir = env::current_dir()?;
    for ancestor in current_dir.ancestors() {
        let config_path = ancestor.join(".shelf.yaml");
        if config_path.exists() {
            return Ok(config_path);
        }
    }
    Err(AppError::ConfigNotFound)
}
