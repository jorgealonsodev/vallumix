pub mod common;

use std::fs;
use std::path::PathBuf;

use vallumix_core::control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus, Control, Severity};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::error::ControlError;
use vallumix_core::profile::Backup;

#[derive(Debug, Clone)]
pub struct EnsureCronDaemon {
    cron_paths: Vec<PathBuf>,
}

impl Default for EnsureCronDaemon {
    fn default() -> Self {
        EnsureCronDaemon {
            cron_paths: vec![
                PathBuf::from("/lib/systemd/system/cron.service"),
                PathBuf::from("/usr/lib/systemd/system/crond.service"),
            ],
        }
    }
}

impl EnsureCronDaemon {
    pub fn new() -> Self { Self::default() }
    pub fn with_paths(paths: Vec<PathBuf>) -> Self { EnsureCronDaemon { cron_paths: paths } }
}

impl Control for EnsureCronDaemon {
    fn id(&self) -> &str { "5.1.1" }
    fn description(&self) -> &str { "Ensure cron daemon is enabled" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let exists = self.cron_paths.iter().any(|p| p.exists());
        Ok(if exists {
            CheckResult { status: CheckStatus::Compliant, evidence: "cron service found".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "cron service not found".into(), message: Some("ensure cron is installed".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would enable cron".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("cron enablement not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsurePamPasswordQuality {
    pam_path: PathBuf,
}

impl Default for EnsurePamPasswordQuality {
    fn default() -> Self {
        EnsurePamPasswordQuality { pam_path: PathBuf::from("/etc/pam.d/common-password") }
    }
}

impl EnsurePamPasswordQuality {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(pam_path: PathBuf) -> Self { EnsurePamPasswordQuality { pam_path } }
}

impl Control for EnsurePamPasswordQuality {
    fn id(&self) -> &str { "5.3.1" }
    fn description(&self) -> &str { "Ensure password quality checking is enabled" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.pam_path).unwrap_or_default();
        let has_pwquality = content.contains("pam_pwquality.so") || content.contains("pam_cracklib.so");
        Ok(if has_pwquality {
            CheckResult { status: CheckStatus::Compliant, evidence: "password quality module found".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "password quality module missing".into(), message: Some("enable pam_pwquality".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure PAM".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("PAM configuration not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsurePamMinlen {
    pwquality_path: PathBuf,
}

impl Default for EnsurePamMinlen {
    fn default() -> Self {
        EnsurePamMinlen { pwquality_path: PathBuf::from("/etc/security/pwquality.conf") }
    }
}

impl EnsurePamMinlen {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(pwquality_path: PathBuf) -> Self { EnsurePamMinlen { pwquality_path } }
}

impl Control for EnsurePamMinlen {
    fn id(&self) -> &str { "5.3.2" }
    fn description(&self) -> &str { "Ensure password minimum length is configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.pwquality_path).unwrap_or_default();
        for line in content.lines() {
            if line.trim().starts_with("minlen") {
                let val: i32 = line.split('=').nth(1).unwrap_or("0").trim().parse().unwrap_or(0);
                if val >= 14 {
                    return Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("minlen = {}", val), message: None });
                } else {
                    return Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("minlen = {} (expected >= 14)", val), message: Some("set minlen >= 14".into()) });
                }
            }
        }
        Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "minlen not configured".into(), message: Some("set minlen >= 14".into()) })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would set minlen".into()) });
        }
        let content = fs::read_to_string(&self.pwquality_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;
        for line in &mut lines {
            if line.trim().starts_with("minlen") && !line.trim().starts_with('#') {
                *line = "minlen = 14".to_string();
                found = true;
            }
        }
        if !found { lines.push("minlen = 14".to_string()); }
        fs::write(&self.pwquality_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set minlen = 14".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsurePamCredit {
    pwquality_path: PathBuf,
}

impl Default for EnsurePamCredit {
    fn default() -> Self {
        EnsurePamCredit { pwquality_path: PathBuf::from("/etc/security/pwquality.conf") }
    }
}

impl EnsurePamCredit {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(pwquality_path: PathBuf) -> Self { EnsurePamCredit { pwquality_path } }
}

impl Control for EnsurePamCredit {
    fn id(&self) -> &str { "5.3.3" }
    fn description(&self) -> &str { "Ensure password complexity credits are configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.pwquality_path).unwrap_or_default();
        let mut missing = Vec::new();
        for name in ["dcredit", "ucredit", "lcredit", "ocredit"] {
            let found = content.lines().any(|l| l.trim().starts_with(name) && l.contains("-1"));
            if !found { missing.push(name); }
        }
        if missing.is_empty() {
            Ok(CheckResult { status: CheckStatus::Compliant, evidence: "all credit parameters set to -1".into(), message: None })
        } else {
            Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("missing credits: {:?}", missing), message: Some("set all credit params to -1".into()) })
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would set credits".into()) });
        }
        let content = fs::read_to_string(&self.pwquality_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        for name in ["dcredit", "ucredit", "lcredit", "ocredit"] {
            let mut found = false;
            for line in &mut lines {
                if line.trim().starts_with(name) && !line.trim().starts_with('#') {
                    *line = format!("{} = -1", name);
                    found = true;
                }
            }
            if !found { lines.push(format!("{} = -1", name)); }
        }
        fs::write(&self.pwquality_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set credit parameters".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsurePamFaillock {
    pam_path: PathBuf,
}

impl Default for EnsurePamFaillock {
    fn default() -> Self {
        EnsurePamFaillock { pam_path: PathBuf::from("/etc/pam.d/common-auth") }
    }
}

impl EnsurePamFaillock {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(pam_path: PathBuf) -> Self { EnsurePamFaillock { pam_path } }
}

impl Control for EnsurePamFaillock {
    fn id(&self) -> &str { "5.3.4" }
    fn description(&self) -> &str { "Ensure PAM faillock is configured" }
    fn severity(&self) -> Severity { Severity::High }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.pam_path).unwrap_or_default();
        let has_faillock = content.contains("pam_faillock.so");
        Ok(if has_faillock {
            CheckResult { status: CheckStatus::Compliant, evidence: "pam_faillock configured".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "pam_faillock not configured".into(), message: Some("configure pam_faillock".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure faillock".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("faillock configuration not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsurePamRemember {
    pam_path: PathBuf,
}

impl Default for EnsurePamRemember {
    fn default() -> Self {
        EnsurePamRemember { pam_path: PathBuf::from("/etc/pam.d/common-password") }
    }
}

impl EnsurePamRemember {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(pam_path: PathBuf) -> Self { EnsurePamRemember { pam_path } }
}

impl Control for EnsurePamRemember {
    fn id(&self) -> &str { "5.3.5" }
    fn description(&self) -> &str { "Ensure password history is configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.pam_path).unwrap_or_default();
        let has_remember = content.contains("remember=");
        Ok(if has_remember {
            CheckResult { status: CheckStatus::Compliant, evidence: "password remember found".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "password remember not configured".into(), message: Some("add remember=5 to pam_unix".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would configure remember".into()) });
        }
        Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("remember configuration not implemented".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsurePasswordHashing {
    login_defs_path: PathBuf,
}

impl Default for EnsurePasswordHashing {
    fn default() -> Self {
        EnsurePasswordHashing { login_defs_path: PathBuf::from("/etc/login.defs") }
    }
}

impl EnsurePasswordHashing {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(login_defs_path: PathBuf) -> Self { EnsurePasswordHashing { login_defs_path } }
}

impl Control for EnsurePasswordHashing {
    fn id(&self) -> &str { "5.4.1" }
    fn description(&self) -> &str { "Ensure password hashing algorithm is SHA512 or yescrypt" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.login_defs_path).unwrap_or_default();
        let ok = content.lines().any(|l| l.trim().starts_with("ENCRYPT_METHOD") && (l.contains("SHA512") || l.contains("yescrypt")));
        Ok(if ok {
            CheckResult { status: CheckStatus::Compliant, evidence: "password hashing method OK".into(), message: None }
        } else {
            CheckResult { status: CheckStatus::NonCompliant, evidence: "password hashing method not SHA512/yescrypt".into(), message: Some("set ENCRYPT_METHOD SHA512 or yescrypt".into()) }
        })
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would set ENCRYPT_METHOD".into()) });
        }
        let content = fs::read_to_string(&self.login_defs_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;
        for line in &mut lines {
            if line.trim().starts_with("ENCRYPT_METHOD") && !line.trim().starts_with('#') {
                *line = "ENCRYPT_METHOD SHA512".to_string();
                found = true;
            }
        }
        if !found { lines.push("ENCRYPT_METHOD SHA512".to_string()); }
        fs::write(&self.login_defs_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set ENCRYPT_METHOD SHA512".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureUmask {
    profile_path: PathBuf,
    bashrc_path: PathBuf,
}

impl Default for EnsureUmask {
    fn default() -> Self {
        EnsureUmask {
            profile_path: PathBuf::from("/etc/profile"),
            bashrc_path: PathBuf::from("/etc/bash.bashrc"),
        }
    }
}

impl EnsureUmask {
    pub fn new() -> Self { Self::default() }
    pub fn with_paths(profile_path: PathBuf, bashrc_path: PathBuf) -> Self {
        EnsureUmask { profile_path, bashrc_path }
    }
}

impl Control for EnsureUmask {
    fn id(&self) -> &str { "5.5.1" }
    fn description(&self) -> &str { "Ensure default user umask is 0077 or more restrictive" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.profile_path).unwrap_or_default();
        let mut umask_val = None;
        for line in content.lines() {
            if line.trim().starts_with("umask") {
                let val = line.split_whitespace().nth(1).unwrap_or("022");
                umask_val = Some(val.to_string());
            }
        }
        match umask_val {
            Some(v) => {
                let val = u32::from_str_radix(&v, 8).unwrap_or(22);
                if val >= 0o77 {
                    Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("umask = {:03o}", val), message: None })
                } else {
                    Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("umask = {:03o} (too permissive)", val), message: Some("set umask 0077".into()) })
                }
            }
            None => Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "umask not set in /etc/profile".into(), message: Some("set umask 0077".into()) }),
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would set umask".into()) });
        }
        for path in [&self.profile_path, &self.bashrc_path] {
            let content = fs::read_to_string(path).unwrap_or_default();
            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            let mut found = false;
            for line in &mut lines {
                if line.trim().starts_with("umask") && !line.trim().starts_with('#') {
                    *line = "umask 0077".to_string();
                    found = true;
                }
            }
            if !found { lines.push("umask 0077".to_string()); }
            fs::write(path, lines.join("\n"))?;
        }
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set umask 0077".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct EnsureShellTimeout {
    profile_path: PathBuf,
}

impl Default for EnsureShellTimeout {
    fn default() -> Self {
        EnsureShellTimeout { profile_path: PathBuf::from("/etc/profile") }
    }
}

impl EnsureShellTimeout {
    pub fn new() -> Self { Self::default() }
    pub fn with_path(profile_path: PathBuf) -> Self { EnsureShellTimeout { profile_path } }
}

impl Control for EnsureShellTimeout {
    fn id(&self) -> &str { "5.5.2" }
    fn description(&self) -> &str { "Ensure shell timeout is configured" }
    fn severity(&self) -> Severity { Severity::Medium }
    fn applicable_distros(&self) -> &[Distro] {
        &[Distro::Debian12, Distro::Ubuntu2204, Distro::Ubuntu2404, Distro::Rocky9]
    }
    fn category(&self) -> Category { Category::Auth }

    fn check(&self, _ctx: &Context) -> Result<CheckResult, ControlError> {
        let content = fs::read_to_string(&self.profile_path).unwrap_or_default();
        let mut tmout = None;
        for line in content.lines() {
            if line.trim().starts_with("TMOUT") {
                tmout = line.split('=').nth(1).and_then(|s| s.trim().parse::<u32>().ok());
            }
        }
        match tmout {
            Some(v) if v <= 300 => Ok(CheckResult { status: CheckStatus::Compliant, evidence: format!("TMOUT = {}", v), message: None }),
            Some(v) => Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: format!("TMOUT = {} (too high)", v), message: Some("set TMOUT <= 300".into()) }),
            None => Ok(CheckResult { status: CheckStatus::NonCompliant, evidence: "TMOUT not set".into(), message: Some("set TMOUT=300".into()) }),
        }
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        if ctx.dry_run {
            return Ok(ApplyResult { status: ApplyStatus::Skipped, backup_path: None, message: Some("dry-run: would set TMOUT".into()) });
        }
        let content = fs::read_to_string(&self.profile_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut found = false;
        for line in &mut lines {
            if line.trim().starts_with("TMOUT") && !line.trim().starts_with('#') {
                *line = "TMOUT=300".to_string();
                found = true;
            }
        }
        if !found { lines.push("TMOUT=300".to_string()); }
        fs::write(&self.profile_path, lines.join("\n"))?;
        Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: Some("set TMOUT=300".into()) })
    }

    fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
    fn clone_box(&self) -> Box<dyn Control> { Box::new(self.clone()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::common::AuthContext;
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
    fn ensure_cron_daemon_check_compliant_when_unit_absent() {
        let ctrl = EnsureCronDaemon::with_paths(vec![PathBuf::from("/tmp/nonexistent-cron-service")]);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("not found"));
    }

    #[test]
    fn ensure_cron_daemon_check_compliant_when_unit_exists() {
        let tmpdir = tempfile::tempdir().unwrap();
        let svc = tmpdir.path().join("cron.service");
        std::fs::write(&svc, "[Service]\n").unwrap();
        let ctrl = EnsureCronDaemon::with_paths(vec![svc]);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("found"));
    }

    #[test]
    fn ensure_pam_password_quality_check_compliant_when_module_present() {
        let ctx = AuthContext::new();
        ctx.write_pam("password requisite pam_pwquality.so try_first_pass retry=3\n");
        let ctrl = EnsurePamPasswordQuality::with_path(ctx.pam_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_pam_password_quality_check_non_compliant_when_module_absent() {
        let ctx = AuthContext::new();
        ctx.write_pam("password requisite pam_unix.so\n");
        let ctrl = EnsurePamPasswordQuality::with_path(ctx.pam_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_pam_minlen_check_compliant_when_minlen_14() {
        let ctx = AuthContext::new();
        ctx.write_pwquality("minlen = 14\n");
        let ctrl = EnsurePamMinlen::with_path(ctx.pwquality_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("minlen = 14"));
    }

    #[test]
    fn ensure_pam_minlen_check_non_compliant_when_minlen_8() {
        let ctx = AuthContext::new();
        ctx.write_pwquality("minlen = 8\n");
        let ctrl = EnsurePamMinlen::with_path(ctx.pwquality_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("minlen = 8"));
    }

    #[test]
    fn ensure_pam_minlen_apply_writes_minlen() {
        let ctx = AuthContext::new();
        ctx.write_pwquality("minlen = 8\n");
        let ctrl = EnsurePamMinlen::with_path(ctx.pwquality_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.pwquality_path()).unwrap();
        assert!(content.contains("minlen = 14"));
    }

    #[test]
    fn ensure_pam_credit_check_compliant_when_all_negative_one() {
        let ctx = AuthContext::new();
        ctx.write_pwquality("dcredit = -1\nucredit = -1\nlcredit = -1\nocredit = -1\n");
        let ctrl = EnsurePamCredit::with_path(ctx.pwquality_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_pam_credit_check_non_compliant_when_credits_zero() {
        let ctx = AuthContext::new();
        ctx.write_pwquality("dcredit = 0\nucredit = 0\n");
        let ctrl = EnsurePamCredit::with_path(ctx.pwquality_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("missing credits"));
    }

    #[test]
    fn ensure_pam_credit_apply_writes_credit_settings() {
        let ctx = AuthContext::new();
        ctx.write_pwquality("");
        let ctrl = EnsurePamCredit::with_path(ctx.pwquality_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.pwquality_path()).unwrap();
        assert!(content.contains("dcredit = -1"));
        assert!(content.contains("ucredit = -1"));
        assert!(content.contains("lcredit = -1"));
        assert!(content.contains("ocredit = -1"));
    }

    #[test]
    fn ensure_pam_faillock_check_compliant_when_configured() {
        let ctx = AuthContext::new();
        ctx.write_pam("auth required pam_faillock.so preauth deny=5 unlock_time=900\n");
        let ctrl = EnsurePamFaillock::with_path(ctx.pam_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_pam_faillock_check_non_compliant_when_missing() {
        let ctx = AuthContext::new();
        ctx.write_pam("auth required pam_unix.so\n");
        let ctrl = EnsurePamFaillock::with_path(ctx.pam_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_pam_remember_check_compliant_when_remember_5() {
        let ctx = AuthContext::new();
        ctx.write_pam("password requisite pam_unix.so remember=5\n");
        let ctrl = EnsurePamRemember::with_path(ctx.pam_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_pam_remember_check_non_compliant_when_no_remember() {
        let ctx = AuthContext::new();
        ctx.write_pam("password requisite pam_unix.so\n");
        let ctrl = EnsurePamRemember::with_path(ctx.pam_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_password_hashing_check_compliant_when_sha512() {
        let ctx = AuthContext::new();
        ctx.write_login_defs("ENCRYPT_METHOD SHA512\n");
        let ctrl = EnsurePasswordHashing::with_path(ctx.login_defs_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
    }

    #[test]
    fn ensure_password_hashing_check_non_compliant_when_md5() {
        let ctx = AuthContext::new();
        ctx.write_login_defs("ENCRYPT_METHOD MD5\n");
        let ctrl = EnsurePasswordHashing::with_path(ctx.login_defs_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
    }

    #[test]
    fn ensure_password_hashing_apply_writes_sha512() {
        let ctx = AuthContext::new();
        ctx.write_login_defs("ENCRYPT_METHOD MD5\n");
        let ctrl = EnsurePasswordHashing::with_path(ctx.login_defs_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.login_defs_path()).unwrap();
        assert!(content.contains("ENCRYPT_METHOD SHA512"));
    }

    #[test]
    fn ensure_umask_check_compliant_when_umask_077() {
        let ctx = AuthContext::new();
        ctx.write_profile("umask 0077\n");
        let ctrl = EnsureUmask::with_paths(ctx.profile_path(), ctx.bashrc_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("077"));
    }

    #[test]
    fn ensure_umask_check_non_compliant_when_umask_022() {
        let ctx = AuthContext::new();
        ctx.write_profile("umask 022\n");
        let ctrl = EnsureUmask::with_paths(ctx.profile_path(), ctx.bashrc_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("022"));
    }

    #[test]
    fn ensure_umask_apply_sets_umask_in_both_files() {
        let ctx = AuthContext::new();
        ctx.write_profile("umask 022\n");
        ctx.write_bashrc("umask 022\n");
        let ctrl = EnsureUmask::with_paths(ctx.profile_path(), ctx.bashrc_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        assert!(std::fs::read_to_string(ctx.profile_path()).unwrap().contains("umask 0077"));
        assert!(std::fs::read_to_string(ctx.bashrc_path()).unwrap().contains("umask 0077"));
    }

    #[test]
    fn ensure_shell_timeout_check_compliant_when_tmout_300() {
        let ctx = AuthContext::new();
        ctx.write_profile("TMOUT=300\n");
        let ctrl = EnsureShellTimeout::with_path(ctx.profile_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::Compliant);
        assert!(result.evidence.contains("TMOUT = 300"));
    }

    #[test]
    fn ensure_shell_timeout_check_non_compliant_when_tmout_600() {
        let ctx = AuthContext::new();
        ctx.write_profile("TMOUT=600\n");
        let ctrl = EnsureShellTimeout::with_path(ctx.profile_path());
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert!(result.evidence.contains("TMOUT = 600"));
    }

    #[test]
    fn ensure_shell_timeout_apply_writes_tmout() {
        let ctx = AuthContext::new();
        ctx.write_profile("TMOUT=600\n");
        let ctrl = EnsureShellTimeout::with_path(ctx.profile_path());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, ApplyStatus::Applied);
        let content = std::fs::read_to_string(ctx.profile_path()).unwrap();
        assert!(content.contains("TMOUT=300"));
    }

    #[test]
    fn ensure_pam_minlen_dry_run_skips_apply() {
        let ctx = AuthContext::new();
        ctx.write_pwquality("minlen = 8\n");
        let ctrl = EnsurePamMinlen::with_path(ctx.pwquality_path());
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }

    #[test]
    fn ensure_password_hashing_dry_run_skips_apply() {
        let ctx = AuthContext::new();
        ctx.write_login_defs("ENCRYPT_METHOD MD5\n");
        let ctrl = EnsurePasswordHashing::with_path(ctx.login_defs_path());
        let result = ctrl.apply(&test_ctx(true)).unwrap();
        assert_eq!(result.status, ApplyStatus::Skipped);
    }
}
