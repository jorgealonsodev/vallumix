# Delta for profile-model

## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Profile is_applicable Method

The `Profile` struct MUST provide `is_applicable(&self, distro: &Distro) -> bool` that returns `true` if ALL controls in the resolved list are applicable to the given distro. It MUST resolve controls against the registry to check distro applicability. A profile with an empty control list MUST return `true` for any distro. If any control is not applicable to the distro, the method MUST return `false`.
(Previously: `is_applicable()` was a no-op that always returned `true` regardless of distro, since controls were not yet populated.)

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