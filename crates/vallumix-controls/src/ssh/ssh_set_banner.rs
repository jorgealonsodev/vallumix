use std::fs;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct SshSetBanner {
    sshd_config_path: PathBuf,
}

impl Default for SshSetBanner {
    fn default() -> Self {
        SshSetBanner { sshd_config_path: PathBuf::from("/etc/ssh/sshd_config") }
    }
}

impl SshSetBanner {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(sshd_config_path: PathBuf) -> Self {
        SshSetBanner { sshd_config_path }
    }
}

impl Control for SshSetBanner {
    fn id(&self) -> &str { "5.2.9" }
    fn description(&self) -> &str { "Ensure SSH warning banner is configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Ssh }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("Banner") && !t.starts_with('#') {
                let v = t.strip_prefix("Banner").unwrap().trim().trim_start_matches('=').trim();
                if !v.is_empty() && v != "none" {
                    return Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("Banner set to {}", v), message: None });
                }
            }
        }
        Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "Banner not configured".into(), message: Some("Set Banner /etc/issue.net".into()) })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would set Banner".into()) });
        }
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        let mut found = false;
        let mut lines: Vec<String> = Vec::new();
        for line in content.lines() {
            if line.trim().starts_with("Banner") && !line.trim().starts_with('#') {
                lines.push("Banner /etc/issue.net".to_string());
                found = true;
            } else {
                lines.push(line.to_string());
            }
        }
        if !found { lines.push("Banner /etc/issue.net".to_string()); }
        fs::write(&self.sshd_config_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set Banner /etc/issue.net".into()) })
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
    fn compliant_with_banner() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "Banner /etc/issue.net\n").unwrap();
        let ctrl = SshSetBanner::with_path(tmp.path().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::Compliant);
    }

    #[test]
    fn non_compliant_without() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "Port 22\n").unwrap();
        let ctrl = SshSetBanner::with_path(tmp.path().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::NonCompliant);
    }
}
