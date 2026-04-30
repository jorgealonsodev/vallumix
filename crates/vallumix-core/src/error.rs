use std::io::Error as IoError;
use std::path::PathBuf;

use crate::distro::Distro;

#[derive(thiserror::Error, Debug)]
pub enum ControlError {
    #[error("control {0} not applicable to distribution {1}")]
    NotApplicable(String, Distro),

    #[error("backup failed for {path}: {source}")]
    BackupFailed { path: PathBuf, source: IoError },

    #[error("post-check failed: expected {expected}, got {actual}")]
    PostCheckFailed { expected: String, actual: String },

    #[error(transparent)]
    Io(#[from] IoError),
}

#[derive(thiserror::Error, Debug)]
pub enum VallumixError {
    #[error("unsupported distribution: {0}")]
    UnsupportedDistro(String),

    #[error("privilege error: {0}")]
    Privilege(String),

    #[error("profile not found: {0}")]
    ProfileNotFound(PathBuf),

    #[error("report generation failed: {0}")]
    ReportGeneration(String),

    #[error(transparent)]
    Io(#[from] IoError),
}

impl From<ControlError> for VallumixError {
    fn from(err: ControlError) -> Self {
        match err {
            ControlError::Io(e) => VallumixError::Io(e),
            other => VallumixError::ReportGeneration(other.to_string()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReportError {
    #[error("serialization failed: {0}")]
    Serialize(String),
}
