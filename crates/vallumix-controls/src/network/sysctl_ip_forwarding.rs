use std::fs;
use std::io::Write;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct SysctlIpForwarding {
    proc_path: PathBuf,
    sysctl_dir: PathBuf,
}

impl Default for SysctlIpForwarding {
    fn default() -> Self {
        SysctlIpForwarding {
            proc_path: PathBuf::from("/proc/sys/net/ipv4/ip_forward"),
            sysctl_dir: PathBuf::from("/etc/sysctl.d"),
        }
    }
}

impl SysctlIpForwarding {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_paths(proc_path: PathBuf, sysctl_dir: PathBuf) -> Self {
        SysctlIpForwarding {
            proc_path,
            sysctl_dir,
        }
    }
}

impl Control for SysctlIpForwarding {
    fn id(&self) -> &str {
        "3.1.1"
    }

    fn description(&self) -> &str {
        "Ensure IP forwarding is disabled"
    }

    fn severity(&self) -> Severity {
        Severity::Medium
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
        Category::Network
    }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.proc_path).unwrap_or_else(|_| "1\n".into());
        let value = content.trim();
        if value == "0" {
            Ok(CheckResult {
                status: CheckStatus::Compliant,
                evidence: "ip_forward = 0".into(),
                message: None,
            })
        } else {
            Ok(CheckResult {
                status: CheckStatus::NonCompliant,
                evidence: format!("ip_forward = {}", value),
                message: Some("IP forwarding is enabled".into()),
            })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult {
                status: ApplyStatus::Skipped,
                backup_path: None,
                message: Some("dry-run: would disable IP forwarding".into()),
            });
        }
        let dropin = self.sysctl_dir.join("99-vallumix-ip-forward.conf");
        fs::create_dir_all(&self.sysctl_dir)?;
        let mut file = fs::File::create(&dropin)?;
        file.write_all(b"net.ipv4.ip_forward = 0\n")?;
        Ok(ApplyResult {
            status: ApplyStatus::Applied,
            backup_path: None,
            message: Some(format!("wrote {}", dropin.display())),
        })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        let dropin = self.sysctl_dir.join("99-vallumix-ip-forward.conf");
        if dropin.exists() {
            fs::remove_file(&dropin)?;
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
    fn check_compliant_when_ip_forward_0() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "0\n").unwrap();
        let ctrl = SysctlIpForwarding::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn check_non_compliant_when_ip_forward_1() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "1\n").unwrap();
        let ctrl = SysctlIpForwarding::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn apply_writes_sysctl_dropin() {
        let tmpdir = tempfile::tempdir().unwrap();
        let ctrl = SysctlIpForwarding::with_paths(tmpdir.path().join("ip_forward"), tmpdir.path().join("sysctl.d"));
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmpdir.path().join("sysctl.d/99-vallumix-ip-forward.conf")).unwrap();
        assert!(content.contains("net.ipv4.ip_forward = 0"));
    }

    #[test]
    fn fixture_ip_forward_0_is_compliant() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = manifest.join("tests/fixtures/ip_forward_0");
        let ctrl = SysctlIpForwarding::with_paths(fixture, manifest.join("tests/fixtures"));
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn fixture_ip_forward_1_is_non_compliant() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = manifest.join("tests/fixtures/ip_forward_1");
        let ctrl = SysctlIpForwarding::with_paths(fixture, manifest.join("tests/fixtures"));
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }
}
