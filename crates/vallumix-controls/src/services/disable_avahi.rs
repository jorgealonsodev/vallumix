use std::fs;
use std::path::PathBuf;
use std::process::Command;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct DisableAvahi {
    service_name: String,
    systemctl_path: PathBuf,
}

impl Default for DisableAvahi {
    fn default() -> Self {
        DisableAvahi {
            service_name: "avahi-daemon".into(),
            systemctl_path: PathBuf::from("/usr/bin/systemctl"),
        }
    }
}

impl DisableAvahi {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_paths(service_name: String, systemctl_path: PathBuf) -> Self {
        DisableAvahi {
            service_name,
            systemctl_path,
        }
    }

    fn is_service_active(&self) -> Result<bool, ControlError> {
        let output = Command::new(&self.systemctl_path)
            .args(["is-active", &self.service_name])
            .output()
            .map_err(ControlError::Io)?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim() == "active")
    }

    fn is_service_enabled(&self) -> Result<bool, ControlError> {
        let output = Command::new(&self.systemctl_path)
            .args(["is-enabled", &self.service_name])
            .output()
            .map_err(ControlError::Io)?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim() == "enabled")
    }

    fn service_exists(&self) -> bool {
        // Check if the service unit exists
        let paths = [
            format!("/etc/systemd/system/{}.service", self.service_name),
            format!("/lib/systemd/system/{}.service", self.service_name),
            format!("/usr/lib/systemd/system/{}.service", self.service_name),
        ];
        paths.iter().any(|p| std::path::Path::new(p).exists())
    }
}

impl Control for DisableAvahi {
    fn id(&self) -> &str {
        "2.2.3"
    }

    fn description(&self) -> &str {
        "Ensure Avahi Server is not enabled"
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
        Category::Services
    }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        if !self.service_exists() {
            return Ok(CheckResult {
                status: CheckStatus::Compliant,
                evidence: format!("{} service not installed", self.service_name),
                message: None,
            });
        }

        let active = self.is_service_active().unwrap_or(false);
        let enabled = self.is_service_enabled().unwrap_or(false);

        if !active && !enabled {
            Ok(CheckResult {
                status: CheckStatus::Compliant,
                evidence: format!("{} is stopped and disabled", self.service_name),
                message: None,
            })
        } else {
            let mut reasons = Vec::new();
            if active {
                reasons.push("active");
            }
            if enabled {
                reasons.push("enabled");
            }
            Ok(CheckResult {
                status: CheckStatus::NonCompliant,
                evidence: format!("{} is {}", self.service_name, reasons.join(" and ")),
                message: Some("avahi-daemon should be stopped and disabled".into()),
            })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult {
                status: ApplyStatus::Skipped,
                backup_path: None,
                message: Some(format!(
                    "dry-run: would stop and disable {}",
                    self.service_name
                )),
            });
        }

        if !self.service_exists() {
            return Ok(ApplyResult {
                status: ApplyStatus::Applied,
                backup_path: None,
                message: Some(format!("{} not installed — no action needed", self.service_name)),
            });
        }

        // Stop the service first
        let stop = Command::new(&self.systemctl_path)
            .args(["stop", &self.service_name])
            .output()
            .map_err(ControlError::Io)?;

        // Disable the service
        let disable = Command::new(&self.systemctl_path)
            .args(["disable", &self.service_name])
            .output()
            .map_err(ControlError::Io)?;

        if stop.status.success() && disable.status.success() {
            Ok(ApplyResult {
                status: ApplyStatus::Applied,
                backup_path: None,
                message: Some(format!(
                    "{} stopped and disabled successfully",
                    self.service_name
                )),
            })
        } else {
            let mut errors = Vec::new();
            if !stop.status.success() {
                errors.push(format!("stop failed: {}", String::from_utf8_lossy(&stop.stderr)));
            }
            if !disable.status.success() {
                errors.push(format!(
                    "disable failed: {}",
                    String::from_utf8_lossy(&disable.stderr)
                ));
            }
            Ok(ApplyResult {
                status: ApplyStatus::Failed,
                backup_path: None,
                message: Some(errors.join("; ")),
            })
        }
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        // Re-enable the service if it exists
        if self.service_exists() {
            Command::new(&self.systemctl_path)
                .args(["enable", &self.service_name])
                .output()
                .map_err(ControlError::Io)?;
            Command::new(&self.systemctl_path)
                .args(["start", &self.service_name])
                .output()
                .map_err(ControlError::Io)?;
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

    #[test]
    fn check_compliant_when_service_not_installed() {
        let ctrl =
            DisableAvahi::with_paths("nonexistent-service".into(), PathBuf::from("/bin/true"));
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
    fn idempotent_apply_when_service_not_installed() {
        let ctrl =
            DisableAvahi::with_paths("nonexistent-service".into(), PathBuf::from("/bin/true"));
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
        // Apply again — should still succeed (idempotent)
        let result2 = ctrl.apply(&ctx).unwrap();
        assert_eq!(result2.status, ApplyStatus::Applied);
    }

    #[test]
    fn dry_run_skips_apply() {
        let ctrl = DisableAvahi::with_paths("avahi-daemon".into(), PathBuf::from("/bin/true"));
        let ctx = Context::with_paths(
            "test".into(),
            Distro::Debian12,
            "/tmp".into(),
            "/tmp".into(),
            "/tmp".into(),
            true, // dry_run
        );
        let result = ctrl.apply(&ctx).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }

    #[test]
    fn control_is_clonable() {
        let ctrl = DisableAvahi::new();
        let cloned = ctrl.clone_box();
        assert_eq!(cloned.id(), "2.2.3");
        assert_eq!(cloned.severity(), Severity::Low);
    }
}
