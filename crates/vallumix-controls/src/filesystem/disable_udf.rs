use std::fs;
use std::io::Write;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct DisableUdf {
    filesystems_path: PathBuf,
    modprobe_dir: PathBuf,
}

impl Default for DisableUdf {
    fn default() -> Self {
        DisableUdf {
            filesystems_path: PathBuf::from("/proc/filesystems"),
            modprobe_dir: PathBuf::from("/etc/modprobe.d"),
        }
    }
}

impl DisableUdf {
    pub fn new() -> Self { Self::default() }
    pub fn with_paths(filesystems_path: PathBuf, modprobe_dir: PathBuf) -> Self {
        DisableUdf { filesystems_path, modprobe_dir }
    }
}

impl Control for DisableUdf {
    fn id(&self) -> &str { "1.1.1.7" }
    fn description(&self) -> &str { "Ensure mounting of udf filesystems is disabled" }
    fn severity(&self) -> Severity { Severity::Low }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Filesystem }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.filesystems_path)?;
        let present = content.lines().any(|line| line.contains("udf"));
        Ok(if present {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "udf found in /proc/filesystems".into(), message: Some("udf is available".into()) }
        } else {
            CheckResult { status: CheckStatus::Compliant, evidence: "udf not found in /proc/filesystems".into(), message: None }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would disable udf".into()) });
        }
        let file = self.modprobe_dir.join("vallumix-disable-udf.conf");
        fs::create_dir_all(&self.modprobe_dir)?;
        let mut f = fs::File::create(&file)?;
        f.write_all(b"install udf /bin/true\n")?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some(format!("wrote {}", file.display())) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        let file = self.modprobe_dir.join("vallumix-disable-udf.conf");
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
        write!(tmp, "nodev\text4\n").unwrap();
        let ctrl = DisableUdf::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::Compliant);
    }

    #[test]
    fn check_non_compliant_when_present() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "nodev\tudf\n").unwrap();
        let ctrl = DisableUdf::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::NonCompliant);
    }

    #[test]
    fn apply_writes_modprobe_file() {
        let tmpdir = tempfile::tempdir().unwrap();
        let ctrl = DisableUdf::with_paths(tmpdir.path().join("fs"), tmpdir.path().join("modprobe.d"));
        let result = ctrl.apply(&ctx()).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmpdir.path().join("modprobe.d/vallumix-disable-udf.conf")).unwrap();
        assert!(content.contains("install udf /bin/true"));
    }
}
