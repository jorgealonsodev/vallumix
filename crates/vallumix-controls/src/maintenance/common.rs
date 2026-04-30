#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub struct PermsContext {
    pub tmpdir: tempfile::TempDir,
}

#[cfg(test)]
impl PermsContext {
    pub fn new() -> Self {
        PermsContext {
            tmpdir: tempfile::tempdir().unwrap(),
        }
    }

    pub fn file_path(&self, name: &str) -> PathBuf {
        self.tmpdir.path().join(name)
    }

    pub fn create_file_with_perms(&self, name: &str, mode: u32) -> PathBuf {
        let path = self.file_path(name);
        std::fs::write(&path, "content\n").unwrap();
        let mut perms = std::fs::metadata(&path).unwrap().permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            perms.set_mode(mode);
        }
        std::fs::set_permissions(&path, perms).unwrap();
        path
    }

    pub fn cron_dir(&self) -> PathBuf {
        self.tmpdir.path().join("cron.d")
    }

    pub fn create_cron_dirs(&self, mode: u32) {
        for sub in ["cron.d", "cron.daily", "cron.weekly", "cron.monthly"] {
            let path = self.tmpdir.path().join(sub);
            std::fs::create_dir_all(&path).unwrap();
            let mut perms = std::fs::metadata(&path).unwrap().permissions();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                perms.set_mode(mode);
            }
            std::fs::set_permissions(&path, perms).unwrap();
        }
    }
}
