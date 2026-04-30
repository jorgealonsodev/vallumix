# profile-model Specification

## Purpose

Define the `Profile` struct that replaces the former trait, enabling TOML deserialization, control list resolution against a registry, and profile integrity validation.

## Requirements

### Requirement: Profile Struct Definition

The `Profile` struct MUST be defined in `vallumix-core::profile` with fields `name: String`, `description: String`, `controls: Vec<String>`. It MUST derive `Debug, Clone, serde::Serialize, serde::Deserialize`. The `controls` field MUST use `serde(default)` to allow empty control lists in TOML.

#### Scenario: Profile deserializes from TOML

- GIVEN a TOML string `name = "web"\ndescription = "Web server"\ncontrols = ["1.1.1.1", "5.2.4"]`
- WHEN `toml::from_str::<Profile>(toml_str)` is called
- THEN `profile.name` is `"web"`, `profile.controls` is `["1.1.1.1", "5.2.4"]`

#### Scenario: Profile with empty controls list

- GIVEN a TOML string with `controls = []`
- WHEN deserialized into `Profile`
- THEN `profile.controls` is an empty `Vec` and deserialization succeeds

#### Scenario: Profile without controls field uses default

- GIVEN a TOML string with `name = "web"` and `description = "..."` but no `controls` field
- WHEN deserialized into `Profile`
- THEN `profile.controls` is an empty `Vec` via `serde(default)`

### Requirement: Profile from_file Method

The `Profile` struct MUST provide `from_file(path: impl AsRef<Path>) -> Result<Profile, VallumixError>` that reads a TOML file and deserializes it. It MUST return `VallumixError::ProfileNotFound` if the file does not exist.

#### Scenario: Valid TOML file loads correctly

- GIVEN a valid `web.toml` at `/etc/vallumix/profiles/web.toml`
- WHEN `Profile::from_file("/etc/vallumix/profiles/web.toml")` is called
- THEN it returns `Ok(Profile)` with the file's contents

#### Scenario: Missing file returns ProfileNotFound

- GIVEN no file at `/etc/vallumix/profiles/nonexistent.toml`
- WHEN `Profile::from_file("/etc/vallumix/profiles/nonexistent.toml")` is called
- THEN it returns `Err(VallumixError::ProfileNotFound(path))`

#### Scenario: Malformed TOML returns descriptive error

- GIVEN a file containing invalid TOML syntax
- WHEN `Profile::from_file(path)` is called
- THEN it returns an error describing the parse failure

### Requirement: Profile resolve_controls Method

The `Profile` struct MUST provide `resolve_controls(&self, registry: &ControlRegistry) -> Result<Vec<Box<dyn Control>>, VallumixError>` that maps each control ID string to a concrete `Control` implementation. It MUST return an error listing any IDs not found in the registry.

#### Scenario: All control IDs resolve

- GIVEN a profile with `controls = ["1.1.1.1", "5.2.4"]` and a registry containing both controls
- WHEN `profile.resolve_controls(&registry)` is called
- THEN it returns a `Vec<Box<dyn Control>>` with 2 entries in registry order

#### Scenario: Unknown control ID returns error

- GIVEN a profile with `controls = ["1.1.1.1", "9.9.9"]` and a registry without `"9.9.9"`
- WHEN `profile.resolve_controls(&registry)` is called
- THEN it returns an error indicating control `"9.9.9"` was not found

### Requirement: Profile is_applicable Method

The `Profile` struct MUST provide `is_applicable(&self, distro: &Distro) -> bool` that returns `true` if ALL controls in the resolved list are applicable to the given distro. It MUST resolve controls against the registry to check distro applicability. A profile with an empty control list MUST return `true` for any distro. If any control is not applicable to the distro, the method MUST return `false`.

#### Scenario: Profile is applicable when all controls apply

- GIVEN a profile with controls all applicable to `Distro::Debian12`
- WHEN `profile.is_applicable(&Distro::Debian12)` is called
- THEN it returns `true`

#### Scenario: Empty profile is universally applicable

- GIVEN a profile with `controls = []`
- WHEN `profile.is_applicable(&any_distro)` is called
- THEN it returns `true`

#### Scenario: Profile not applicable when control excludes distro

- GIVEN a profile containing a control with `applicable_distros() = [Rocky9]`
- WHEN `profile.is_applicable(&Distro::Debian12)` is called
- THEN it returns `false`

### Requirement: Profile Category Filtering

The `Profile` struct MUST provide a `controls_by_category(&self, registry: &ControlRegistry) -> HashMap<Category, Vec<String>>` method that groups the profile's control IDs by their `Category`. This enables UI and report grouping by CIS domain.

#### Scenario: Controls grouped by category

- GIVEN a profile with controls `["1.1.1.1", "5.2.4", "3.1.1"]`
- WHEN `profile.controls_by_category(&registry)` is called
- THEN it returns `HashMap` with categories `Filesystem: ["1.1.1.1"]`, `Ssh: ["5.2.4"]`, `Network: ["3.1.1"]`

### Requirement: Database Profile Control Population

The `database.toml` profile MUST contain control IDs relevant for database servers per PRD §5.3: restricted network access, strict filesystem mount options, kernel hardening for I/O workloads, SUID/SGID removal, SSH hardening, and audit controls.

#### Scenario: database.toml is parseable and non-empty

- GIVEN `profiles/database.toml`
- WHEN `Profile::from_file("profiles/database.toml")` is called
- THEN it returns `Ok(Profile)` with `controls.len() >= 20`

#### Scenario: database.toml controls resolve in registry

- GIVEN a registry with 60+ controls
- WHEN `database_profile.resolve_controls(&registry)` is called
- THEN all control IDs resolve without error

### Requirement: Bastion Profile Control Population

The `bastion.toml` profile MUST contain control IDs for aggressive hardening per PRD §5.3: only SSH exposed, key-based auth required, extensive auditd logging, strict PAM policies, and maximum service disable controls.

#### Scenario: bastion.toml is parseable and non-empty

- GIVEN `profiles/bastion.toml`
- WHEN `Profile::from_file("profiles/bastion.toml")` is called
- THEN it returns `Ok(Profile)` with `controls.len() >= 25`

#### Scenario: bastion.toml includes SSH and auth controls

- GIVEN the parsed bastion profile
- WHEN `profile.controls` is inspected
- THEN it contains CIS IDs for SSH hardening (5.2.x) and PAM/auth (5.3.x)
