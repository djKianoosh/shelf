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
        "*",
        "!/frontend-include",
        "frontend-exclude",
        "global-exclude",
        "# --- SHELF END ---",
    ]
    .join("\n");

    assert!(gemini_ignore_content.contains(&expected_content));

    Ok(())
}

#[cfg(unix)]
#[test]
fn test_enable_profile_symlink_rejected() -> Result<()> {
    use std::os::unix::fs::symlink;

    let dir = tempdir()?;
    fs::write(dir.path().join(".shelf.yaml"), SHELF_YAML_CONTENT)?;

    let target_file = dir.path().join("sensitive_file.txt");
    let initial_content = "DO_NOT_OVERWRITE";
    fs::write(&target_file, initial_content)?;

    let gemini_ignore_symlink = dir.path().join(".geminiignore");
    symlink(&target_file, &gemini_ignore_symlink)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("enable").arg("frontend");

    cmd.assert().failure().stderr(predicate::str::contains(
        "is a symlink, which is not allowed",
    ));

    let content = fs::read_to_string(&target_file)?;
    assert_eq!(content, initial_content);

    Ok(())
}

#[test]
fn test_enable_profile_with_directory_include() -> Result<()> {
    let dir = tempdir()?;
    let shelf_yaml_content = r#"
frontend:
  description: "Frontend profile"
  includes:
    - "src/"
    - "README.md"
  excludes:
    - "src/tests/"
"#;
    fs::write(dir.path().join(".shelf.yaml"), shelf_yaml_content)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("enable").arg("frontend");

    cmd.assert().success();

    let gemini_ignore_content = fs::read_to_string(dir.path().join(".geminiignore"))?;

    assert!(gemini_ignore_content.contains("!/src"));
    assert!(gemini_ignore_content.contains("!/src/**"));
    assert!(gemini_ignore_content.contains("!/README.md"));
    assert!(gemini_ignore_content.contains("src/tests/"));

    Ok(())
}

#[test]
fn test_enable_profile_with_global_includes() -> Result<()> {
    let dir = tempdir()?;
    let shelf_yaml_content = r#"
global:
  includes:
    - "docs/"
    - "config.toml"
  excludes:
    - "global-exclude"

frontend:
  description: "Frontend profile"
  includes:
    - "src/"
  excludes:
    - "frontend-exclude"
"#;
    fs::write(dir.path().join(".shelf.yaml"), shelf_yaml_content)?;

    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir(dir.path());
    cmd.arg("enable").arg("frontend");

    cmd.assert().success();

    let gemini_ignore_content = fs::read_to_string(dir.path().join(".geminiignore"))?;

    // Check for profile includes
    assert!(gemini_ignore_content.contains("!/src"));
    assert!(gemini_ignore_content.contains("!/src/**"));

    // Check for global includes
    assert!(gemini_ignore_content.contains("!/docs"));
    assert!(gemini_ignore_content.contains("!/docs/**"));
    assert!(gemini_ignore_content.contains("!/config.toml"));

    // Check for excludes
    assert!(gemini_ignore_content.contains("frontend-exclude"));
    assert!(gemini_ignore_content.contains("global-exclude"));

    Ok(())
}
