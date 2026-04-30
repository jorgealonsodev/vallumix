#![allow(missing_docs)]

use std::collections::HashMap;

pub mod control;
pub mod context;
pub mod distro;
pub mod error;
pub mod profile;

pub use control::{ApplyResult, ApplyStatus, Category, CheckResult, CheckStatus};

pub type ControlRegistry = HashMap<String, Box<dyn control::Control>>;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde::Deserialize;

    use crate::control::Control;
    use crate::distro::Distro;

    #[test]
    fn harness_smoke_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn control_trait_object_safety() {
        // If Box<dyn Control> compiles, the trait is object-safe.
        let _maybe: Option<Box<dyn Control>> = None;
    }

    #[test]
    fn severity_enum_exists() {
        use crate::control::Severity;
        let _s = Severity::High;
        let _s = Severity::Medium;
        let _s = Severity::Low;
    }

    #[test]
    fn distro_enum_exists() {
        let _d = Distro::Debian12;
        let _d = Distro::Ubuntu2204;
        let _d = Distro::Ubuntu2404;
        let _d = Distro::Rocky9;
    }

    fn workspace_root() -> PathBuf {
        // vallumix-core lives at crates/vallumix-core; workspace root is two levels up.
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn profile_web_toml_parses() {
        let path = workspace_root().join("profiles").join("web.toml");
        let content = std::fs::read_to_string(&path).expect("web.toml should exist");
        let profile: crate::profile::Profile = toml::from_str(&content).expect("web.toml should parse");
        assert_eq!(profile.name, "web");
        assert!(!profile.description.is_empty());
        assert!(!profile.controls.is_empty());
        assert!(profile.controls.len() >= 20);
    }

    #[test]
    fn profile_database_toml_parses() {
        let path = workspace_root().join("profiles").join("database.toml");
        let content = std::fs::read_to_string(&path).expect("database.toml should exist");
        let profile: crate::profile::Profile = toml::from_str(&content).expect("database.toml should parse");
        assert_eq!(profile.name, "database");
        assert!(!profile.description.is_empty());
        assert!(profile.controls.len() >= 20);
    }

    #[test]
    fn profile_bastion_toml_parses() {
        let path = workspace_root().join("profiles").join("bastion.toml");
        let content = std::fs::read_to_string(&path).expect("bastion.toml should exist");
        let profile: crate::profile::Profile = toml::from_str(&content).expect("bastion.toml should parse");
        assert_eq!(profile.name, "bastion");
        assert!(!profile.description.is_empty());
        assert!(profile.controls.len() >= 25);
    }
}
