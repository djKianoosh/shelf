use anyhow::Result;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

const SHELF_START_MARKER: &str = "# --- SHELF START ---";
const SHELF_END_MARKER: &str = "# --- SHELF END ---";

#[test]
fn test_disable_profile() -> Result<()> {
    let dir = tempdir()?;
    let gemini_ignore_path = dir.path().join(".geminiignore");

    let content = [
        "user-rule",
        SHELF_START_MARKER,
        "# Profile: frontend",
        "**/*",
        "!frontend-include",
        "frontend-exclude",
        "global-exclude",
        SHELF_END_MARKER,
        "another-user-rule",
    ]
    .join("\n");
    fs::write(&gemini_ignore_path, content)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("disable");

    cmd.assert().success().stdout(predicate::str::contains(
        "✔ All shelf profiles disabled. .geminiignore updated.",
    ));

    let gemini_ignore_content = fs::read_to_string(gemini_ignore_path)?;
    let expected_content = [
        "user-rule",
        SHELF_START_MARKER,
        SHELF_END_MARKER,
        "another-user-rule",
    ]
    .join("\n");

    assert_eq!(gemini_ignore_content.trim(), expected_content.trim());

    Ok(())
}
