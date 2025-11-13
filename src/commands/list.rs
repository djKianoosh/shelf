use crate::config;
use crate::error::AppError;

pub fn list_profiles() -> Result<(), AppError> {
    let config = config::find_and_parse()?;

    println!("Available profiles:");
    for (name, profile) in config.profiles {
        // We filter out the 'global' key as it's not a selectable profile
        if name == "global" {
            continue;
        }
        let description = profile.description.unwrap_or_default();
        // This padding is a simple way to align the descriptions
        println!("• {:<14}: {}", name, description);
    }

    Ok(())
}
