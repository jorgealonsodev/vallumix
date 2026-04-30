# Delta for core-traits

## ADDED Requirements

### Requirement: CheckStatus and ApplyStatus Enums

The system MUST define `CheckStatus` and `ApplyStatus` enums in `vallumix-core::control`. `CheckStatus` MUST have variants `Compliant`, `NonCompliant`, `Error`, `Skipped`. `ApplyStatus` MUST have variants `Applied`, `AlreadyCompliant`, `Failed`, `Skipped`. Both MUST derive `Debug, Clone, PartialEq, Eq`.

#### Scenario: CheckStatus enum compiles with all variants

- GIVEN `CheckStatus` is defined in `vallumix_core::control`
- WHEN a caller constructs `CheckStatus::Compliant`, `NonCompliant`, `Error`, `Skipped`
- THEN all variants compile without errors

#### Scenario: ApplyStatus enum compiles with all variants

- GIVEN `ApplyStatus` is defined in `vallumix_core::control`
- WHEN a caller constructs `ApplyStatus::Applied`, `AlreadyCompliant`, `Failed`, `Skipped`
- THEN all variants compile without errors

## MODIFIED Requirements

### Requirement: Associated Types for Control Trait

The `Control` trait's associated types MUST include `Severity`, `Distro`, `CheckResult`, `ApplyResult`, `ControlError`, `Context`, and `Backup` defined in `vallumix-core`. `CheckResult` MUST contain fields `status: CheckStatus`, `evidence: String`, `message: Option<String>`. `ApplyResult` MUST contain fields `status: ApplyStatus`, `backup_path: Option<PathBuf>`, `message: Option<String>`. Both structs MUST derive `Debug, Clone`.

(Previously: CheckResult and ApplyResult were empty structs with no fields)

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

(Previously: Reporter trait was an empty trait with no methods)

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

(Previously: Only trait and type re-exports were required, not status enums)

#### Scenario: Public API is accessible from dependent crates

- GIVEN `vallumix-controls` depends on `vallumix-core`
- WHEN `vallumix-controls` imports `use vallumix_core::Control`
- THEN the import resolves without errors

#### Scenario: Status types are accessible from crate root

- GIVEN `vallumix-core` re-exports status enums at crate root
- WHEN a dependent crate imports `use vallumix_core::CheckStatus`
- THEN the import resolves without errors

## REMOVED Requirements

### Requirement: Profile Trait Definition

(Reason: Profile is converted from a trait to a concrete struct. The new `Profile` struct with `name`, `description`, `controls` fields and `controls()`, `is_applicable()` methods is specified in the `profile-model` capability.)