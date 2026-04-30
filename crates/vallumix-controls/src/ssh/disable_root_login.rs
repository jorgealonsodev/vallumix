use std::fs;
use std::io::Write;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct SshDisableRootLogin {
    sshd_config_path: PathBuf,
}

impl Default for SshDisableRootLogin {
    fn default() -> Self {
        SshDisableRootLogin {
            sshd_config_path: PathBuf::from("/etc/ssh/sshd_config"),
        }
    }
}

impl SshDisableRootLogin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(sshd_config_path: PathBuf) -> Self {
        SshDisableRootLogin { sshd_config_path }
    }
}

fn parse_permit_root_login(content: &str) -> Option<bool> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("PermitRootLogin") {
            let value = rest.trim().trim_start_matches('=').trim();
            return Some(value.eq_ignore_ascii_case("no"));
        }
    }
    None
}

impl Control for SshDisableRootLogin {
    fn id(&self) -> &str {
        "5.2.4"
    }

    fn description(&self) -> &str {
        "Ensure SSH root login is disabled"
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn applicable_distros(&self) -> &[Distro] {
        &[
            Distro::Debian12,
            Distro::Ubuntu2204,
            Distro::Ubuntu2404,
            Distro::Rocky9,
        ]
    }

    fn category(&self) -> Category {
        Category::Ssh
    }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        match parse_permit_root_login(&content) {
            Some(true) => Ok(CheckResult {
                status: CheckStatus::Compliant,
                evidence: "PermitRootLogin is set to a secure value".into(),
                message: None,
            }),
            Some(false) => Ok(CheckResult {
                status: CheckStatus::NonCompliant,
                evidence: "PermitRootLogin allows root login".into(),
                message: Some("PermitRootLogin should be set to no".into()),
            }),
            None => Ok(CheckResult {
                status: CheckStatus::NonCompliant,
                evidence: "PermitRootLogin directive not found".into(),
                message: Some("PermitRootLogin should be explicitly set to no".into()),
            }),
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult {
                status: ApplyStatus::Skipped,
                backup_path: None,
                message: Some("dry-run: would set PermitRootLogin no".into()),
            });
        }
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        let mut found = false;
        let mut new_lines: Vec<String> = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("PermitRootLogin") && !trimmed.starts_with('#') {
                new_lines.push("PermitRootLogin no".to_string());
                found = true;
            } else {
                new_lines.push(line.to_string());
            }
        }
        if !found {
            new_lines.push("PermitRootLogin no".to_string());
        }
        fs::write(&self.sshd_config_path, new_lines.join("\n"))?;
        Ok(ApplyResult {
            status: ApplyStatus::Applied,
            backup_path: None,
            message: Some("set PermitRootLogin no".into()),
        })
    }

    fn rollback(&self, _ctx: &Context, backup: &Backup) -> Result<(), ControlError> {
        if backup.backup_path.exists() {
            fs::copy(&backup.backup_path, &backup.original_path)?;
        }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn check_compliant_when_permit_root_login_no() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "PermitRootLogin no\n").unwrap();
        let ctrl = SshDisableRootLogin::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn check_non_compliant_when_permit_root_login_yes() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "PermitRootLogin yes\n").unwrap();
        let ctrl = SshDisableRootLogin::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn check_non_compliant_when_missing() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "Port 22\n").unwrap();
        let ctrl = SshDisableRootLogin::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn apply_sets_permit_root_login_no() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "Port 22\nPermitRootLogin yes\n").unwrap();
        let ctrl = SshDisableRootLogin::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmp.path()).unwrap();
        assert!(content.contains("PermitRootLogin no"));
        assert!(!content.contains("PermitRootLogin yes"));
    }

    #[test]
    fn apply_adds_directive_when_missing() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "Port 22\n").unwrap();
        let ctrl = SshDisableRootLogin::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmp.path()).unwrap();
        assert!(content.contains("PermitRootLogin no"));
    }

    #[test]
    fn apply_adds_directive_when_only_commented_exists() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "Port 22\n#PermitRootLogin yes\n").unwrap();
        let ctrl = SshDisableRootLogin::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmp.path()).unwrap();
        assert!(content.contains("PermitRootLogin no"));
    }

    #[test]
    fn fixture_sshd_root_no_is_compliant() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = manifest.join("tests/fixtures/sshd_root_no");
        let ctrl = SshDisableRootLogin::with_path(fixture);
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn fixture_sshd_root_yes_is_non_compliant() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = manifest.join("tests/fixtures/sshd_root_yes");
        let ctrl = SshDisableRootLogin::with_path(fixture);
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }
}
