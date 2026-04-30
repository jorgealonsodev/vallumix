#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub struct LoggingContext {
    pub tmpdir: tempfile::TempDir,
}

#[cfg(test)]
impl LoggingContext {
    pub fn new() -> Self {
        LoggingContext {
            tmpdir: tempfile::tempdir().unwrap(),
        }
    }

    pub fn rsyslog_conf_path(&self) -> PathBuf {
        self.tmpdir.path().join("rsyslog.conf")
    }

    pub fn journald_conf_path(&self) -> PathBuf {
        self.tmpdir.path().join("journald.conf")
    }

    pub fn journald_dropin_dir(&self) -> PathBuf {
        self.tmpdir.path().join("journald.conf.d")
    }

    pub fn auditd_conf_path(&self) -> PathBuf {
        self.tmpdir.path().join("auditd.conf")
    }

    pub fn audit_rules_path(&self) -> PathBuf {
        self.tmpdir.path().join("audit.rules")
    }

    pub fn logrotate_conf_path(&self) -> PathBuf {
        self.tmpdir.path().join("rsyslog")
    }

    pub fn log_dir(&self) -> PathBuf {
        self.tmpdir.path().join("log")
    }

    pub fn write_rsyslog(&self, content: &str) {
        std::fs::write(self.rsyslog_conf_path(), content).unwrap();
    }

    pub fn write_journald(&self, content: &str) {
        std::fs::write(self.journald_conf_path(), content).unwrap();
    }

    pub fn write_auditd(&self, content: &str) {
        std::fs::write(self.auditd_conf_path(), content).unwrap();
    }

    pub fn write_audit_rules(&self, content: &str) {
        let path = self.audit_rules_path();
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
    }

    pub fn write_logrotate(&self, content: &str) {
        std::fs::write(self.logrotate_conf_path(), content).unwrap();
    }

    pub fn create_log_file(&self, name: &str, mode: u32) {
        let path = self.log_dir().join(name);
        std::fs::create_dir_all(self.log_dir()).unwrap();
        std::fs::write(&path, "log content\n").unwrap();
        let mut perms = std::fs::metadata(&path).unwrap().permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            perms.set_mode(mode);
        }
        std::fs::set_permissions(&path, perms).unwrap();
    }
}
