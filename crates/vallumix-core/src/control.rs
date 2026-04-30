use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::context::Context;
use crate::distro::Distro;
use crate::error::ControlError;
use crate::profile::Backup;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Filesystem,
    Services,
    Network,
    Logging,
    Ssh,
    Auth,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    Compliant,
    NonCompliant,
    Skipped,
    Error,
    Warning(Option<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplyStatus {
    Applied,
    AlreadyCompliant,
    Failed,
    Skipped,
    PartialApply(Option<String>),
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub status: CheckStatus,
    pub evidence: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApplyResult {
    pub status: ApplyStatus,
    pub backup_path: Option<PathBuf>,
    pub message: Option<String>,
}

pub type ControlResult = Result<CheckResult, ControlError>;

pub trait Control: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn severity(&self) -> Severity;
    fn applicable_distros(&self) -> &[Distro];

    fn category(&self) -> Category {
        Category::Filesystem
    }

    fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError>;
    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError>;
    fn rollback(&self, ctx: &Context, backup: &Backup) -> Result<(), ControlError>;

    fn clone_box(&self) -> Box<dyn Control>;
}

impl Clone for Box<dyn Control> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_enum_variants_compile() {
        let _ = Category::Filesystem;
        let _ = Category::Services;
        let _ = Category::Network;
        let _ = Category::Logging;
        let _ = Category::Ssh;
        let _ = Category::Auth;
        let _ = Category::Maintenance;
    }

    #[test]
    fn category_default_impl_returns_filesystem() {
        struct DummyControl;
        impl Control for DummyControl {
            fn id(&self) -> &str { "dummy" }
            fn description(&self) -> &str { "dummy" }
            fn severity(&self) -> Severity { Severity::Low }
            fn applicable_distros(&self) -> &[Distro] { &[] }
            fn check(&self, _ctx: &Context) -> ControlResult {
                Ok(CheckResult { status: CheckStatus::Compliant, evidence: "".into(), message: None })
            }
            fn apply(&self, _ctx: &Context) -> Result<ApplyResult, ControlError> {
                Ok(ApplyResult { status: ApplyStatus::Applied, backup_path: None, message: None })
            }
            fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
            fn clone_box(&self) -> Box<dyn Control> { Box::new(DummyControl) }
        }
        let ctrl = DummyControl;
        assert_eq!(ctrl.category(), Category::Filesystem);
    }

    #[test]
    fn check_status_warning_variant_exists() {
        let w = CheckStatus::Warning(Some("disk full".into()));
        assert_eq!(w, CheckStatus::Warning(Some("disk full".into())));
    }

    #[test]
    fn apply_status_partial_apply_variant_exists() {
        let p = ApplyStatus::PartialApply(Some("needs reboot".into()));
        assert_eq!(p, ApplyStatus::PartialApply(Some("needs reboot".into())));
    }

    #[test]
    fn check_status_serializes() {
        let status = CheckStatus::Warning(Some("test".into()));
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("Warning"));
    }

    #[test]
    fn apply_status_serializes() {
        let status = ApplyStatus::PartialApply(Some("test".into()));
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("PartialApply"));
    }

    #[test]
    fn check_status_variants_compile() {
        let _ = CheckStatus::Compliant;
        let _ = CheckStatus::NonCompliant;
        let _ = CheckStatus::Skipped;
        let _ = CheckStatus::Error;
        let _ = CheckStatus::Warning(None);
    }

    #[test]
    fn apply_status_variants_compile() {
        let _ = ApplyStatus::Applied;
        let _ = ApplyStatus::AlreadyCompliant;
        let _ = ApplyStatus::Failed;
        let _ = ApplyStatus::Skipped;
        let _ = ApplyStatus::PartialApply(None);
    }

    #[test]
    fn check_result_has_expected_fields() {
        let result = CheckResult {
            status: CheckStatus::NonCompliant,
            evidence: "module loaded".into(),
            message: Some("cramfs is available".into()),
        };
        assert_eq!(result.status, CheckStatus::NonCompliant);
        assert_eq!(result.evidence, "module loaded");
        assert_eq!(result.message, Some("cramfs is available".into()));
    }

    #[test]
    fn apply_result_has_expected_fields() {
        let path: PathBuf = "/tmp/backup".into();
        let result = ApplyResult {
            status: ApplyStatus::Applied,
            backup_path: Some(path.clone()),
            message: Some("disabled cramfs".into()),
        };
        assert_eq!(result.status, ApplyStatus::Applied);
        assert_eq!(result.backup_path, Some(path));
        assert_eq!(result.message, Some("disabled cramfs".into()));
    }

    #[test]
    fn control_result_type_alias_compiles() {
        let _ok: ControlResult = Ok(CheckResult {
            status: CheckStatus::Compliant,
            evidence: "ok".into(),
            message: None,
        });
        let _err: ControlResult = Err(ControlError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test",
        )));
    }
}
