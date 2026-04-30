use std::fs;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct SshdConfigControl {
    id: &'static str,
    description: &'static str,
    severity: Severity,
    sshd_config_path: PathBuf,
    directive: &'static str,
    expected_values: Vec<&'static str>,
    apply_value: &'static str,
}

impl SshdConfigControl {
    pub fn new(
        id: &'static str,
        description: &'static str,
        severity: Severity,
        directive: &'static str,
        expected_values: Vec<&'static str>,
        apply_value: &'static str,
    ) -> Self {
        SshdConfigControl {
            id,
            description,
            severity,
            sshd_config_path: PathBuf::from("/etc/ssh/sshd_config"),
            directive,
            expected_values,
            apply_value,
        }
    }

    pub fn with_path(
        id: &'static str,
        description: &'static str,
        severity: Severity,
        sshd_config_path: PathBuf,
        directive: &'static str,
        expected_values: Vec<&'static str>,
        apply_value: &'static str,
    ) -> Self {
        SshdConfigControl {
            id,
            description,
            severity,
            sshd_config_path,
            directive,
            expected_values,
            apply_value,
        }
    }

    fn find_value(&self) -> Option<String> {
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix(self.directive) {
                let value = rest.trim().trim_start_matches(|c| c == ' ' || c == '=' || c == '\t');
                return Some(value.into());
            }
        }
        None
    }
}

impl Control for SshdConfigControl {
    fn id(&self) -> &str { self.id }
    fn description(&self) -> &str { self.description }
    fn severity(&self) -> Severity { self.severity }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Ssh }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        match self.find_value() {
            Some(value) => {
                if self.expected_values.iter().any(|&ev| ev.eq_ignore_ascii_case(&value)) {
                    Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("{} {}", self.directive, value), message: None })
                } else {
                    Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("{} {} (expected one of: {:?})", self.directive, value, self.expected_values), message: Some(format!("{} should be set to {}", self.directive, self.apply_value)) })
                }
            }
            None => {
                // Some directives (like Protocol) default to safe values
                if self.directive == "Protocol" {
                    return Ok(CheckResult { status: CheckStatus::Compliant, evidence: "Protocol defaults to 2".into(), message: None });
                }
                Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("{} not found", self.directive), message: Some(format!("{} should be explicitly set to {}", self.directive, self.apply_value)) })
            }
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some(format!("dry-run: would set {} {}", self.directive, self.apply_value)) });
        }
        let content = fs::read_to_string(&self.sshd_config_path).unwrap_or_default();
        let mut found = false;
        let mut new_lines: Vec<String> = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(self.directive) && !trimmed.starts_with('#') {
                new_lines.push(format!("{} {}", self.directive, self.apply_value));
                found = true;
            } else {
                new_lines.push(line.to_string());
            }
        }
        if !found {
            new_lines.push(format!("{} {}", self.directive, self.apply_value));
        }
        fs::write(&self.sshd_config_path, new_lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some(format!("set {} {}", self.directive, self.apply_value)) })
    }

    fn rollback(&self, _ctx: &Context, backup: &Backup) -> Result<(), ControlError> {
        if backup.backup_path.exists() {
            fs::copy(&backup.backup_path, &backup.original_path)?;
        }
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

    fn make_sshd_config(path: &std::path::Path, content: &str) {
        std::fs::write(path, content).unwrap();
    }

    #[test]
    fn new_sets_defaults() {
        let ctrl = SshdConfigControl::new(
            "5.2.1",
            "Ensure SSH Protocol is set to 2",
            Severity::Medium,
            "Protocol",
            vec!["2"],
            "2",
        );
        assert_eq!(ctrl.id(), "5.2.1");
        assert_eq!(ctrl.sshd_config_path, PathBuf::from("/etc/ssh/sshd_config"));
    }

    #[test]
    fn find_value_parses_directive() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Port 22\nProtocol 2\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config.clone(), "Protocol",
            vec!["2"], "2",
        );
        assert_eq!(ctrl.find_value(), Some("2".into()));
    }

    #[test]
    fn find_value_ignores_comments_and_empty_lines() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "# Protocol 1\n\nProtocol 2\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config, "Protocol",
            vec!["2"], "2",
        );
        assert_eq!(ctrl.find_value(), Some("2".into()));
    }

    #[test]
    fn find_value_returns_none_when_missing() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Port 22\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config, "Protocol",
            vec!["2"], "2",
        );
        assert_eq!(ctrl.find_value(), None);
    }

    #[test]
    fn check_compliant_when_value_matches() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Protocol 2\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config, "Protocol",
            vec!["2"], "2",
        );
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("Protocol 2"));
    }

    #[test]
    fn check_compliant_protocol_defaults_to_2() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Port 22\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config, "Protocol",
            vec!["2"], "2",
        );
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("defaults to 2"));
    }

    #[test]
    fn check_non_compliant_when_value_wrong() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Protocol 1\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config, "Protocol",
            vec!["2"], "2",
        );
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("Protocol 1"));
    }

    #[test]
    fn check_non_compliant_when_directive_missing_and_no_default() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Port 22\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.4", "desc", Severity::Low, config, "PermitRootLogin",
            vec!["no"], "no",
        );
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("not found"));
    }

    #[test]
    fn apply_updates_existing_directive() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Protocol 1\nPort 22\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config.clone(), "Protocol",
            vec!["2"], "2",
        );
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);

        let content = std::fs::read_to_string(&config).unwrap();
        assert!(content.contains("Protocol 2"));
        assert!(!content.contains("Protocol 1"));
    }

    #[test]
    fn apply_appends_directive_when_missing() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Port 22\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config.clone(), "Protocol",
            vec!["2"], "2",
        );
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);

        let content = std::fs::read_to_string(&config).unwrap();
        assert!(content.contains("Protocol 2"));
    }

    #[test]
    fn apply_skips_when_dry_run() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        make_sshd_config(&config, "Protocol 1\n");

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config.clone(), "Protocol",
            vec!["2"], "2",
        );
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);

        let content = std::fs::read_to_string(&config).unwrap();
        assert!(content.contains("Protocol 1"));
    }

    #[test]
    fn rollback_copies_backup() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = tmpdir.path().join("sshd_config");
        let backup = tmpdir.path().join("sshd_config.bak");
        std::fs::write(&config, "modified").unwrap();
        std::fs::write(&backup, "original").unwrap();

        let ctrl = SshdConfigControl::with_path(
            "5.2.1", "desc", Severity::Low, config.clone(), "Protocol",
            vec!["2"], "2",
        );
        let b = vallumix_core::profile::Backup {
            id: "b".into(),
            timestamp: chrono::Utc::now(),
            original_path: config.clone(),
            backup_path: backup,
        };
        ctrl.rollback(&test_ctx(false), &b).unwrap();
        assert_eq!(std::fs::read_to_string(&config).unwrap(), "original");
    }

    #[test]
    fn clone_box_produces_identical_control() {
        let ctrl = SshdConfigControl::new("5.2.1", "desc", Severity::Low, "Protocol", vec!["2"], "2");
        let cloned = ctrl.clone_box();
        assert_eq!(cloned.id(), "5.2.1");
    }
}
