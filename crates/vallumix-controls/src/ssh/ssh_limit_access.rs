use std::fs;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct SshLimitAccess {
    sshd_config_path: PathBuf,
}

impl Default for SshLimitAccess {
    fn default() -> Self {
        SshLimitAccess { sshd_config_path: PathBuf::from("/etc/ssh/sshd_config") }
    }
}

impl SshLimitAccess {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(sshd_config_path: PathBuf) -> Self {
        SshLimitAccess { sshd_config_path }
    }
}

impl Control for SshLimitAccess {
    fn id(&self) -> &str { "5.2.8" }
    fn description(&self) -> &str { "Ensure SSH access is limited" }
    fn severity(&self) -> Severity { Severity::High }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Ssh }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        let has_allowusers = content.lines().any(|l| l.trim().starts_with("AllowUsers"));
        let has_allowgroups = content.lines().any(|l| l.trim().starts_with("AllowGroups"));
        if has_allowusers || has_allowgroups {
            Ok(CheckResult { status: CheckStatus::Compliant, evidence: "AllowUsers or AllowGroups configured".into(), message: None })
        } else {
            Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "Neither AllowUsers nor AllowGroups configured".into(), message: Some("Configure AllowUsers or AllowGroups".into()) })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure AllowUsers".into()) });
        }
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        lines.push("AllowUsers admin".to_string());
        fs::write(&self.sshd_config_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("added AllowUsers".into()) })
    }

    fn rollback(&self, _ctx: &Context, backup: &Backup) -> Result<(), ControlError> {
        if backup.backup_path.exists() { fs::copy(&backup.backup_path, &backup.original_path)?; }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn ctx() -> Context {
        Context::with_paths("t".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false)
    }

    #[test]
    fn compliant_with_allowusers() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "AllowUsers admin\n").unwrap();
        let ctrl = SshLimitAccess::with_path(tmp.path().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::Compliant);
    }

    #[test]
    fn non_compliant_without() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "Port 22\n").unwrap();
        let ctrl = SshLimitAccess::with_path(tmp.path().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::NonCompliant);
    }
}
