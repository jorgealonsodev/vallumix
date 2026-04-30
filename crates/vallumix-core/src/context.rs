use std::path::PathBuf;

use crate::distro::Distro;
use crate::error::VallumixError;

#[derive(Debug, Clone)]
pub struct Context {
    pub hostname: String,
    pub distro: Distro,
    pub work_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub profile_dir: PathBuf,
    pub dry_run: bool,
}

impl Context {
    pub fn new(distro: Distro) -> Result<Context, VallumixError> {
        let hostname = match nix::unistd::gethostname() {
            Ok(h) => h.to_string_lossy().to_string(),
            Err(e) => {
                tracing::warn!("failed to get hostname: {}, falling back to localhost", e);
                "localhost".to_string()
            }
        };

        let work_dir = std::env::var("VALLUMIX_WORK_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/var/lib/vallumix"));
        let backup_dir = std::env::var("VALLUMIX_BACKUP_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/var/backups/vallumix"));
        let profile_dir = std::env::var("VALLUMIX_PROFILE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/etc/vallumix/profiles"));

        Ok(Context {
            hostname,
            distro,
            work_dir,
            backup_dir,
            profile_dir,
            dry_run: false,
        })
    }

    pub fn with_paths(
        hostname: String,
        distro: Distro,
        work_dir: PathBuf,
        backup_dir: PathBuf,
        profile_dir: PathBuf,
        dry_run: bool,
    ) -> Context {
        Context {
            hostname,
            distro,
            work_dir,
            backup_dir,
            profile_dir,
            dry_run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_with_paths() {
        let ctx = Context::with_paths(
            "testhost".into(),
            Distro::Debian12,
            "/work".into(),
            "/backup".into(),
            "/profiles".into(),
            true,
        );
        assert_eq!(ctx.hostname, "testhost");
        assert_eq!(ctx.distro, Distro::Debian12);
        assert_eq!(ctx.work_dir, PathBuf::from("/work"));
        assert_eq!(ctx.backup_dir, PathBuf::from("/backup"));
        assert_eq!(ctx.profile_dir, PathBuf::from("/profiles"));
        assert!(ctx.dry_run);
    }

    #[test]
    fn context_new_uses_env_vars() {
        std::env::set_var("VALLUMIX_WORK_DIR", "/custom/work");
        std::env::set_var("VALLUMIX_BACKUP_DIR", "/custom/backup");
        std::env::set_var("VALLUMIX_PROFILE_DIR", "/custom/profiles");

        let ctx = Context::new(Distro::Ubuntu2204).expect("context should build");
        assert_eq!(ctx.work_dir, PathBuf::from("/custom/work"));
        assert_eq!(ctx.backup_dir, PathBuf::from("/custom/backup"));
        assert_eq!(ctx.profile_dir, PathBuf::from("/custom/profiles"));

        std::env::remove_var("VALLUMIX_WORK_DIR");
        std::env::remove_var("VALLUMIX_BACKUP_DIR");
        std::env::remove_var("VALLUMIX_PROFILE_DIR");
    }
}
