pub mod common;

use std::fs;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct EnsureRsyslogInstalled {
    binary_paths: Vec<PathBuf>,
}

impl Default for EnsureRsyslogInstalled {
    fn default() -> Self {
        EnsureRsyslogInstalled {
            binary_paths: vec![
                PathBuf::from("/usr/bin/rsyslogd"),
                PathBuf::from("/usr/sbin/rsyslogd"),
            ],
        }
    }
}

impl EnsureRsyslogInstalled {
    pub fn new() -> Self { Self::default() }
    pub fn with_paths(paths: Vec<PathBuf>) -> Self { EnsureRsyslogInstalled { binary_paths: paths } }
}

impl Control for EnsureRsyslogInstalled {
    fn id(&self) -> &str { "4.1.1.1" }
    fn description(&self) -> &str { "Ensure rsyslog is installed" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let installed = self.binary_paths.iter().any(|p| p.exists());
        Ok(if installed {
            CheckResult { status: CheckStatus::Compliant, evidence: "rsyslog is installed".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "rsyslog is not installed".into(), message: Some("install rsyslog".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would install rsyslog".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("package installation not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureRsyslogConfigured {
    config_path: PathBuf,
}

impl Default for EnsureRsyslogConfigured {
    fn default() -> Self {
        EnsureRsyslogConfigured { config_path: PathBuf::from("/etc/rsyslog.conf") }
    }
}

impl EnsureRsyslogConfigured {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(config_path: PathBuf) -> Self { EnsureRsyslogConfigured { config_path } }
}

impl Control for EnsureRsyslogConfigured {
    fn id(&self) -> &str { "4.1.1.2" }
    fn description(&self) -> &str { "Ensure rsyslog default file permissions are configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.config_path).unwrap_or_default();
        let has_auth = content.lines().any(|l| l.contains("auth,authpriv.*"));
        Ok(if has_auth {
            CheckResult { status: CheckStatus::Compliant, evidence: "rsyslog auth directive found".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "rsyslog auth directive missing".into(), message: Some("add auth,authpriv.* directive".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure rsyslog".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("rsyslog configuration not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureRsyslogPerms {
    log_dir: PathBuf,
}

impl Default for EnsureRsyslogPerms {
    fn default() -> Self {
        EnsureRsyslogPerms { log_dir: PathBuf::from("/var/log") }
    }
}

impl EnsureRsyslogPerms {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(log_dir: PathBuf) -> Self { EnsureRsyslogPerms { log_dir } }
}

impl Control for EnsureRsyslogPerms {
    fn id(&self) -> &str { "4.1.1.3" }
    fn description(&self) -> &str { "Ensure rsyslog log file permissions are configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        use std::os::unix::fs::PermissionsExt;
        let mut bad = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.log_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".log") {
                    if let Ok(meta) = entry.metadata() {
                        let mode = meta.permissions().mode() & 0o777;
                        if mode > 0o640 {
                            bad.push(name);
                        }
                    }
                }
            }
        }
        if bad.is_empty() {
            Ok(CheckResult { status: CheckStatus::Compliant, evidence: "log files have correct permissions".into(), message: None })
        } else {
            Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("log files with incorrect permissions: {:?}", bad), message: Some("log files should be 0640 or more restrictive".into()) })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would fix log perms".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("log permission fix not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureJournaldConfigured {
    config_path: PathBuf,
}

impl Default for EnsureJournaldConfigured {
    fn default() -> Self {
        EnsureJournaldConfigured { config_path: PathBuf::from("/etc/systemd/journald.conf") }
    }
}

impl EnsureJournaldConfigured {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(config_path: PathBuf) -> Self { EnsureJournaldConfigured { config_path } }
}

impl Control for EnsureJournaldConfigured {
    fn id(&self) -> &str { "4.1.2.1" }
    fn description(&self) -> &str { "Ensure journald is configured to write to persistent disk" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.config_path).unwrap_or_default();
        let persistent = content.lines().any(|l| l.trim().starts_with("Storage") && l.contains("persistent"));
        Ok(if persistent {
            CheckResult { status: CheckStatus::Compliant, evidence: "journald Storage=persistent".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "journald Storage not set to persistent".into(), message: Some("set Storage=persistent in journald.conf".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure journald".into()) });
        }
        let content = fs::read_to_string(&self.config_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;
        for line in &mut lines {
            if line.trim().starts_with("Storage") && !line.trim().starts_with('#') {
                *line = "Storage=persistent".to_string();
                found = true;
            }
        }
        if !found { lines.push("Storage=persistent".to_string()); }
        fs::write(&self.config_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set Storage=persistent".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureJournaldOverride {
    dropin_dir: PathBuf,
}

impl Default for EnsureJournaldOverride {
    fn default() -> Self {
        EnsureJournaldOverride { dropin_dir: PathBuf::from("/etc/systemd/journald.conf.d") }
    }
}

impl EnsureJournaldOverride {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(dropin_dir: PathBuf) -> Self { EnsureJournaldOverride { dropin_dir } }
}

impl Control for EnsureJournaldOverride {
    fn id(&self) -> &str { "4.1.2.2" }
    fn description(&self) -> &str { "Ensure journald drop-in is configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let dropin = self.dropin_dir.join("vallumix-journald.conf");
        Ok(if dropin.exists() {
            CheckResult { status: CheckStatus::Compliant, evidence: "journald drop-in exists".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "journald drop-in missing".into(), message: Some("create journald drop-in".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would create journald drop-in".into()) });
        }
        fs::create_dir_all(&self.dropin_dir)?;
        fs::write(self.dropin_dir.join("vallumix-journald.conf"), "[Journal]\nStorage=persistent\n")?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("created journald drop-in".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        let dropin = self.dropin_dir.join("vallumix-journald.conf");
        if dropin.exists() { fs::remove_file(&dropin)?; }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureAuditdInstalled {
    binary_paths: Vec<PathBuf>,
}

impl Default for EnsureAuditdInstalled {
    fn default() -> Self {
        EnsureAuditdInstalled {
            binary_paths: vec![
                PathBuf::from("/usr/sbin/auditd"),
                PathBuf::from("/sbin/auditd"),
            ],
        }
    }
}

impl EnsureAuditdInstalled {
    pub fn new() -> Self { Self::default() }
    pub fn with_paths(paths: Vec<PathBuf>) -> Self { EnsureAuditdInstalled { binary_paths: paths } }
}

impl Control for EnsureAuditdInstalled {
    fn id(&self) -> &str { "4.1.3.1" }
    fn description(&self) -> &str { "Ensure auditd is installed" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let installed = self.binary_paths.iter().any(|p| p.exists());
        Ok(if installed {
            CheckResult { status: CheckStatus::Compliant, evidence: "auditd is installed".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "auditd is not installed".into(), message: Some("install auditd".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would install auditd".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("package installation not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureAuditdConfigured {
    config_path: PathBuf,
}

impl Default for EnsureAuditdConfigured {
    fn default() -> Self {
        EnsureAuditdConfigured { config_path: PathBuf::from("/etc/audit/auditd.conf") }
    }
}

impl EnsureAuditdConfigured {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(config_path: PathBuf) -> Self { EnsureAuditdConfigured { config_path } }
}

impl Control for EnsureAuditdConfigured {
    fn id(&self) -> &str { "4.1.3.2" }
    fn description(&self) -> &str { "Ensure auditd max_log_file_action is configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.config_path).unwrap_or_default();
        let ok = content.lines().any(|l| l.trim().starts_with("max_log_file_action") && l.contains("keep_logs"));
        Ok(if ok {
            CheckResult { status: CheckStatus::Compliant, evidence: "max_log_file_action = keep_logs".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "max_log_file_action not set to keep_logs".into(), message: Some("set max_log_file_action = keep_logs".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure auditd".into()) });
        }
        let content = fs::read_to_string(&self.config_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;
        for line in &mut lines {
            if line.trim().starts_with("max_log_file_action") && !line.trim().starts_with('#') {
                *line = "max_log_file_action = keep_logs".to_string();
                found = true;
            }
        }
        if !found { lines.push("max_log_file_action = keep_logs".to_string()); }
        fs::write(&self.config_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set max_log_file_action = keep_logs".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureAuditIdentityRules {
    rules_path: PathBuf,
}

impl Default for EnsureAuditIdentityRules {
    fn default() -> Self {
        EnsureAuditIdentityRules { rules_path: PathBuf::from("/etc/audit/rules.d/vallumix.rules") }
    }
}

impl EnsureAuditIdentityRules {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(rules_path: PathBuf) -> Self { EnsureAuditIdentityRules { rules_path } }
}

impl Control for EnsureAuditIdentityRules {
    fn id(&self) -> &str { "4.1.4.1" }
    fn description(&self) -> &str { "Ensure audit identity events are collected" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        if !self.rules_path.exists() {
            return Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "audit rules file missing".into(), message: Some("create audit identity rules".into()) });
        }
        let content = fs::read_to_string(&self.rules_path).unwrap_or_default();
        let has_rule = content.contains("/etc/group") && content.contains("/etc/passwd");
        Ok(if has_rule {
            CheckResult { status: CheckStatus::Compliant, evidence: "audit identity rules present".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "audit identity rules incomplete".into(), message: Some("add identity audit rules".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would add audit identity rules".into()) });
        }
        fs::create_dir_all(self.rules_path.parent().unwrap())?;
        let rules = "-w /etc/passwd -p wa -k identity\n-w /etc/group -p wa -k identity\n";
        fs::write(&self.rules_path, rules)?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("added audit identity rules".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        if self.rules_path.exists() { fs::remove_file(&self.rules_path)?; }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureAuditLoginEvents {
    rules_path: PathBuf,
}

impl Default for EnsureAuditLoginEvents {
    fn default() -> Self {
        EnsureAuditLoginEvents { rules_path: PathBuf::from("/etc/audit/rules.d/vallumix.rules") }
    }
}

impl EnsureAuditLoginEvents {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(rules_path: PathBuf) -> Self { EnsureAuditLoginEvents { rules_path } }
}

impl Control for EnsureAuditLoginEvents {
    fn id(&self) -> &str { "4.1.4.2" }
    fn description(&self) -> &str { "Ensure audit login events are collected" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        if !self.rules_path.exists() {
            return Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "audit rules file missing".into(), message: Some("create audit login rules".into()) });
        }
        let content = fs::read_to_string(&self.rules_path).unwrap_or_default();
        let has_rule = content.contains("/var/log/wtmp") || content.contains("/var/log/btmp");
        Ok(if has_rule {
            CheckResult { status: CheckStatus::Compliant, evidence: "audit login rules present".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "audit login rules incomplete".into(), message: Some("add login audit rules".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would add audit login rules".into()) });
        }
        fs::create_dir_all(self.rules_path.parent().unwrap())?;
        let rules = "-w /var/log/wtmp -p wa -k logins\n-w /var/log/btmp -p wa -k logins\n";
        fs::write(&self.rules_path, rules)?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("added audit login rules".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        if self.rules_path.exists() { fs::remove_file(&self.rules_path)?; }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureAuditSessionEvents {
    rules_path: PathBuf,
}

impl Default for EnsureAuditSessionEvents {
    fn default() -> Self {
        EnsureAuditSessionEvents { rules_path: PathBuf::from("/etc/audit/rules.d/vallumix.rules") }
    }
}

impl EnsureAuditSessionEvents {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(rules_path: PathBuf) -> Self { EnsureAuditSessionEvents { rules_path } }
}

impl Control for EnsureAuditSessionEvents {
    fn id(&self) -> &str { "4.1.4.3" }
    fn description(&self) -> &str { "Ensure audit session events are collected" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        if !self.rules_path.exists() {
            return Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "audit rules file missing".into(), message: Some("create audit session rules".into()) });
        }
        let content = fs::read_to_string(&self.rules_path).unwrap_or_default();
        let has_rule = content.contains("/var/run/utmp");
        Ok(if has_rule {
            CheckResult { status: CheckStatus::Compliant, evidence: "audit session rules present".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "audit session rules incomplete".into(), message: Some("add session audit rules".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would add audit session rules".into()) });
        }
        fs::create_dir_all(self.rules_path.parent().unwrap())?;
        let rules = "-w /var/run/utmp -p wa -k session\n";
        fs::write(&self.rules_path, rules)?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("added audit session rules".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> {
        if self.rules_path.exists() { fs::remove_file(&self.rules_path)?; }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureLogrotate {
    config_path: PathBuf,
}

impl Default for EnsureLogrotate {
    fn default() -> Self {
        EnsureLogrotate { config_path: PathBuf::from("/etc/logrotate.d/rsyslog") }
    }
}

impl EnsureLogrotate {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(config_path: PathBuf) -> Self { EnsureLogrotate { config_path } }
}

impl Control for EnsureLogrotate {
    fn id(&self) -> &str { "4.1.7" }
    fn description(&self) -> &str { "Ensure logrotate is configured" }
    fn severity(&self) -> Severity { Severity::Low }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Logging }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        if !self.config_path.exists() {
            return Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "logrotate config missing".into(), message: Some("configure logrotate".into()) });
        }
        let content = fs::read_to_string(&self.config_path).unwrap_or_default();
        let has_weekly = content.contains("weekly") || content.contains("daily");
        Ok(if has_weekly {
            CheckResult { status: CheckStatus::Compliant, evidence: "logrotate configured".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "logrotate rotation not configured".into(), message: Some("add rotation schedule".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure logrotate".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("logrotate configuration not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::common::LoggingContext;
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

    #[test]
    fn ensure_rsyslog_installed_check_compliant_when_binary_exists() {
        let ctx = LoggingContext::new();
        let bin = ctx.tmpdir.path().join("rsyslogd");
        std::fs::write(&bin, "").unwrap();
        let ctrl = EnsureRsyslogInstalled::with_paths(vec![bin]);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_rsyslog_installed_check_non_compliant_when_binary_absent() {
        let ctrl = EnsureRsyslogInstalled::with_paths(vec![PathBuf::from("/tmp/nonexistent-rsyslogd")]);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_rsyslog_configured_check_compliant_when_directive_present() {
        let ctx = LoggingContext::new();
        ctx.write_rsyslog("auth,authpriv.* /var/log/auth.log\n");
        let ctrl = EnsureRsyslogConfigured::with_path(ctx.rsyslog_conf_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_rsyslog_configured_check_non_compliant_when_directive_absent() {
        let ctx = LoggingContext::new();
        ctx.write_rsyslog("*.* /var/log/syslog\n");
        let ctrl = EnsureRsyslogConfigured::with_path(ctx.rsyslog_conf_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_rsyslog_perms_check_compliant_when_perms_0640() {
        let ctx = LoggingContext::new();
        ctx.create_log_file("auth.log", 0o640);
        let ctrl = EnsureRsyslogPerms::with_path(ctx.log_dir());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_rsyslog_perms_check_non_compliant_when_perms_too_permissive() {
        let ctx = LoggingContext::new();
        ctx.create_log_file("auth.log", 0o666);
        let ctrl = EnsureRsyslogPerms::with_path(ctx.log_dir());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_journald_configured_check_compliant_when_storage_persistent() {
        let ctx = LoggingContext::new();
        ctx.write_journald("[Journal]\nStorage=persistent\n");
        let ctrl = EnsureJournaldConfigured::with_path(ctx.journald_conf_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_journald_configured_apply_creates_drop_in() {
        let ctx = LoggingContext::new();
        ctx.write_journald("[Journal]\nStorage=auto\n");
        let ctrl = EnsureJournaldConfigured::with_path(ctx.journald_conf_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.journald_conf_path()).unwrap();
        assert!(content.contains("Storage=persistent"));
    }

    #[test]
    fn ensure_journald_override_check_compliant_when_dropin_exists() {
        let ctx = LoggingContext::new();
        let dropin = ctx.journald_dropin_dir().join("vallumix-journald.conf");
        std::fs::create_dir_all(ctx.journald_dropin_dir()).unwrap();
        std::fs::write(&dropin, "[Journal]\n").unwrap();
        let ctrl = EnsureJournaldOverride::with_path(ctx.journald_dropin_dir());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_journald_override_apply_creates_dropin_with_compress() {
        let ctx = LoggingContext::new();
        let ctrl = EnsureJournaldOverride::with_path(ctx.journald_dropin_dir());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let dropin = ctx.journald_dropin_dir().join("vallumix-journald.conf");
        assert!(dropin.exists());
        let content = std::fs::read_to_string(&dropin).unwrap();
        assert!(content.contains("Storage=persistent"));
    }

    #[test]
    fn ensure_auditd_installed_check_compliant_when_binary_exists() {
        let ctx = LoggingContext::new();
        let bin = ctx.tmpdir.path().join("auditd");
        std::fs::write(&bin, "").unwrap();
        let ctrl = EnsureAuditdInstalled::with_paths(vec![bin]);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_auditd_installed_check_non_compliant_when_absent() {
        let ctrl = EnsureAuditdInstalled::with_paths(vec![PathBuf::from("/tmp/nonexistent-auditd")]);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_auditd_configured_check_compliant_when_keep_logs() {
        let ctx = LoggingContext::new();
        ctx.write_auditd("max_log_file_action = keep_logs\n");
        let ctrl = EnsureAuditdConfigured::with_path(ctx.auditd_conf_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_auditd_configured_apply_updates_auditd_conf() {
        let ctx = LoggingContext::new();
        ctx.write_auditd("max_log_file_action = SYSLOG\n");
        let ctrl = EnsureAuditdConfigured::with_path(ctx.auditd_conf_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.auditd_conf_path()).unwrap();
        assert!(content.contains("keep_logs"));
    }

    #[test]
    fn ensure_audit_identity_rules_check_compliant_when_rule_exists() {
        let ctx = LoggingContext::new();
        ctx.write_audit_rules("-w /etc/passwd -p wa -k identity\n-w /etc/group -p wa -k identity\n");
        let ctrl = EnsureAuditIdentityRules::with_path(ctx.audit_rules_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_audit_identity_rules_apply_appends_rules_with_backup() {
        let ctx = LoggingContext::new();
        let ctrl = EnsureAuditIdentityRules::with_path(ctx.audit_rules_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.audit_rules_path()).unwrap();
        assert!(content.contains("/etc/passwd"));
        assert!(content.contains("/etc/group"));
    }

    #[test]
    fn ensure_audit_login_events_check_compliant_when_login_rules_present() {
        let ctx = LoggingContext::new();
        ctx.write_audit_rules("-w /var/log/wtmp -p wa -k logins\n");
        let ctrl = EnsureAuditLoginEvents::with_path(ctx.audit_rules_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_audit_login_events_apply_appends_session_rules() {
        let ctx = LoggingContext::new();
        let ctrl = EnsureAuditLoginEvents::with_path(ctx.audit_rules_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.audit_rules_path()).unwrap();
        assert!(content.contains("/var/log/wtmp"));
    }

    #[test]
    fn ensure_audit_session_events_check_compliant_when_session_rules_present() {
        let ctx = LoggingContext::new();
        ctx.write_audit_rules("-w /var/run/utmp -p wa -k session\n");
        let ctrl = EnsureAuditSessionEvents::with_path(ctx.audit_rules_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_logrotate_check_compliant_when_config_exists() {
        let ctx = LoggingContext::new();
        ctx.write_logrotate("/var/log/syslog {\n  weekly\n}\n");
        let ctrl = EnsureLogrotate::with_path(ctx.logrotate_conf_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_logrotate_check_non_compliant_when_config_missing() {
        let ctrl = EnsureLogrotate::with_path(PathBuf::from("/tmp/nonexistent-logrotate"));
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_rsyslog_configured_dry_run_skips_apply() {
        let ctx = LoggingContext::new();
        let ctrl = EnsureRsyslogConfigured::with_path(ctx.rsyslog_conf_path());
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }

    #[test]
    fn ensure_auditd_configured_dry_run_skips_apply() {
        let ctx = LoggingContext::new();
        let ctrl = EnsureAuditdConfigured::with_path(ctx.auditd_conf_path());
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }
}
