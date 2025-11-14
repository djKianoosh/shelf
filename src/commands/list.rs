use crate::config;
use crate::error::AppError;
use std::collections::BTreeMap;

pub fn list_profiles() -> Result<(), AppError> {
    let config = config::find_and_parse()?;

    let profiles: BTreeMap<String, config::Profile> = config
        .profiles
        .into_iter()
        .filter(|(name, _)| name != "global")
        .collect();

    let max_name_len = profiles.keys().map(|name| name.len()).max().unwrap_or(0);

    println!("Available profiles:");
    for (name, profile) in profiles {
        let description = profile.description.unwrap_or_default();
        println!("• {:<width$}: {}", name, description, width = max_name_len);
    }

    Ok(())
}
