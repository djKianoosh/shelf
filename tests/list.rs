use anyhow::Result;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_list_profiles() -> Result<()> {
    let mut cmd = cargo_bin_cmd!("shelf");
    cmd.current_dir("tests/fixtures");
    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"Available profiles:").unwrap())
        .stdout(
            predicate::str::is_match(r"•\s*backend\s*:\s*Focus on all backend services.").unwrap(),
        )
        .stdout(
            predicate::str::is_match(
                r"•\s*feature-slice\s*:\s*Full-stack context for a feature \(orders service and main app\).",
            )
            .unwrap(),
        )
        .stdout(
            predicate::str::is_match(
                r"•\s*frontend\s*:\s*Scope to the main web application and its shared UI components.",
            )
            .unwrap(),
        )
        .stdout(
            predicate::str::is_match(r"•\s*tech-docs\s*:\s*Context for writing documentation.")
                .unwrap(),
        );

    Ok(())
}

