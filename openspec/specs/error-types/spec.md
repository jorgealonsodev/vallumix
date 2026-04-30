# error-types Specification

## Purpose

Define the application error hierarchy for `vallumix-core` using `thiserror`, with `ControlError` for control-level failures and `VallumixError` for application-level failures, both providing structured, typed error information.

## Requirements

### Requirement: ControlError Enum

The `ControlError` enum MUST be defined in `vallumix-core::error` with `thiserror::Error` derivation and the following variants: `NotApplicable(String, Distro)`, `BackupFailed { path: PathBuf, source: IoError }`, `PostCheckFailed { expected: String, actual: String }`, `Io(#[from] IoError)`. It MUST derive `Debug`.

#### Scenario: ControlError displays human-readable messages

- GIVEN `ControlError::NotApplicable("1.1.1.1".into(), Distro::Rocky9)`
- WHEN `.to_string()` is called
- THEN the message contains both the control ID and the distro name

#### Scenario: ControlError::BackupFailed chains IO errors

- GIVEN a file operation fails with `std::io::Error`
- WHEN `ControlError::BackupFailed { path, source }` is constructed
- THEN the `source` chain is accessible via `std::error::Error::source()`

#### Scenario: ControlError::Io converts from std::io::Error

- GIVEN a `std::io::Error` from a file operation
- WHEN `?` operator is used to convert it into `ControlError`
- THEN it is automatically converted via `From` impl

### Requirement: VallumixError Enum

The system MUST define a `VallumixError` enum in `vallumix-core::error` for application-level errors with variants: `UnsupportedDistro(String)`, `Privilege(String)`, `ProfileNotFound(PathBuf)`, `ReportGeneration(String)`, `Io(#[from] std::io::Error)`. It MUST derive `thiserror::Error` and `Debug`.

#### Scenario: UnsupportedDistro includes distro information

- GIVEN an unsupported distro `fedora/40` is detected
- WHEN `VallumixError::UnsupportedDistro("fedora/40".into())` is displayed
- THEN the error message includes "fedora/40"

#### Scenario: Privilege error for non-root execution

- GIVEN the binary is run without root privileges
- WHEN `VallumixError::Privilege("vallumix requires root".into())` is displayed
- THEN the error message clearly indicates root is required

#### Scenario: ProfileNotFound includes path

- GIVEN a profile file at `/etc/vallumix/profiles/web.toml` does not exist
- WHEN `VallumixError::ProfileNotFound(path)` is displayed
- THEN the error message includes the path that was not found

### Requirement: Error Conversion for CLI

`VallumixError` MUST implement `From<ControlError>` to allow propagation from control execution to the CLI layer. `anyhow::Error` MUST be used only in `main.rs` as the top-level error type.

#### Scenario: ControlError converts to VallumixError

- GIVEN a `ControlError::NotApplicable` returned from a control
- WHEN the error is propagated via `?` to a function returning `Result<_, VallumixError>`
- THEN the conversion succeeds and the error is wrapped appropriately
