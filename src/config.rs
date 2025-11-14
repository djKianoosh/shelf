use crate::error::AppError;
use crate::file_utils;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;

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
    let config_path =
        file_utils::find_file_upwards(".shelf.yaml")?.ok_or(AppError::ConfigNotFound)?;
    let file_content = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&file_content)?;
    Ok(config)
}
