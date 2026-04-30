#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub struct AuthContext {
    pub tmpdir: tempfile::TempDir,
}

#[cfg(test)]
impl AuthContext {
    pub fn new() -> Self {
        AuthContext {
            tmpdir: tempfile::tempdir().unwrap(),
        }
    }

    pub fn pam_path(&self) -> PathBuf {
        self.tmpdir.path().join("pam.conf")
    }

    pub fn pwquality_path(&self) -> PathBuf {
        self.tmpdir.path().join("pwquality.conf")
    }

    pub fn login_defs_path(&self) -> PathBuf {
        self.tmpdir.path().join("login.defs")
    }

    pub fn profile_path(&self) -> PathBuf {
        self.tmpdir.path().join("profile")
    }

    pub fn bashrc_path(&self) -> PathBuf {
        self.tmpdir.path().join("bash.bashrc")
    }

    pub fn write_pam(&self, content: &str) {
        std::fs::write(self.pam_path(), content).unwrap();
    }

    pub fn write_pwquality(&self, content: &str) {
        std::fs::write(self.pwquality_path(), content).unwrap();
    }

    pub fn write_login_defs(&self, content: &str) {
        std::fs::write(self.login_defs_path(), content).unwrap();
    }

    pub fn write_profile(&self, content: &str) {
        std::fs::write(self.profile_path(), content).unwrap();
    }

    pub fn write_bashrc(&self, content: &str) {
        std::fs::write(self.bashrc_path(), content).unwrap();
    }
}
