use std::path::PathBuf;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn profile_dir() -> PathBuf {
    workspace_root().join("profiles")
}

#[test]
fn cli_apply_dry_run_requires_root() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("apply")
        .arg("--profile")
        .arg("web")
        .arg("--dry-run")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    // When not root, apply exits with code 3
    cmd.assert().failure().code(3);
}

#[test]
fn cli_audit_json_report() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--report")
        .arg("json")
        .arg("--threshold")
        .arg("0")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("host"))
        .stdout(predicate::str::contains("summary"));
}

#[test]
fn cli_list_outputs_controls() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("1.1.1.1"))
        .stdout(predicate::str::contains("5.2.4"));
}

#[test]
fn cli_completion_bash() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("completion").arg("bash");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("vallumix"));
}

#[test]
fn cli_audit_exit_code_with_threshold() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--threshold")
        .arg("0")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    cmd.assert().success();
}

#[test]
fn cli_audit_exit_code_fails_below_threshold() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--threshold")
        .arg("100")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    // Exit code 1 when compliance is below threshold
    cmd.assert().failure().code(1);
}
