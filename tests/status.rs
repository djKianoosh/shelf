use anyhow::Result;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

const SHELF_START_MARKER: &str = "# --- SHELF START ---";
const SHELF_END_MARKER: &str = "# --- SHELF END ---";

#[test]
fn test_status_active_profile_and_user_patterns() -> Result<()> {
    let dir = tempdir()?;
    let gemini_ignore_path = dir.path().join(".geminiignore");

    let content = [
        "node_modules/",
        ".env",
        "",
        SHELF_START_MARKER,
        "# Profile: frontend",
        "**/*",
        "!web/main-app/",
        "!packages/ui-components/",
        SHELF_END_MARKER,
        "",
        "# User comment",
        "target/",
    ]
    .join("\n");
    fs::write(gemini_ignore_path, content)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("status");

    cmd.assert().success().stdout(
        predicate::str::contains("Profile 'frontend' is active.")
            .and(predicate::str::contains("User-defined patterns:"))
            .and(predicate::str::contains("• node_modules/"))
            .and(predicate::str::contains("• .env"))
            .and(predicate::str::contains("• target/")),
    );

    Ok(())
}

#[test]
fn test_status_only_user_patterns() -> Result<()> {
    let dir = tempdir()?;
    let gemini_ignore_path = dir.path().join(".geminiignore");

    let content = [
        "node_modules/",
        ".env",
        "",
        SHELF_START_MARKER,
        SHELF_END_MARKER,
        "",
    ]
    .join("\n");
    fs::write(gemini_ignore_path, content)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("status");

    cmd.assert().success().stdout(
        predicate::str::contains("No shelf profile is active.")
            .and(predicate::str::contains("User-defined patterns:"))
            .and(predicate::str::contains("• node_modules/"))
            .and(predicate::str::contains("• .env")),
    );

    Ok(())
}

#[test]
fn test_status_only_active_profile() -> Result<()> {
    let dir = tempdir()?;
    let gemini_ignore_path = dir.path().join(".geminiignore");

    let content = [
        SHELF_START_MARKER,
        "# Profile: backend",
        "**/*",
        "!services/",
        SHELF_END_MARKER,
    ]
    .join("\n");
    fs::write(gemini_ignore_path, content)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("status");

    cmd.assert().success().stdout(
        predicate::str::contains("Profile 'backend' is active.")
            .and(predicate::str::contains("User-defined patterns:").not()),
    );

    Ok(())
}

#[test]
fn test_status_empty_gemini_ignore() -> Result<()> {
    let dir = tempdir()?;
    fs::File::create(dir.path().join(".geminiignore"))?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("status");

    cmd.assert().success().stdout(
        predicate::str::contains("No shelf profile is active.")
            .and(predicate::str::contains("User-defined patterns:").not()),
    );

    Ok(())
}

#[test]
fn test_status_no_gemini_ignore_file() -> Result<()> {
    let dir = tempdir()?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("status");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No .geminiignore file found."));

    Ok(())
}
