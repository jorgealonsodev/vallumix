use std::fs;
use std::io::Write;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct DisableHfs {
    filesystems_path: PathBuf,
    modprobe_dir: PathBuf,
}

impl Default for DisableHfs {
    fn default() -> Self {
        DisableHfs {
            filesystems_path: PathBuf::from("/proc/filesystems"),
            modprobe_dir: PathBuf::from("/etc/modprobe.d"),
        }
    }
}

impl DisableHfs {
    pub fn new() -> Self { Self::default() }
    pub fn with_paths(filesystems_path: PathBuf, modprobe_dir: PathBuf) -> Self {
        DisableHfs { filesystems_path, modprobe_dir }
    }
}

impl Control for DisableHfs {
    fn id(&self) -> &str { "1.1.1.4" }
    fn description(&self) -> &str { "Ensure mounting of hfs filesystems is disabled" }
    fn severity(&self) -> Severity { Severity::Low }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Filesystem }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.filesystems_path)?;
        let present = content.lines().any(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            words.iter().any(|&w| w == "hfs")
        });
        Ok(if present {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "hfs found in /proc/filesystems".into(), message: Some("hfs is available".into()) }
        } else {
            CheckResult { status: CheckStatus::Compliant, evidence: "hfs not found in /proc/filesystems".into(), message: None }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would disable hfs".into()) });
        }
        let file = self.modprobe_dir.join("vallumix-disable-hfs.conf");
        fs::create_dir_all(&self.modprobe_dir)?;
        let mut f = fs::File::create(&file)?;
        f.write_all(b"install hfs /bin/true\n")?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some(format!("wrote {}", file.display())) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        let file = self.modprobe_dir.join("vallumix-disable-hfs.conf");
        if file.exists() { fs::remove_file(&file)?; }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn ctx() -> Context {
        Context::with_paths("test".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false)
    }

    #[test]
    fn check_compliant_when_absent() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "nodev\tsquashfs\n").unwrap();
        let ctrl = DisableHfs::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::Compliant);
    }

    #[test]
    fn check_non_compliant_when_present() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "nodev\thfs\n").unwrap();
        let ctrl = DisableHfs::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::NonCompliant);
    }

    #[test]
    fn apply_writes_modprobe_file() {
        let tmpdir = tempfile::tempdir().unwrap();
        let ctrl = DisableHfs::with_paths(tmpdir.path().join("fs"), tmpdir.path().join("modprobe.d"));
        let result = ctrl.apply(&ctx()).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmpdir.path().join("modprobe.d/vallumix-disable-hfs.conf")).unwrap();
        assert!(content.contains("install hfs /bin/true"));
    }
}
