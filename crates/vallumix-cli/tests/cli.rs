use std::path::PathBuf;
use std::process::Command;

use assert_cmd::prelude::*;
use nix::unistd;
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
    if unistd::geteuid().is_root() {
        eprintln!("skipping: running as root (CI container)");
        return;
    }
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

#[test]
fn cli_audit_html_report() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--report")
        .arg("html")
        .arg("--threshold")
        .arg("0")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<!DOCTYPE html>"))
        .stdout(predicate::str::contains("<html"));
}

#[test]
fn cli_audit_junit_report() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--report")
        .arg("junit")
        .arg("--threshold")
        .arg("0")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<?xml"))
        .stdout(predicate::str::contains("<testsuite"));
}

#[test]
fn cli_audit_text_report() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--report")
        .arg("text")
        .arg("--threshold")
        .arg("0")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Compliant"))
        .stdout(predicate::str::contains("Non-Compliant"));
}

#[test]
fn cli_audit_multi_report() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--report")
        .arg("html,json")
        .arg("--threshold")
        .arg("0")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<!DOCTYPE html>"))
        .stdout(predicate::str::contains("\"host\""));
}

#[test]
fn cli_audit_output_file() {
    let tmpdir = tempfile::tempdir().unwrap();
    let output_path = tmpdir.path().join("vallumix-report.html");
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("audit")
        .arg("--profile")
        .arg("web")
        .arg("--report")
        .arg("html")
        .arg("--output")
        .arg(&output_path)
        .arg("--threshold")
        .arg("0")
        .env("VALLUMIX_PROFILE_DIR", profile_dir());
    cmd.assert().success();
    assert!(output_path.exists(), "output file should exist");
    let content = std::fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("<!DOCTYPE html>"));
}

#[test]
fn cli_rollback_no_session() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("rollback")
        .arg("--control-id")
        .arg("9.9.9.9");
    // No backup exists for this control → exit code 2
    cmd.assert().failure().code(2);
}

#[test]
fn cli_completion_zsh() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("completion").arg("zsh");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("vallumix"));
}

#[test]
fn cli_completion_fish() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("completion").arg("fish");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("vallumix"));
}

#[test]
fn cli_completion_nushell() {
    let mut cmd = Command::cargo_bin("vallumix").unwrap();
    cmd.arg("completion").arg("nushell");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'nushell'"));
}
