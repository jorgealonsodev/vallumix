use std::path::PathBuf;
use std::process::Command;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct ConfigureFirewalld {
    systemctl_path: PathBuf,
}

impl Default for ConfigureFirewalld {
    fn default() -> Self {
        ConfigureFirewalld { systemctl_path: PathBuf::from("/usr/bin/systemctl") }
    }
}

impl ConfigureFirewalld {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(systemctl_path: PathBuf) -> Self { ConfigureFirewalld { systemctl_path } }
}

impl Control for ConfigureFirewalld {
    fn id(&self) -> &str { "3.3.1" }
    fn description(&self) -> &str { "Ensure a firewall is installed and enabled" }
    fn severity(&self) -> Severity { Severity::High }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Network }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let out = Command::new(&self.systemctl_path)
            .args(["is-active", "firewalld"])
            .output()
            .map_err(ControlError::Io)?;
        let active = String::from_utf8_lossy(&out.stdout).trim() == "active";
        if active {
            Ok(CheckResult { status: CheckStatus::Compliant, evidence: "firewalld is active".into(), message: None })
        } else {
            let out2 = Command::new(&self.systemctl_path)
                .args(["is-active", "nftables"])
                .output()
                .map_err(ControlError::Io)?;
            let nft_active = String::from_utf8_lossy(&out2.stdout).trim() == "active";
            if nft_active {
                Ok(CheckResult { status: CheckStatus::Compliant, evidence: "nftables is active".into(), message: None })
            } else {
                Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "no firewall daemon active".into(), message: Some("install and enable firewalld or nftables".into()) })
            }
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would enable firewall".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("firewall installation not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> Context {
        Context::with_paths("t".into(), Distro::Debian12, "/tmp".into(), "/tmp".into(), "/tmp".into(), false)
    }

    #[test]
    fn control_has_correct_id() {
        let ctrl = ConfigureFirewalld::new();
        assert_eq!(ctrl.id(), "3.3.1");
    }
}
