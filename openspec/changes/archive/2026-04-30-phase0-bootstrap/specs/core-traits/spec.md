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

### Requirement: Associated Types for Control Trait

The `Control` trait's associated types MUST include `Severity`, `Distro`, `CheckResult`, `ApplyResult`, `ControlError`, `Context`, and `Backup` defined in `vallumix-core`.

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

### Requirement: Reporter Trait Definition

The `Reporter` trait MUST be defined in `vallumix-core` as a trait with `Send + Sync` bounds and a method for generating reports, enabling multiple output formats.

#### Scenario: Reporter trait compiles

- GIVEN `Reporter` trait is defined in `vallumix-core`
- WHEN `cargo check -p vallumix-core` is executed
- THEN the trait compiles and is usable as `Box<dyn Reporter>`

### Requirement: Profile Trait Definition

The `Profile` trait MUST be defined in `vallumix-core::profile` providing the contract for selecting which controls apply to a given server role.

#### Scenario: Profile trait compiles with control selection method

- GIVEN `Profile` trait is defined with a method to list applicable controls
- WHEN `cargo check -p vallumix-core` is executed
- THEN the trait compiles and can be implemented by concrete profile types

### Requirement: vallumix-core Public API

The `vallumix-core` crate's `lib.rs` MUST publicly export all trait and type definitions via module declarations for `control`, `error`, `context`, `distro`, and `profile`.

#### Scenario: Public API is accessible from dependent crates

- GIVEN `vallumix-controls` depends on `vallumix-core`
- WHEN `vallumix-controls` imports `use vallumix_core::Control`
- THEN the import resolves without errors

#### Scenario: Unused module declarations do not produce warnings

- GIVEN `lib.rs` declares `pub mod control;` and `control.rs` exists
- WHEN `cargo check -p vallumix-core` is executed
- THEN no unused-imports or dead-code warnings are emitted for public modules