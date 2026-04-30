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

The `Profile` struct MUST provide `is_applicable(&self, distro: &Distro) -> bool` that returns `true` if ALL controls in the resolved list are applicable to the given distro. A profile with an empty control list MUST return `true` for any distro.

#### Scenario: Profile is applicable when all controls apply

- GIVEN a profile with controls all applicable to `Distro::Debian12`
- WHEN `profile.is_applicable(&Distro::Debian12)` is called
- THEN it returns `true`

#### Scenario: Empty profile is universally applicable

- GIVEN a profile with `controls = []`
- WHEN `profile.is_applicable(&any_distro)` is called
- THEN it returns `true`