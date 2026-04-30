use std::path::PathBuf;
use std::process::Command;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct ServiceDisable {
    id: &'static str,
    description: &'static str,
    service_name: String,
    systemctl_path: PathBuf,
    severity: Severity,
    search_paths: Vec<PathBuf>,
}

impl ServiceDisable {
    pub fn new(
        id: &'static str,
        description: &'static str,
        service_name: &str,
        severity: Severity,
    ) -> Self {
        let svc = service_name.to_string();
        ServiceDisable {
            id,
            description,
            service_name: svc.clone(),
            systemctl_path: PathBuf::from("/usr/bin/systemctl"),
            severity,
            search_paths: vec![
                PathBuf::from(format!("/etc/systemd/system/{}.service", svc)),
                PathBuf::from(format!("/lib/systemd/system/{}.service", svc)),
                PathBuf::from(format!("/usr/lib/systemd/system/{}.service", svc)),
            ],
        }
    }

    pub fn with_paths(
        id: &'static str,
        description: &'static str,
        service_name: String,
        systemctl_path: PathBuf,
        severity: Severity,
    ) -> Self {
        ServiceDisable {
            id,
            description,
            service_name: service_name.clone(),
            systemctl_path,
            severity,
            search_paths: vec![
                PathBuf::from(format!("/etc/systemd/system/{}.service", service_name)),
                PathBuf::from(format!("/lib/systemd/system/{}.service", service_name)),
                PathBuf::from(format!("/usr/lib/systemd/system/{}.service", service_name)),
            ],
        }
    }

    pub fn with_all_paths(
        id: &'static str,
        description: &'static str,
        service_name: String,
        systemctl_path: PathBuf,
        severity: Severity,
        search_paths: Vec<PathBuf>,
    ) -> Self {
        ServiceDisable {
            id,
            description,
            service_name,
            systemctl_path,
            severity,
            search_paths,
        }
    }

    fn service_exists(&self) -> bool {
        self.search_paths.iter().any(|p| p.exists())
    }

    fn is_active(&self) -> Result<bool, ControlError> {
        let out = Command::new(&self.systemctl_path)
            .args(["is-active", &self.service_name])
            .output()
            .map_err(ControlError::Io)?;
        Ok(String::from_utf8_lossy(&out.stdout).trim() == "active")
    }

    fn is_enabled(&self) -> Result<bool, ControlError> {
        let out = Command::new(&self.systemctl_path)
            .args(["is-enabled", &self.service_name])
            .output()
            .map_err(ControlError::Io)?;
        Ok(String::from_utf8_lossy(&out.stdout).trim() == "enabled")
    }
}

impl Control for ServiceDisable {
    fn id(&self) -> &str { self.id }
    fn description(&self) -> &str { self.description }
    fn severity(&self) -> Severity { self.severity }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Services }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        if !self.service_exists() {
            return Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("{} not installed", self.service_name), message: None });
        }
        let active = self.is_active().unwrap_or(false);
        let enabled = self.is_enabled().unwrap_or(false);
        if !active && !enabled {
            Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("{} is stopped and disabled", self.service_name), message: None })
        } else {
            let mut reasons = Vec::new();
            if active { reasons.push("active"); }
            if enabled { reasons.push("enabled"); }
            Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("{} is {}", self.service_name, reasons.join(" and ")), message: Some(format!("{} should be stopped and disabled", self.service_name)) })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some(format!("dry-run: would stop and disable {}", self.service_name)) });
        }
        if !self.service_exists() {
            return Ok(ApplyResult { status: ApplyStatus::AlreadyCompliant, backup_path: None, message: Some(format!("{} not installed", self.service_name)) });
        }
        let stop = Command::new(&self.systemctl_path).args(["stop", &self.service_name]).output().map_err(ControlError::Io)?;
        let disable = Command::new(&self.systemctl_path).args(["disable", &self.service_name]).output().map_err(ControlError::Io)?;
        if stop.status.success() && disable.status.success() {
            Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some(format!("{} stopped and disabled", self.service_name)) })
        } else {
            let mut errs = Vec::new();
            if !stop.status.success() { errs.push(format!("stop failed: {}", String::from_utf8_lossy(&stop.stderr))); }
            if !disable.status.success() { errs.push(format!("disable failed: {}", String::from_utf8_lossy(&disable.stderr))); }
            Ok(ApplyResult { status: ApplyStatus::Failed, backup_path: None, message: Some(errs.join("; ")) })
        }
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        if self.service_exists() {
            Command::new(&self.systemctl_path).args(["enable", &self.service_name]).output().map_err(ControlError::Io)?;
            Command::new(&self.systemctl_path).args(["start", &self.service_name]).output().map_err(ControlError::Io)?;
        }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
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

    fn fake_systemctl_script(tmpdir: &std::path::Path, active: &str, enabled: &str) -> PathBuf {
        let path = tmpdir.join("systemctl");
        let script = format!(
            "#!/bin/bash\ncase \"$1\" in\n  is-active) echo '{}'; exit 0 ;;\n  is-enabled) echo '{}'; exit 0 ;;\n  stop) exit 0 ;;\n  disable) exit 0 ;;\n  enable) exit 0 ;;\n  start) exit 0 ;;\nesac\n",
            active, enabled
        );
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(script.as_bytes()).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[test]
    fn new_sets_defaults() {
        let ctrl = ServiceDisable::new("T.1", "desc", "svc", Severity::Low);
        assert_eq!(ctrl.id(), "T.1");
        assert_eq!(ctrl.description(), "desc");
        assert_eq!(ctrl.service_name, "svc");
        assert_eq!(ctrl.systemctl_path, PathBuf::from("/usr/bin/systemctl"));
        assert_eq!(ctrl.severity(), Severity::Low);
    }

    #[test]
    fn check_compliant_when_service_not_installed() {
        let ctrl = ServiceDisable::new("T.1", "test", "nonexistent-svc-12345", Severity::Low);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("not installed"));
    }

    #[test]
    fn check_compliant_when_service_inactive_and_disabled() {
        let tmpdir = tempfile::tempdir().unwrap();
        let svc_file = tmpdir.path().join("test-svc.service");
        std::fs::write(&svc_file, "[Service]\n").unwrap();
        let systemctl = fake_systemctl_script(tmpdir.path(), "inactive", "disabled");

        let ctrl = ServiceDisable::with_all_paths(
            "T.1",
            "test",
            "test-svc".into(),
            systemctl,
            Severity::Low,
            vec![svc_file],
        );
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("stopped and disabled"));
    }

    #[test]
    fn check_non_compliant_when_service_active() {
        let tmpdir = tempfile::tempdir().unwrap();
        let svc_file = tmpdir.path().join("test-svc.service");
        std::fs::write(&svc_file, "[Service]\n").unwrap();
        let systemctl = fake_systemctl_script(tmpdir.path(), "active", "disabled");

        let ctrl = ServiceDisable::with_all_paths(
            "T.1",
            "test",
            "test-svc".into(),
            systemctl,
            Severity::Low,
            vec![svc_file],
        );
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("active"));
    }

    #[test]
    fn check_non_compliant_when_service_enabled() {
        let tmpdir = tempfile::tempdir().unwrap();
        let svc_file = tmpdir.path().join("test-svc.service");
        std::fs::write(&svc_file, "[Service]\n").unwrap();
        let systemctl = fake_systemctl_script(tmpdir.path(), "inactive", "enabled");

        let ctrl = ServiceDisable::with_all_paths(
            "T.1",
            "test",
            "test-svc".into(),
            systemctl,
            Severity::Low,
            vec![svc_file],
        );
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("enabled"));
    }

    #[test]
    fn apply_returns_already_compliant_when_not_installed() {
        let ctrl = ServiceDisable::new("T.1", "test", "nonexistent-svc-12345", Severity::Low);
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::AlreadyCompliant);
    }

    #[test]
    fn apply_skips_when_dry_run() {
        let ctrl = ServiceDisable::new("T.1", "test", "svc", Severity::Low);
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
        assert!(result.message.unwrap().contains("dry-run"));
    }

    #[test]
    fn apply_stops_and_disables_service() {
        let tmpdir = tempfile::tempdir().unwrap();
        let svc_file = tmpdir.path().join("test-svc.service");
        std::fs::write(&svc_file, "[Service]\n").unwrap();
        let systemctl = fake_systemctl_script(tmpdir.path(), "active", "enabled");

        let ctrl = ServiceDisable::with_all_paths(
            "T.1",
            "test",
            "test-svc".into(),
            systemctl,
            Severity::Low,
            vec![svc_file],
        );
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
    }

    #[test]
    fn rollback_succeeds_when_service_not_installed() {
        let ctrl = ServiceDisable::new("T.1", "test", "nonexistent-svc-12345", Severity::Low);
        let backup = vallumix_core::profile::Backup {
            id: "b".into(),
            timestamp: chrono::Utc::now(),
            original_path: PathBuf::from("/tmp/orig"),
            backup_path: PathBuf::from("/tmp/backup"),
        };
        let result = ctrl.rollback(&test_ctx(false), &backup);
        assert!(result.is_ok());
    }

    #[test]
    fn clone_box_produces_identical_control() {
        let ctrl = ServiceDisable::new("T.1", "test", "svc", Severity::Low);
        let cloned = ctrl.clone_box();
        assert_eq!(cloned.id(), "T.1");
        assert_eq!(cloned.description(), "test");
        assert_eq!(cloned.severity(), Severity::Low);
    }
}
