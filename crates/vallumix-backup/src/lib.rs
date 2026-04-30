#![allow(missing_docs)]

use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use vallumix_core::error::ControlError;

#[derive(Debug, Clone)]
pub struct BackupMeta {
    pub control_id: String,
    pub version: usize,
    pub timestamp: DateTime<Utc>,
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupSession {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub profile: String,
    pub control_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub profile: String,
    pub control_count: usize,
}

#[derive(Debug, Clone)]
pub struct IntegrityFailure {
    pub control_id: String,
    pub version: usize,
    pub path: PathBuf,
    pub reason: String,
}

#[derive(Debug, Clone, Default)]
pub struct BackupManager {
    backup_dir: PathBuf,
}

impl BackupManager {
    pub fn new(backup_dir: impl AsRef<Path>) -> Self {
        BackupManager {
            backup_dir: backup_dir.as_ref().to_path_buf(),
        }
    }

    fn session_path(&self, session_id: &str) -> PathBuf {
        self.backup_dir.join(session_id)
    }

    fn session_json_path(&self, session_id: &str) -> PathBuf {
        self.session_path(session_id).join("session.json")
    }

    fn control_backup_dir(&self, session_id: &str, control_id: &str) -> PathBuf {
        self.session_path(session_id).join(control_id)
    }

    pub fn create_backup(
        &self,
        session_id: &str,
        control_id: &str,
        original_path: impl AsRef<Path>,
    ) -> Result<BackupMeta, ControlError> {
        let original_path = original_path.as_ref();
        let filename = original_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let control_dir = self.control_backup_dir(session_id, control_id);
        let version = next_version(&control_dir)?;
        let version_dir = control_dir.join(format!("v{}", version));
        fs::create_dir_all(&version_dir)?;

        let backup_path = version_dir.join(&filename);
        fs::copy(original_path, &backup_path)?;

        let checksum = self.checksum(&backup_path)?;
        let checksum_path = backup_path.with_extension(format!("{}.sha256", filename));
        fs::write(&checksum_path, &checksum)?;

        let timestamp = Utc::now();
        let meta = BackupMeta {
            control_id: control_id.into(),
            version,
            timestamp,
            original_path: original_path.to_path_buf(),
            backup_path,
            checksum: Some(checksum),
        };

        // Update session metadata
        self.update_session(session_id, control_id, timestamp)?;

        Ok(meta)
    }

    fn update_session(
        &self,
        session_id: &str,
        control_id: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<(), ControlError> {
        let path = self.session_json_path(session_id);
        let mut session = if path.exists() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| BackupSession {
                id: session_id.into(),
                timestamp,
                profile: "default".into(),
                control_ids: vec![],
            })
        } else {
            BackupSession {
                id: session_id.into(),
                timestamp,
                profile: "default".into(),
                control_ids: vec![],
            }
        };
        if !session.control_ids.contains(&control_id.into()) {
            session.control_ids.push(control_id.into());
        }
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(&path, serde_json::to_string_pretty(&session)?)?;
        Ok(())
    }

    pub fn list(&self, session_id: &str) -> Result<Vec<BackupMeta>, ControlError> {
        let mut metas = Vec::new();
        let session_dir = self.session_path(session_id);
        if !session_dir.exists() {
            return Ok(metas);
        }

        for entry in fs::read_dir(&session_dir)? {
            let entry = entry?;
            let control_id = entry.file_name().to_string_lossy().to_string();
            if control_id == "session.json" {
                continue;
            }
            let control_dir = entry.path();
            if !control_dir.is_dir() {
                continue;
            }
            for version_entry in fs::read_dir(&control_dir)? {
                let version_entry = version_entry?;
                let name = version_entry.file_name().to_string_lossy().to_string();
                if !name.starts_with('v') {
                    continue;
                }
                let version: usize = name[1..].parse().unwrap_or(0);
                let version_dir = version_entry.path();
                for file_entry in fs::read_dir(&version_dir)? {
                    let file_entry = file_entry?;
                    let fname = file_entry.file_name().to_string_lossy().to_string();
                    if fname.ends_with(".sha256") {
                        continue;
                    }
                    let backup_path = file_entry.path();
                    let checksum_path = backup_path.with_extension(format!("{}.sha256", fname));
                    let checksum = if checksum_path.exists() {
                        Some(fs::read_to_string(&checksum_path)?.trim().into())
                    } else {
                        None
                    };
                    let original_path = backup_path.clone(); // best effort; real original stored elsewhere
                    metas.push(BackupMeta {
                        control_id: control_id.clone(),
                        version,
                        timestamp: fs::metadata(&backup_path)?.modified()?.into(),
                        original_path,
                        backup_path,
                        checksum,
                    });
                }
            }
        }

        metas.sort_by(|a, b| a.control_id.cmp(&b.control_id).then(a.version.cmp(&b.version)));
        Ok(metas)
    }

    pub fn list_all_sessions(&self) -> Result<Vec<SessionInfo>, ControlError> {
        let mut sessions = Vec::new();
        if !self.backup_dir.exists() {
            return Ok(sessions);
        }
        for entry in fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            if !entry.path().is_dir() {
                continue;
            }
            let session_id = entry.file_name().to_string_lossy().to_string();
            let session_path = entry.path();
            let json_path = session_path.join("session.json");
            if let Ok(content) = fs::read_to_string(&json_path) {
                if let Ok(session) = serde_json::from_str::<BackupSession>(&content) {
                    sessions.push(SessionInfo {
                        id: session_id,
                        timestamp: session.timestamp,
                        profile: session.profile,
                        control_count: session.control_ids.len(),
                    });
                    continue;
                }
            }
            // Fallback if session.json missing or corrupt
            sessions.push(SessionInfo {
                id: session_id,
                timestamp: entry.metadata()?.modified()?.into(),
                profile: "unknown".into(),
                control_count: 0,
            });
        }
        sessions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(sessions)
    }

    pub fn restore(&self, backup: &vallumix_core::profile::Backup) -> Result<(), ControlError> {
        fs::copy(&backup.backup_path, &backup.original_path)?;
        Ok(())
    }

    pub fn rollback_by_control(&self, control_id: &str) -> Result<(), ControlError> {
        let sessions = self.list_all_sessions()?;
        for session in sessions {
            let metas = self.list(&session.id)?;
            if let Some(meta) = metas
                .into_iter()
                .filter(|m| m.control_id == control_id)
                .max_by_key(|m| m.version)
            {
                fs::copy(&meta.backup_path, &meta.original_path)?;
                return Ok(());
            }
        }
        Err(ControlError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no backup found for control {}", control_id),
        )))
    }

    pub fn rollback_session(&self, session_id: &str) -> Result<usize, ControlError> {
        let metas = self.list(session_id)?;
        let mut restored = 0;
        // Take the latest version per control
        let mut seen = std::collections::HashSet::new();
        for meta in metas.into_iter().rev() {
            if seen.insert(meta.control_id.clone()) {
                fs::copy(&meta.backup_path, &meta.original_path)?;
                restored += 1;
            }
        }
        Ok(restored)
    }

    pub fn prune(&self, session_id: &str, keep: usize) -> Result<usize, ControlError> {
        let mut pruned = 0;
        let session_dir = self.session_path(session_id);
        if !session_dir.exists() {
            return Ok(0);
        }
        for entry in fs::read_dir(&session_dir)? {
            let entry = entry?;
            let control_id = entry.file_name().to_string_lossy().to_string();
            if control_id == "session.json" || !entry.path().is_dir() {
                continue;
            }
            let control_dir = entry.path();
            let mut versions: Vec<(usize, PathBuf)> = Vec::new();
            for ventry in fs::read_dir(&control_dir)? {
                let ventry = ventry?;
                let name = ventry.file_name().to_string_lossy().to_string();
                if !name.starts_with('v') {
                    continue;
                }
                let version: usize = name[1..].parse().unwrap_or(0);
                versions.push((version, ventry.path()));
            }
            versions.sort_by_key(|(v, _)| *v);
            if versions.len() > keep {
                for (_, path) in versions.iter().take(versions.len() - keep) {
                    fs::remove_dir_all(path)?;
                    pruned += 1;
                }
            }
        }
        Ok(pruned)
    }

    pub fn verify(&self, session_id: &str) -> Result<Vec<IntegrityFailure>, ControlError> {
        let mut failures = Vec::new();
        let metas = self.list(session_id)?;
        for meta in metas {
            if !meta.backup_path.exists() {
                failures.push(IntegrityFailure {
                    control_id: meta.control_id.clone(),
                    version: meta.version,
                    path: meta.backup_path.clone(),
                    reason: "backup file missing".into(),
                });
                continue;
            }
            let size = fs::metadata(&meta.backup_path)?.len();
            if size == 0 {
                failures.push(IntegrityFailure {
                    control_id: meta.control_id.clone(),
                    version: meta.version,
                    path: meta.backup_path.clone(),
                    reason: "backup file is empty".into(),
                });
                continue;
            }
            if let Some(expected) = meta.checksum {
                let actual = self.checksum(&meta.backup_path)?;
                if expected.trim() != actual.trim() {
                    failures.push(IntegrityFailure {
                        control_id: meta.control_id.clone(),
                        version: meta.version,
                        path: meta.backup_path.clone(),
                        reason: "checksum mismatch".into(),
                    });
                }
            }
        }
        Ok(failures)
    }

    pub fn checksum(&self, path: &Path) -> Result<String, ControlError> {
        let data = fs::read(path)?;
        let hash = Sha256::digest(&data);
        Ok(format!("{:x}", hash))
    }
}

fn next_version(control_dir: &Path) -> Result<usize, ControlError> {
    if !control_dir.exists() {
        return Ok(1);
    }
    let mut max = 0;
    for entry in fs::read_dir(control_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('v') {
            if let Ok(v) = name[1..].parse::<usize>() {
                if v > max {
                    max = v;
                }
            }
        }
    }
    Ok(max + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_backup_in_session_with_versioned_dirs() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "hello").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        let meta = mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();

        assert!(meta.backup_path.exists());
        assert_eq!(meta.version, 1);
        assert!(meta.checksum.is_some());

        // session.json should exist
        let session_json = mgr.session_json_path("sess1");
        assert!(session_json.exists());
        let session: BackupSession = serde_json::from_str(&fs::read_to_string(&session_json).unwrap()).unwrap();
        assert!(session.control_ids.contains(&"1.1.1.1".into()));
    }

    #[test]
    fn create_backup_increments_version() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "hello").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        let meta1 = mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();
        let meta2 = mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();

        assert_eq!(meta1.version, 1);
        assert_eq!(meta2.version, 2);
    }

    #[test]
    fn list_returns_backups_ordered() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "hello").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();
        mgr.create_backup("sess1", "2.2.3", &original).unwrap();

        let list = mgr.list("sess1").unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].control_id, "1.1.1.1");
        assert_eq!(list[1].control_id, "2.2.3");
    }

    #[test]
    fn list_all_sessions_newest_first() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "hello").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();
        mgr.create_backup("sess2", "2.2.3", &original).unwrap();

        let sessions = mgr.list_all_sessions().unwrap();
        assert_eq!(sessions.len(), 2);
        // sess2 was created after sess1
        assert_eq!(sessions[0].id, "sess2");
        assert_eq!(sessions[1].id, "sess1");
    }

    #[test]
    fn rollback_by_control_restores_latest() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "v1").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();

        fs::write(&original, "v2").unwrap();
        mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();

        fs::write(&original, "modified").unwrap();
        mgr.rollback_by_control("1.1.1.1").unwrap();

        assert_eq!(fs::read_to_string(&original).unwrap(), "v2");
    }

    #[test]
    fn rollback_session_restores_all_controls() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original1 = tmpdir.path().join("a.txt");
        let original2 = tmpdir.path().join("b.txt");
        fs::write(&original1, "a").unwrap();
        fs::write(&original2, "b").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        mgr.create_backup("sess1", "1.1.1.1", &original1).unwrap();
        mgr.create_backup("sess1", "2.2.3", &original2).unwrap();

        fs::write(&original1, "modified").unwrap();
        fs::write(&original2, "modified").unwrap();

        let count = mgr.rollback_session("sess1").unwrap();
        assert_eq!(count, 2);
        assert_eq!(fs::read_to_string(&original1).unwrap(), "a");
        assert_eq!(fs::read_to_string(&original2).unwrap(), "b");
    }

    #[test]
    fn prune_removes_oldest_versions() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "hello").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        for i in 1..=5 {
            fs::write(&original, format!("v{}", i)).unwrap();
            mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();
        }

        let pruned = mgr.prune("sess1", 2).unwrap();
        assert_eq!(pruned, 3);

        let list = mgr.list("sess1").unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].version, 4);
        assert_eq!(list[1].version, 5);
    }

    #[test]
    fn verify_detects_missing_file() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "hello").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();

        let mut list = mgr.list("sess1").unwrap();
        // Remove the backup file to simulate corruption
        fs::remove_file(&list[0].backup_path).unwrap();

        let failures = mgr.verify("sess1").unwrap();
        assert_eq!(failures.len(), 1);
        assert!(failures[0].reason.contains("missing"));
    }

    #[test]
    fn checksum_is_consistent() {
        let tmpdir = tempfile::tempdir().unwrap();
        let path = tmpdir.path().join("file.txt");
        fs::write(&path, "hello world").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        let sum1 = mgr.checksum(&path).unwrap();
        let sum2 = mgr.checksum(&path).unwrap();
        assert_eq!(sum1, sum2);
        assert_eq!(sum1.len(), 64);
    }

    #[test]
    fn checksum_sidecar_exists_after_backup() {
        let tmpdir = tempfile::tempdir().unwrap();
        let original = tmpdir.path().join("original.txt");
        fs::write(&original, "hello").unwrap();

        let mgr = BackupManager::new(tmpdir.path().join("backups"));
        let meta = mgr.create_backup("sess1", "1.1.1.1", &original).unwrap();

        let sidecar = meta.backup_path.with_extension("original.txt.sha256");
        assert!(sidecar.exists());
    }
}
