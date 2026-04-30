use std::fs;
use std::io::Write;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct DisableCramfs {
    filesystems_path: PathBuf,
    modprobe_dir: PathBuf,
}

impl Default for DisableCramfs {
    fn default() -> Self {
        DisableCramfs {
            filesystems_path: PathBuf::from("/proc/filesystems"),
            modprobe_dir: PathBuf::from("/etc/modprobe.d"),
        }
    }
}

impl DisableCramfs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_paths(filesystems_path: PathBuf, modprobe_dir: PathBuf) -> Self {
        DisableCramfs {
            filesystems_path,
            modprobe_dir,
        }
    }
}

impl Control for DisableCramfs {
    fn id(&self) -> &str {
        "1.1.1.1"
    }

    fn description(&self) -> &str {
        "Ensure mounting of cramfs filesystems is disabled"
    }

    fn severity(&self) -> Severity {
        Severity::Low
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
        Category::Filesystem
    }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.filesystems_path)?;
        let present = content.lines().any(|line| line.contains("cramfs"));
        if present {
            Ok(CheckResult {
                status: CheckStatus::NonCompliant,
                evidence: "cramfs found in /proc/filesystems".into(),
                message: Some("cramfs filesystem is available".into()),
            })
        } else {
            Ok(CheckResult {
                status: CheckStatus::Compliant,
                evidence: "cramfs not found in /proc/filesystems".into(),
                message: None,
            })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult {
                status: ApplyStatus::Skipped,
                backup_path: None,
                message: Some("dry-run: would disable cramfs".into()),
            });
        }
        let modprobe_file = self.modprobe_dir.join("vallumix-disable-cramfs.conf");
        fs::create_dir_all(&self.modprobe_dir)?;
        let mut file = fs::File::create(&modprobe_file)?;
        file.write_all(b"install cramfs /bin/true\n")?;
        Ok(ApplyResult {
            status: ApplyStatus::Applied,
            backup_path: None,
            message: Some(format!("wrote {}", modprobe_file.display())),
        })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        let modprobe_file = self.modprobe_dir.join("vallumix-disable-cramfs.conf");
        if modprobe_file.exists() {
            fs::remove_file(&modprobe_file)?;
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
    fn check_compliant_when_cramfs_absent() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "nodev\tsquashfs\n").unwrap();
        let ctrl = DisableCramfs::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        let ctx = Context::with_paths(
            "test".into(),
            Distro::Debian12,
            "/tmp".into(),
            "/tmp".into(),
            "/tmp".into(),
            false,
        );
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn check_non_compliant_when_cramfs_present() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "nodev\tcramfs\n").unwrap();
        let ctrl = DisableCramfs::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        let ctx = Context::with_paths(
            "test".into(),
            Distro::Debian12,
            "/tmp".into(),
            "/tmp".into(),
            "/tmp".into(),
            false,
        );
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn apply_writes_modprobe_file() {
        let tmpdir = tempfile::tempdir().unwrap();
        let ctrl = DisableCramfs::with_paths(
            tmpdir.path().join("fs"),
            tmpdir.path().join("modprobe.d"),
        );
        let ctx = Context::with_paths(
            "test".into(),
            Distro::Debian12,
            "/tmp".into(),
            "/tmp".into(),
            "/tmp".into(),
            false,
        );
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmpdir.path().join("modprobe.d/vallumix-disable-cramfs.conf")).unwrap();
        assert!(content.contains("install cramfs /bin/true"));
    }

    #[test]
    fn apply_skips_in_dry_run() {
        let tmpdir = tempfile::tempdir().unwrap();
        let ctrl = DisableCramfs::with_paths(
            tmpdir.path().join("fs"),
            tmpdir.path().join("modprobe.d"),
        );
        let ctx = Context::with_paths(
            "test".into(),
            Distro::Debian12,
            "/tmp".into(),
            "/tmp".into(),
            "/tmp".into(),
            true,
        );
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }

    #[test]
    fn fixture_no_cramfs_is_compliant() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = manifest.join("tests/fixtures/filesystems_no_cramfs");
        let ctrl = DisableCramfs::with_paths(fixture, manifest.join("tests/fixtures"));
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn fixture_with_cramfs_is_non_compliant() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = manifest.join("tests/fixtures/filesystems_with_cramfs");
        let ctrl = DisableCramfs::with_paths(fixture, manifest.join("tests/fixtures"));
        let ctx = Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false);
        let result = ctrl.check(&ctx).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }
}
