use anyhow::Result;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

const SHELF_YAML_CONTENT: &str = r#"
global:
  excludes:
    - "global-exclude"

frontend:
  description: "Frontend profile"
  includes:
    - "frontend-include"
  excludes:
    - "frontend-exclude"
"#;

#[test]
fn test_enable_profile() -> Result<()> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".shelf.yaml"), SHELF_YAML_CONTENT)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("enable").arg("frontend");

    cmd.assert().success().stdout(predicate::str::contains(
        "✔ Activated profile 'frontend'. .geminiignore updated.",
    ));

    let gemini_ignore_content = fs::read_to_string(dir.path().join(".geminiignore"))?;
    let expected_content = [
        "# --- SHELF START ---",
        "# Profile: frontend",
        "**/*",
        "!frontend-include",
        "frontend-exclude",
        "global-exclude",
        "# --- SHELF END ---",
    ]
    .join("\n");

    assert!(gemini_ignore_content.contains(&expected_content));

    Ok(())
}
