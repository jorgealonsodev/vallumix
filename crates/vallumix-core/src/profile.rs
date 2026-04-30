use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::control::Control;
use crate::distro::Distro;
use crate::error::{ReportError, VallumixError};

#[derive(Debug, Clone)]
pub struct Backup {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub original_path: PathBuf,
    pub backup_path: PathBuf,
}

pub trait Reporter: Send + Sync {
    fn generate(&self, report: &Report) -> Result<String, ReportError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub host: HostInfo,
    pub summary: Summary,
    pub controls: Vec<ControlReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostInfo {
    pub hostname: String,
    pub distro: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub total: usize,
    pub pass: usize,
    pub fail: usize,
    pub skip: usize,
    pub compliance_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlReport {
    pub id: String,
    pub description: String,
    pub severity: String,
    pub status: String,
    pub evidence: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub controls: Vec<String>,
}

impl Profile {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Profile, VallumixError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                VallumixError::ProfileNotFound(path.to_path_buf())
            } else {
                VallumixError::Io(e)
            }
        })?;
        let profile: Profile = toml::from_str(&content)
            .map_err(|e| VallumixError::ReportGeneration(e.to_string()))?;
        Ok(profile)
    }

    pub fn resolve_controls(
        &self,
        registry: &HashMap<String, Box<dyn Control>>,
    ) -> Result<Vec<Box<dyn Control>>, VallumixError> {
        let mut resolved = Vec::new();
        for id in &self.controls {
            match registry.get(id) {
                Some(ctrl) => resolved.push(ctrl.clone_box()),
                None => {
                    return Err(VallumixError::ReportGeneration(format!(
                        "control '{}' not found in registry",
                        id
                    )))
                }
            }
        }
        Ok(resolved)
    }

    pub fn controls_by_category(
        &self,
        registry: &std::collections::HashMap<String, Box<dyn Control>>,
    ) -> std::collections::HashMap<crate::control::Category, Vec<String>> {
        let mut map: std::collections::HashMap<crate::control::Category, Vec<String>> =
            std::collections::HashMap::new();
        for id in &self.controls {
            if let Some(ctrl) = registry.get(id) {
                map.entry(ctrl.category()).or_default().push(id.clone());
            }
        }
        map
    }

    pub fn is_applicable(
        &self,
        distro: &Distro,
        registry: &std::collections::HashMap<String, Box<dyn Control>>,
    ) -> bool {
        if self.controls.is_empty() {
            return true;
        }
        for id in &self.controls {
            if let Some(ctrl) = registry.get(id) {
                if !ctrl.applicable_distros().contains(distro) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backup_struct_fields() {
        let backup = Backup {
            id: "test".into(),
            timestamp: Utc::now(),
            original_path: PathBuf::from("/etc/test"),
            backup_path: PathBuf::from("/tmp/backup"),
        };
        assert_eq!(backup.id, "test");
    }

    #[test]
    fn profile_deserializes_empty_controls() {
        let toml_str = r#"
name = "test"
description = "test profile"
"#;
        let profile: Profile = toml::from_str(toml_str).expect("should parse");
        assert_eq!(profile.name, "test");
        assert_eq!(profile.description, "test profile");
        assert!(profile.controls.is_empty());
    }

    #[test]
    fn profile_deserializes_with_controls() {
        let toml_str = r#"
name = "web"
description = "web profile"
controls = ["1.1.1.1", "5.2.4"]
"#;
        let profile: Profile = toml::from_str(toml_str).expect("should parse");
        assert_eq!(profile.controls.len(), 2);
    }

    #[test]
    fn report_struct_fields() {
        let report = Report {
            host: HostInfo {
                hostname: "testhost".into(),
                distro: "Debian12".into(),
            },
            summary: Summary {
                total: 5,
                pass: 3,
                fail: 1,
                skip: 1,
                compliance_rate: 60.0,
            },
            controls: vec![ControlReport {
                id: "1.1.1.1".into(),
                description: "Disable cramfs".into(),
                severity: "Low".into(),
                status: "Compliant".into(),
                evidence: "not present".into(),
                message: None,
            }],
        };
        assert_eq!(report.host.hostname, "testhost");
        assert_eq!(report.summary.total, 5);
        assert_eq!(report.controls.len(), 1);
    }

    #[test]
    fn controls_by_category_groups_correctly() {
        use crate::control::{Category, Control};
        use crate::context::Context;
        use crate::distro::Distro;
        use crate::error::ControlError;
        use crate::profile::Backup;

        struct FakeControl { id: &'static str, cat: Category }
        impl Control for FakeControl {
            fn id(&self) -> &str { self.id }
            fn description(&self) -> &str { "fake" }
            fn severity(&self) -> crate::control::Severity { crate::control::Severity::Low }
            fn applicable_distros(&self) -> &[Distro] { &[] }
            fn category(&self) -> Category { self.cat }
            fn check(&self, _ctx: &Context) -> Result<crate::control::CheckResult, ControlError> {
                Ok(crate::control::CheckResult { status: crate::control::CheckStatus::Compliant, evidence: "".into(), message: None })
            }
            fn apply(&self, _ctx: &Context) -> Result<crate::control::ApplyResult, ControlError> {
                Ok(crate::control::ApplyResult { status: crate::control::ApplyStatus::Applied, backup_path: None, message: None })
            }
            fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
            fn clone_box(&self) -> Box<dyn Control> { Box::new(FakeControl { id: self.id, cat: self.cat }) }
        }

        let mut registry = std::collections::HashMap::new();
        registry.insert("1.1.1.1".into(), Box::new(FakeControl { id: "1.1.1.1", cat: Category::Filesystem }) as Box<dyn Control>);
        registry.insert("5.2.4".into(), Box::new(FakeControl { id: "5.2.4", cat: Category::Ssh }) as Box<dyn Control>);
        registry.insert("3.1.1".into(), Box::new(FakeControl { id: "3.1.1", cat: Category::Network }) as Box<dyn Control>);

        let profile = Profile {
            name: "test".into(),
            description: "test".into(),
            controls: vec!["1.1.1.1".into(), "5.2.4".into(), "3.1.1".into()],
        };

        let grouped = profile.controls_by_category(&registry);
        assert_eq!(grouped.get(&Category::Filesystem).unwrap().len(), 1);
        assert_eq!(grouped.get(&Category::Ssh).unwrap().len(), 1);
        assert_eq!(grouped.get(&Category::Network).unwrap().len(), 1);
        assert_eq!(grouped.len(), 3);
    }

    #[test]
    fn is_applicable_empty_profile_returns_true() {
        let profile = Profile {
            name: "empty".into(),
            description: "empty".into(),
            controls: vec![],
        };
        let registry = std::collections::HashMap::new();
        assert!(profile.is_applicable(&Distro::Debian12, &registry));
    }

    #[test]
    fn is_applicable_when_all_controls_match_distro() {
        use crate::control::{Category, Control};
        use crate::context::Context;
        use crate::distro::Distro;
        use crate::error::ControlError;
        use crate::profile::Backup;

        struct DebianOnly;
        impl Control for DebianOnly {
            fn id(&self) -> &str { "d" }
            fn description(&self) -> &str { "d" }
            fn severity(&self) -> crate::control::Severity { crate::control::Severity::Low }
            fn applicable_distros(&self) -> &[Distro] { &[Distro::Debian12] }
            fn category(&self) -> Category { Category::Filesystem }
            fn check(&self, _ctx: &Context) -> Result<crate::control::CheckResult, ControlError> {
                Ok(crate::control::CheckResult { status: crate::control::CheckStatus::Compliant, evidence: "".into(), message: None })
            }
            fn apply(&self, _ctx: &Context) -> Result<crate::control::ApplyResult, ControlError> {
                Ok(crate::control::ApplyResult { status: crate::control::ApplyStatus::Applied, backup_path: None, message: None })
            }
            fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
            fn clone_box(&self) -> Box<dyn Control> { Box::new(DebianOnly) }
        }

        let mut registry = std::collections::HashMap::new();
        registry.insert("d".into(), Box::new(DebianOnly) as Box<dyn Control>);

        let profile = Profile {
            name: "test".into(),
            description: "test".into(),
            controls: vec!["d".into()],
        };
        assert!(profile.is_applicable(&Distro::Debian12, &registry));
    }

    #[test]
    fn is_applicable_when_control_does_not_match_distro() {
        use crate::control::{Category, Control};
        use crate::context::Context;
        use crate::distro::Distro;
        use crate::error::ControlError;
        use crate::profile::Backup;

        struct DebianOnly;
        impl Control for DebianOnly {
            fn id(&self) -> &str { "d" }
            fn description(&self) -> &str { "d" }
            fn severity(&self) -> crate::control::Severity { crate::control::Severity::Low }
            fn applicable_distros(&self) -> &[Distro] { &[Distro::Debian12] }
            fn category(&self) -> Category { Category::Filesystem }
            fn check(&self, _ctx: &Context) -> Result<crate::control::CheckResult, ControlError> {
                Ok(crate::control::CheckResult { status: crate::control::CheckStatus::Compliant, evidence: "".into(), message: None })
            }
            fn apply(&self, _ctx: &Context) -> Result<crate::control::ApplyResult, ControlError> {
                Ok(crate::control::ApplyResult { status: crate::control::ApplyStatus::Applied, backup_path: None, message: None })
            }
            fn rollback(&self, _ctx: &Context, _backup: &Backup) -> Result<(), ControlError> { Ok(()) }
            fn clone_box(&self) -> Box<dyn Control> { Box::new(DebianOnly) }
        }

        let mut registry = std::collections::HashMap::new();
        registry.insert("d".into(), Box::new(DebianOnly) as Box<dyn Control>);

        let profile = Profile {
            name: "test".into(),
            description: "test".into(),
            controls: vec!["d".into()],
        };
        assert!(!profile.is_applicable(&Distro::Rocky9, &registry));
    }
}
