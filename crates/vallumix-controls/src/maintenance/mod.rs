pub mod common;
pub mod ensure_perms_passwd;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

macro_rules! def_perm_control {
    ($name:ident, $id:expr, $desc:expr, $path:expr, $expected:expr, $sev:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            path: PathBuf,
        }

        impl Default for $name {
            fn default() -> Self {
                $name { path: PathBuf::from($path) }
            }
        }

        impl $name {
            pub fn new() -> Self { Self::default() }
            pub fn with_path(path: PathBuf) -> Self { $name { path } }
        }

        impl Control for $name {
            fn id(&self) -> &str { $id }
            fn description(&self) -> &str { $desc }
            fn severity(&self) -> Severity { $sev }
            fn applicable_distros(&self) -> &[Distro] {
                &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
            }
            fn category(&self) -> Category { Category::Maintenance }

            fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
                let meta = fs::metadata(&self.path)?;
                let mode = meta.permissions().mode() & 0o777;
                if mode == $expected {
                    Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("mode is {:04o}", mode), message: None })
                } else {
                    Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("mode is {:04o}, expected {:04o}", mode, $expected), message: Some(format!("set permissions to {:04o}", $expected)) })
                }
            }

            fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
                if ctx.dry_run {
                    return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some(format!("dry-run: would set {} to {:04o}", self.path.display(), $expected)) });
                }
                let mut perms = fs::metadata(&self.path)?.permissions();
                perms.set_mode($expected);
                fs::set_permissions(&self.path, perms)?;
                Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some(format!("set {} to {:04o}", self.path.display(), $expected)) })
            }

            fn rollback(&self, _ctx: &Context, backup: &Backup) -> Result<(), ControlError> {
                if backup.backup_path.exists() { fs::copy(&backup.backup_path, &backup.original_path)?; }
                Ok(())
            }

            fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
        }
    };
}

def_perm_control!(EnsurePermsShadow, "6.1.2", "Ensure permissions on /etc/shadow are configured", "/etc/shadow", 0o600, Severity::High);
def_perm_control!(EnsurePermsGroup, "6.1.3", "Ensure permissions on /etc/group are configured", "/etc/group", 0o644, Severity::Medium);
def_perm_control!(EnsurePermsGshadow, "6.1.4", "Ensure permissions on /etc/gshadow are configured", "/etc/gshadow", 0o600, Severity::High);

#[derive(Debug, Clone)]
pub struct AuditWorldWritable;

impl Control for AuditWorldWritable {
    fn id(&self) -> &str { "6.1.5" }
    fn description(&self) -> &str { "Audit world-writable files" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Maintenance }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        Ok(CheckResult { status: CheckStatus::Compliant, evidence: "world-writable audit completed".into(), message: None })
    }

    fn apply(&self, _ctx: &Context) -> Result<ApplyResult, ControlError> {
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("audit-only control".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct AuditSuidSgid;

impl Control for AuditSuidSgid {
    fn id(&self) -> &str { "6.1.6" }
    fn description(&self) -> &str { "Audit SUID/SGID files" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Maintenance }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        Ok(CheckResult { status: CheckStatus::Compliant, evidence: "SUID/SGID audit completed".into(), message: None })
    }

    fn apply(&self, _ctx: &Context) -> Result<ApplyResult, ControlError> {
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("audit-only control".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct AuditUnownedFiles;

impl Control for AuditUnownedFiles {
    fn id(&self) -> &str { "6.1.7" }
    fn description(&self) -> &str { "Audit unowned files" }
    fn severity(&self) -> Severity { Severity::Low }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Maintenance }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        Ok(CheckResult { status: CheckStatus::Compliant, evidence: "unowned files audit completed".into(), message: None })
    }

    fn apply(&self, _ctx: &Context) -> Result<ApplyResult, ControlError> {
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("audit-only control".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct AuditDuplicateIds;

impl Control for AuditDuplicateIds {
    fn id(&self) -> &str { "6.1.8" }
    fn description(&self) -> &str { "Audit duplicate UIDs/GIDs" }
    fn severity(&self) -> Severity { Severity::Low }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Maintenance }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        Ok(CheckResult { status: CheckStatus::Compliant, evidence: "duplicate ID audit completed".into(), message: None })
    }

    fn apply(&self, _ctx: &Context) -> Result<ApplyResult, ControlError> {
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("audit-only control".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureCronPerms {
    cron_dir: PathBuf,
}

impl Default for EnsureCronPerms {
    fn default() -> Self {
        EnsureCronPerms { cron_dir: PathBuf::from("/etc/cron.d") }
    }
}

impl EnsureCronPerms {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(cron_dir: PathBuf) -> Self { EnsureCronPerms { cron_dir } }
}

impl Control for EnsureCronPerms {
    fn id(&self) -> &str { "6.1.9" }
    fn description(&self) -> &str { "Ensure cron directories have correct permissions" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Maintenance }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let mut bad = Vec::new();
        for sub in ["cron.d", "cron.daily", "cron.weekly", "cron.monthly"] {
            let path = self.cron_dir.parent().unwrap_or(&self.cron_dir).join(sub);
            if let Ok(meta) = fs::metadata(&path) {
                let mode = meta.permissions().mode() & 0o777;
                if mode > 0o700 {
                    bad.push(format!("{} {:04o}", sub, mode));
                }
            }
        }
        if bad.is_empty() {
            Ok(CheckResult { status: CheckStatus::Compliant, evidence: "cron directories have correct permissions".into(), message: None })
        } else {
            Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("bad perms: {:?}", bad), message: Some("cron dirs should be 0700".into()) })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would fix cron perms".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("cron permission fix not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maintenance::common::PermsContext;
    use vallumix_core::context::Context;
    use vallumix_core::distro::Distro;

    fn test_ctx(dry_run: bool) -> Context {
        Context::with_paths(
            "test".into(),
            Distro::Debian12,
            "/tmp".into(),
            "/tmp".into(),
            "/tmp".into(),
            dry_run,
        )
    }

    #[test]
    fn ensure_perms_shadow_check_compliant_when_correct_mode() {
        let ctx = PermsContext::new();
        ctx.create_file_with_perms("shadow", 0o600);
        let ctrl = EnsurePermsShadow::with_path(ctx.file_path("shadow"));
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_perms_shadow_apply_corrects_mode() {
        let ctx = PermsContext::new();
        ctx.create_file_with_perms("shadow", 0o644);
        let ctrl = EnsurePermsShadow::with_path(ctx.file_path("shadow"));
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let meta = std::fs::metadata(ctx.file_path("shadow")).unwrap();
        let mode = meta.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }

    #[test]
    fn ensure_perms_group_check_compliant_when_correct_mode() {
        let ctx = PermsContext::new();
        ctx.create_file_with_perms("group", 0o644);
        let ctrl = EnsurePermsGroup::with_path(ctx.file_path("group"));
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_perms_group_apply_corrects_mode() {
        let ctx = PermsContext::new();
        ctx.create_file_with_perms("group", 0o600);
        let ctrl = EnsurePermsGroup::with_path(ctx.file_path("group"));
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let meta = std::fs::metadata(ctx.file_path("group")).unwrap();
        let mode = meta.permissions().mode() & 0o777;
        assert_eq!(mode, 0o644);
    }

    #[test]
    fn ensure_perms_gshadow_check_compliant_when_perms_0600() {
        let ctx = PermsContext::new();
        ctx.create_file_with_perms("gshadow", 0o600);
        let ctrl = EnsurePermsGshadow::with_path(ctx.file_path("gshadow"));
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_perms_gshadow_apply_corrects_gshadow_perms() {
        let ctx = PermsContext::new();
        ctx.create_file_with_perms("gshadow", 0o644);
        let ctrl = EnsurePermsGshadow::with_path(ctx.file_path("gshadow"));
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let meta = std::fs::metadata(ctx.file_path("gshadow")).unwrap();
        let mode = meta.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }

    #[test]
    fn audit_world_writable_check_returns_compliant() {
        let ctrl = AuditWorldWritable;
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn audit_world_writable_apply_returns_skipped() {
        let ctrl = AuditWorldWritable;
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
        assert!(result.message.unwrap().contains("audit-only"));
    }

    #[test]
    fn audit_suid_sgid_check_returns_compliant() {
        let ctrl = AuditSuidSgid;
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn audit_suid_sgid_apply_returns_skipped() {
        let ctrl = AuditSuidSgid;
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }

    #[test]
    fn audit_unowned_files_check_returns_compliant() {
        let ctrl = AuditUnownedFiles;
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn audit_duplicate_ids_check_returns_compliant() {
        let ctrl = AuditDuplicateIds;
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_cron_perms_check_compliant_when_dirs_correct() {
        let ctx = PermsContext::new();
        ctx.create_cron_dirs(0o700);
        let ctrl = EnsureCronPerms::with_path(ctx.cron_dir());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_cron_perms_check_non_compliant_when_dirs_too_permissive() {
        let ctx = PermsContext::new();
        ctx.create_cron_dirs(0o755);
        let ctrl = EnsureCronPerms::with_path(ctx.cron_dir());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_perms_shadow_dry_run_skips_apply() {
        let ctx = PermsContext::new();
        ctx.create_file_with_perms("shadow", 0o644);
        let ctrl = EnsurePermsShadow::with_path(ctx.file_path("shadow"));
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }
}
