use std::fs;
use std::io::Write;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct SysctlControl {
    id: &'static str,
    description: &'static str,
    severity: Severity,
    params: Vec<(&'static str, &'static str)>, // (proc path suffix, expected value)
    sysctl_dir: PathBuf,
    proc_prefix: PathBuf,
    dropin_name: &'static str,
    dropin_content: &'static str,
}

impl SysctlControl {
    pub fn new(
        id: &'static str,
        description: &'static str,
        severity: Severity,
        params: Vec<(&'static str, &'static str)>,
        dropin_name: &'static str,
        dropin_content: &'static str,
    ) -> Self {
        SysctlControl {
            id,
            description,
            severity,
            params,
            sysctl_dir: PathBuf::from("/etc/sysctl.d"),
            proc_prefix: PathBuf::from("/proc/sys"),
            dropin_name,
            dropin_content,
        }
    }

    pub fn with_paths(
        id: &'static str,
        description: &'static str,
        severity: Severity,
        params: Vec<(&'static str, &'static str)>,
        sysctl_dir: PathBuf,
        proc_prefix: PathBuf,
        dropin_name: &'static str,
        dropin_content: &'static str,
    ) -> Self {
        SysctlControl {
            id,
            description,
            severity,
            params,
            sysctl_dir,
            proc_prefix,
            dropin_name,
            dropin_content,
        }
    }

    fn read_param(&self, suffix: &str) -> Result<String, ControlError> {
        let path = self.proc_prefix.join(suffix.trim_start_matches('/'));
        Ok(fs::read_to_string(&path).unwrap_or_else(|_| "1\n".into()).trim().into())
    }
}

impl Control for SysctlControl {
    fn id(&self) -> &str { self.id }
    fn description(&self) -> &str { self.description }
    fn severity(&self) -> Severity { self.severity }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Network }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let mut non_compliant = Vec::new();
        for (param, expected) in &self.params {
            let actual = self.read_param(param)?;
            if actual != *expected {
                non_compliant.push(format!("{} = {} (expected {})", param, actual, expected));
            }
        }
        if non_compliant.is_empty() {
            Ok(CheckResult { status: CheckStatus::Compliant, evidence: "all sysctl parameters correct".into(), message: None })
        } else {
            Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: non_compliant.join("; "), message: Some("sysctl values need adjustment".into()) })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some(format!("dry-run: would write {}", self.dropin_name)) });
        }
        let dropin = self.sysctl_dir.join(self.dropin_name);
        fs::create_dir_all(&self.sysctl_dir)?;
        let mut f = fs::File::create(&dropin)?;
        f.write_all(self.dropin_content.as_bytes())?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some(format!("wrote {}", dropin.display())) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        let dropin = self.sysctl_dir.join(self.dropin_name);
        if dropin.exists() { fs::remove_file(&dropin)?; }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn make_sysctl_control(proc_prefix: PathBuf, sysctl_dir: PathBuf) -> SysctlControl {
        SysctlControl::with_paths(
            "3.1.2",
            "Ensure ICMP redirects are not accepted",
            Severity::Medium,
            vec![
                ("net/ipv4/conf/all/send_redirects", "0"),
                ("net/ipv4/conf/default/send_redirects", "0"),
            ],
            sysctl_dir,
            proc_prefix,
            "99-vallumix-send-redirects.conf",
            "net.ipv4.conf.all.send_redirects = 0\nnet.ipv4.conf.default.send_redirects = 0\n",
        )
    }

    #[test]
    fn new_sets_defaults() {
        let ctrl = SysctlControl::new(
            "3.1.2",
            "desc",
            Severity::Low,
            vec![("net/ipv4/conf/all/forwarding", "0")],
            "99-test.conf",
            "net.ipv4.conf.all.forwarding = 0\n",
        );
        assert_eq!(ctrl.id(), "3.1.2");
        assert_eq!(ctrl.description(), "desc");
        assert_eq!(ctrl.sysctl_dir, PathBuf::from("/etc/sysctl.d"));
        assert_eq!(ctrl.proc_prefix, PathBuf::from("/proc/sys"));
    }

    #[test]
    fn check_compliant_when_values_match() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/all")).unwrap();
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/default")).unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/all/send_redirects"), "0\n").unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/default/send_redirects"), "0\n").unwrap();

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("all sysctl parameters correct"));
    }

    #[test]
    fn check_non_compliant_when_value_mismatch() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/all")).unwrap();
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/default")).unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/all/send_redirects"), "1\n").unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/default/send_redirects"), "0\n").unwrap();

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("send_redirects = 1"));
    }

    #[test]
    fn check_uses_default_when_file_missing() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        // Don't create proc files — read_param falls back to "1"

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn apply_writes_sysctl_drop_in() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir.clone());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);

        let dropin = sysctl_dir.join("99-vallumix-send-redirects.conf");
        assert!(dropin.exists());
        let content = std::fs::read_to_string(&dropin).unwrap();
        assert!(content.contains("net.ipv4.conf.all.send_redirects = 0"));
    }

    #[test]
    fn apply_skips_when_dry_run() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir.clone());
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
        assert!(!sysctl_dir.join("99-vallumix-send-redirects.conf").exists());
    }

    #[test]
    fn apply_is_idempotent() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir.clone());
        ctrl.apply(&test_ctx(false)).unwrap();
        ctrl.apply(&test_ctx(false)).unwrap();

        let dropin = sysctl_dir.join("99-vallumix-send-redirects.conf");
        assert!(dropin.exists());
    }

    #[test]
    fn rollback_removes_drop_in() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir.clone());
        ctrl.apply(&test_ctx(false)).unwrap();
        let dropin = sysctl_dir.join("99-vallumix-send-redirects.conf");
        assert!(dropin.exists());

        let backup = vallumix_core::profile::Backup {
            id: "b".into(),
            timestamp: chrono::Utc::now(),
            original_path: PathBuf::from("/tmp/orig"),
            backup_path: PathBuf::from("/tmp/backup"),
        };
        ctrl.rollback(&test_ctx(false), &backup).unwrap();
        assert!(!dropin.exists());
    }

    #[test]
    fn rollback_succeeds_when_drop_in_missing() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");

        let ctrl = make_sysctl_control(proc_prefix, sysctl_dir);
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
        let ctrl = SysctlControl::new(
            "3.1.2",
            "desc",
            Severity::Low,
            vec![],
            "test.conf",
            "",
        );
        let cloned = ctrl.clone_box();
        assert_eq!(cloned.id(), "3.1.2");
        assert_eq!(cloned.description(), "desc");
    }
}
