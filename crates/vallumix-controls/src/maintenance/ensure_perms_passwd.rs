use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct EnsurePermsPasswd {
    passwd_path: PathBuf,
}

impl Default for EnsurePermsPasswd {
    fn default() -> Self {
        EnsurePermsPasswd {
            passwd_path: PathBuf::from("/etc/passwd"),
        }
    }
}

impl EnsurePermsPasswd {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(passwd_path: PathBuf) -> Self {
        EnsurePermsPasswd { passwd_path }
    }
}

impl Control for EnsurePermsPasswd {
    fn id(&self) -> &str {
        "6.1.1"
    }

    fn description(&self) -> &str {
        "Ensure permissions on /etc/passwd are configured"
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
        Category::Maintenance
    }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let metadata = fs::metadata(&self.passwd_path)?;
        let mode = metadata.permissions().mode() & 0o777;
        if mode == 0o644 {
            Ok(CheckResult {
                status: CheckStatus::Compliant,
                evidence: format!("/etc/passwd mode is {:04o}", mode),
                message: None,
            })
        } else {
            Ok(CheckResult {
                status: CheckStatus::NonCompliant,
                evidence: format!("/etc/passwd mode is {:04o}, expected 0644", mode),
                message: Some("permissions should be 0644".into()),
            })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult {
                status: ApplyStatus::Skipped,
                backup_path: None,
                message: Some("dry-run: would set /etc/passwd permissions to 0644".into()),
            });
        }
        let mut perms = fs::metadata(&self.passwd_path)?.permissions();
        perms.set_mode(0o644);
        fs::set_permissions(&self.passwd_path, perms)?;
        Ok(ApplyResult {
            status: ApplyStatus::Applied,
            backup_path: None,
            message: Some("set /etc/passwd permissions to 0644".into()),
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
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn check_compliant_when_mode_0644() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let mut perms = fs::metadata(tmp.path()).unwrap().permissions();
        perms.set_mode(0o644);
        fs::set_permissions(tmp.path(), perms).unwrap();
        let ctrl = EnsurePermsPasswd::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn check_non_compliant_when_mode_0777() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let mut perms = fs::metadata(tmp.path()).unwrap().permissions();
        perms.set_mode(0o777);
        fs::set_permissions(tmp.path(), perms).unwrap();
        let ctrl = EnsurePermsPasswd::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn apply_sets_mode_0644() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let mut perms = fs::metadata(tmp.path()).unwrap().permissions();
        perms.set_mode(0o777);
        fs::set_permissions(tmp.path(), perms).unwrap();
        let ctrl = EnsurePermsPasswd::with_path(tmp.path().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let metadata = fs::metadata(tmp.path()).unwrap();
        assert_eq!(metadata.permissions().mode() & 0o777, 0o644);
    }
}
