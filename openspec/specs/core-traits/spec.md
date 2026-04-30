# core-traits Specification

## Purpose

Define the foundational traits and associated types in `vallumix-core` that establish the architectural contract for all controls, reporters, and profiles.

## Requirements

### Requirement: Control Trait Definition

The `Control` trait MUST be defined in `vallumix-core::control` with the exact signature from PRD §5.2: `id()`, `description()`, `severity()`, `applicable_distros()`, `check()`, `apply()`, `rollback()`, and the trait MUST be `Send + Sync`.

#### Scenario: Control trait compiles with correct signature

- GIVEN `vallumix-core` crate with `control.rs` defining the `Control` trait
- WHEN `cargo check -p vallumix-core` is executed
- THEN the trait compiles with methods `id() -> &str`, `description() -> &str`, `severity() -> Severity`, `applicable_distros() -> &[Distro]`, `check(&Context) -> Result<CheckResult, ControlError>`, `apply(&Context) -> Result<ApplyResult, ControlError>`, `rollback(&Context, &Backup) -> Result<(), ControlError>`

#### Scenario: Control trait object safety

- GIVEN the `Control` trait is `Send + Sync`
- WHEN `Box<dyn Control>` is used in downstream code
- THEN the trait object MUST compile without object-safety errors

### Requirement: CheckStatus and ApplyStatus Enums

The system MUST define `CheckStatus` and `ApplyStatus` enums in `vallumix-core::control`. `CheckStatus` MUST have variants `Compliant`, `NonCompliant`, `Error`, `Skipped`, `Warning(Option<String>)`. `ApplyStatus` MUST have variants `Applied`, `AlreadyCompliant`, `Failed`, `Skipped`, `PartialApply(Option<String>)`. Both MUST derive `Debug, Clone, PartialEq, Eq, Serialize, Deserialize`.

#### Scenario: CheckStatus enum compiles with all variants

- GIVEN `CheckStatus` is defined in `vallumix_core::control`
- WHEN a caller constructs `CheckStatus::Compliant`, `NonCompliant`, `Error`, `Skipped`, `Warning(Some("msg".into()))`
- THEN all variants compile without errors

#### Scenario: ApplyStatus enum compiles with all variants

- GIVEN `ApplyStatus` is defined in `vallumix_core::control`
- WHEN a caller constructs `ApplyStatus::Applied`, `AlreadyCompliant`, `Failed`, `Skipped`, `PartialApply(Some("msg".into()))`
- THEN all variants compile without errors

#### Scenario: CheckStatus serializes to JSON

- GIVEN `CheckStatus::Warning(Some("test".into()))`
- WHEN serialized with `serde_json`
- THEN it produces valid JSON

### Requirement: Associated Types for Control Trait

The `Control` trait's associated types MUST include `Severity`, `Distro`, `CheckResult`, `ApplyResult`, `ControlError`, `Context`, and `Backup` defined in `vallumix-core`. `CheckResult` MUST contain fields `status: CheckStatus`, `evidence: String`, `message: Option<String>`. `ApplyResult` MUST contain fields `status: ApplyStatus`, `backup_path: Option<PathBuf>`, `message: Option<String>`. Both structs MUST derive `Debug, Clone`.

#### Scenario: Severity enum compiles with expected variants

- GIVEN `Severity` is defined as an enum
- WHEN `cargo check -p vallumix-core` is executed
- THEN `Severity` MUST include at minimum variants for `Low`, `Medium`, and `High`

#### Scenario: Distro enum compiles with supported distributions

- GIVEN `Distro` is defined as an enum
- WHEN `cargo check -p vallumix-core` is executed
- THEN `Distro` MUST include variants for `Debian12`, `Ubuntu2204`, `Ubuntu2404`, and `Rocky9`

#### Scenario: ControlError type supports thiserror derivation

- GIVEN `ControlError` is defined using `thiserror`
- WHEN `cargo check -p vallumix-core` is executed
- THEN `ControlError` MUST derive `thiserror::Error` and `Debug`, and implement `From<std::io::Error>`

#### Scenario: CheckResult carries status and evidence

- GIVEN `CheckResult` has fields `status`, `evidence`, `message`
- WHEN `CheckResult::non_compliant("module loaded", None)` is called
- THEN `status` is `NonCompliant`, `evidence` is `"module loaded"`, `message` is `None`

#### Scenario: ApplyResult carries status and backup path

- GIVEN `ApplyResult` has fields `status`, `backup_path`, `message`
- WHEN a control successfully applies a change with backup path `/var/backups/vallumix/...`
- THEN `status` is `Applied`, `backup_path` is `Some(PathBuf)`, `message` is `None`

### Requirement: Reporter Trait Definition

The `Reporter` trait MUST be defined in `vallumix-core` as a trait with `Send + Sync` bounds and a method `generate(&self, results: &[ControlResult], ctx: &Context) -> Result<String, ReportError>`, enabling multiple output formats.

#### Scenario: Reporter trait compiles with generate method

- GIVEN `Reporter` trait is defined with `fn generate(&self, results: &[ControlResult], ctx: &Context) -> Result<String, ReportError>`
- WHEN `cargo check -p vallumix-core` is executed
- THEN the trait compiles and is usable as `Box<dyn Reporter>`

#### Scenario: Reporter generate method is callable

- GIVEN a type implements `Reporter`
- WHEN `reporter.generate(&results, &ctx)` is called
- THEN it returns `Ok(String)` containing the formatted report or `Err(ReportError)`

### Requirement: vallumix-core Public API

The `vallumix-core` crate's `lib.rs` MUST publicly export all trait and type definitions via module declarations for `control`, `error`, `context`, `distro`, and `profile`. It MUST also re-export `CheckStatus`, `ApplyStatus`, `CheckResult`, and `ApplyResult` at the crate root.

#### Scenario: Public API is accessible from dependent crates

- GIVEN `vallumix-controls` depends on `vallumix-core`
- WHEN `vallumix-controls` imports `use vallumix_core::Control`
- THEN the import resolves without errors

#### Scenario: Status types are accessible from crate root

- GIVEN `vallumix-core` re-exports status enums at crate root
- WHEN a dependent crate imports `use vallumix_core::CheckStatus`
- THEN the import resolves without errors

### Requirement: Control Trait Category Method

The `Control` trait MUST include a `category(&self) -> Category` method that returns the CIS domain category for the control. `Category` MUST be an enum with variants: `Filesystem`, `Services`, `Network`, `Logging`, `Ssh`, `Auth`, `Maintenance`. The `Category` enum MUST derive `Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize`.

#### Scenario: Control trait includes category method

- GIVEN the `Control` trait in `vallumix_core::control`
- WHEN `cargo check -p vallumix-core` is executed
- THEN the trait compiles with the method `fn category(&self) -> Category`

#### Scenario: Category enum provides all CIS domains

- GIVEN `Category` is defined in `vallumix_core::control`
- WHEN a caller constructs `Category::Filesystem`, `Category::Services`, `Category::Network`, `Category::Logging`, `Category::Ssh`, `Category::Auth`, `Category::Maintenance`
- THEN all variants compile without errors

#### Scenario: Category is serializable

- GIVEN a `Category` value
- WHEN serialized with `serde_json`
- THEN it produces the expected string (e.g., `Category::Ssh` → `"Ssh"`)

### Requirement: CheckStatus Warning Variant

`CheckStatus` enum MUST add a `Warning` variant to represent controls that are partially compliant. The `Warning` variant MUST carry an optional `String` message. Both `CheckStatus` and `ApplyStatus` MUST derive `Serialize, Deserialize`.

#### Scenario: CheckStatus Warning variant compiles

- GIVEN `CheckStatus` is defined with the `Warning` variant
- WHEN `CheckStatus::Warning(Some("unauthorized SUID found".into()))` is constructed
- THEN it compiles without errors

#### Scenario: CheckStatus Warning serializes

- GIVEN `CheckStatus::Warning(Some("test".into()))`
- WHEN serialized with `serde_json`
- THEN it produces valid JSON

### Requirement: ApplyStatus PartialApply Variant

`ApplyStatus` enum MUST add a `PartialApply` variant to represent controls where some changes were applied but others could not complete. The variant MUST carry an optional `String` reason.

#### Scenario: ApplyStatus PartialApply variant compiles

- GIVEN `ApplyStatus` is defined with `PartialApply`
- WHEN `ApplyStatus::PartialApply(Some("3 of 5 rules applied".into()))` is constructed
- THEN it compiles without errors
