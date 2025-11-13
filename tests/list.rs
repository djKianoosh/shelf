use anyhow::Result;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_list_profiles() -> Result<()> {
    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir("tests/fixtures");
    cmd.arg("list");

    cmd.assert().success().stdout(
        predicate::str::contains("Available profiles:")
            .and(predicate::str::contains(
                "• backend       : Focus on all backend services.",
            ))
            .and(predicate::str::contains(
                "• feature-slice : Full-stack context for a feature (orders service and main app).",
            ))
            .and(predicate::str::contains(
                "• frontend      : Scope to the main web application and its shared UI components.",
            ))
            .and(predicate::str::contains(
                "• tech-docs     : Context for writing documentation.",
            )),
    );

    Ok(())
}

