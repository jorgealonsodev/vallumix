use std::fs;
use std::io::Write;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct HardenTmpfs {
    mounts_path: PathBuf,
    systemd_dir: PathBuf,
}

impl Default for HardenTmpfs {
    fn default() -> Self {
        HardenTmpfs {
            mounts_path: PathBuf::from("/proc/mounts"),
            systemd_dir: PathBuf::from("/etc/systemd/system/tmp.mount.d"),
        }
    }
}

impl HardenTmpfs {
    pub fn new() -> Self { Self::default() }
    pub fn with_paths(mounts_path: PathBuf, systemd_dir: PathBuf) -> Self {
        HardenTmpfs { mounts_path, systemd_dir }
    }
}

impl Control for HardenTmpfs {
    fn id(&self) -> &str { "1.1.2.1" }
    fn description(&self) -> &str { "Ensure /tmp is configured with nodev, nosuid, noexec" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Filesystem }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.mounts_path)?;
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[1] == "/tmp" && parts[2] == "tmpfs" {
                let opts = parts[3];
                let has_nodev = opts.split(',').any(|o| o == "nodev");
                let has_nosuid = opts.split(',').any(|o| o == "nosuid");
                let has_noexec = opts.split(',').any(|o| o == "noexec");
                if has_nodev && has_nosuid && has_noexec {
                    return Ok(CheckResult {
                        status: CheckStatus::Compliant,
                        evidence: "/tmp tmpfs has nodev,nosuid,noexec".into(),
                        message: None,
                    });
                } else {
                    let mut missing = Vec::new();
                    if !has_nodev { missing.push("nodev"); }
                    if !has_nosuid { missing.push("nosuid"); }
                    if !has_noexec { missing.push("noexec"); }
                    return Ok(CheckResult {
                        status: CheckStatus::NonCompliant,
                        evidence: format!("/tmp tmpfs missing: {}", missing.join(", ")),
                        message: Some("tmpfs mount options should include nodev,nosuid,noexec".into()),
                    });
                }
            }
        }
        Ok(CheckResult {
            status: CheckStatus::Skipped,
            evidence: "/tmp is not mounted as tmpfs".into(),
            message: Some("skipping: /tmp not tmpfs".into()),
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would harden /tmp tmpfs".into()) });
        }
        let dropin = self.systemd_dir.join("vallumix-tmp-options.conf");
        fs::create_dir_all(&self.systemd_dir)?;
        let mut f = fs::File::create(&dropin)?;
        f.write_all(b"[Mount]\nOptions=mode=1777,strictatime,nodev,nosuid,noexec\n")?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some(format!("wrote {}", dropin.display())) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        let dropin = self.systemd_dir.join("vallumix-tmp-options.conf");
        if dropin.exists() { fs::remove_file(&dropin)?; }
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
    fn check_compliant_with_all_options() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "tmpfs /tmp tmpfs rw,nodev,nosuid,noexec 0 0\n").unwrap();
        let ctrl = HardenTmpfs::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::Compliant);
    }

    #[test]
    fn check_non_compliant_missing_option() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "tmpfs /tmp tmpfs rw,nodev 0 0\n").unwrap();
        let ctrl = HardenTmpfs::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::NonCompliant);
    }

    #[test]
    fn check_skipped_when_not_tmpfs() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "ext4 / ext4 rw 0 0\n").unwrap();
        let ctrl = HardenTmpfs::with_paths(tmp.path().into(), tmp.path().parent().unwrap().into());
        assert_eq!(ctrl.check(&ctx()).unwrap().status, CheckStatus::Skipped);
    }

    #[test]
    fn apply_writes_dropin() {
        let tmpdir = tempfile::tempdir().unwrap();
        let ctrl = HardenTmpfs::with_paths(tmpdir.path().join("mounts"), tmpdir.path().join("tmp.mount.d"));
        let result = ctrl.apply(&ctx()).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = fs::read_to_string(tmpdir.path().join("tmp.mount.d/vallumix-tmp-options.conf")).unwrap();
        assert!(content.contains("nodev"));
        assert!(content.contains("nosuid"));
        assert!(content.contains("noexec"));
    }
}
