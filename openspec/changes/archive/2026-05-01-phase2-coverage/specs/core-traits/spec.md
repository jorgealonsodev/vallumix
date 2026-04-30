# Delta for core-traits

## ADDED Requirements

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

#### Scenario: CheckStatus сериализуется

- GIVEN `CheckStatus::Warning(Some("test".into()))`
- WHEN serialized with `serde_json`
- THEN it produces valid JSON

### Requirement: ApplyStatus PartialApply Variant

`ApplyStatus` enum MUST add a `PartialApply` variant to represent controls where some changes were applied but others could not complete. The variant MUST carry an optional `String` reason.

#### Scenario: ApplyStatus PartialApply variant compiles

- GIVEN `ApplyStatus` is defined with `PartialApply`
- WHEN `ApplyStatus::PartialApply(Some("3 of 5 rules applied".into()))` is constructed
- THEN it compiles without errors

## MODIFIED Requirements

### Requirement: CheckStatus and ApplyStatus Enums

The system MUST define `CheckStatus` and `ApplyStatus` enums in `vallumix-core::control`. `CheckStatus` MUST have variants `Compliant`, `NonCompliant`, `Error`, `Skipped`, `Warning(Option<String>)`. `ApplyStatus` MUST have variants `Applied`, `AlreadyCompliant`, `Failed`, `Skipped`, `PartialApply(Option<String>)`. Both MUST derive `Debug, Clone, PartialEq, Eq, Serialize, Deserialize`.
(Previously: `CheckStatus` had variants `Compliant`, `NonCompliant`, `Error`, `Skipped`. `ApplyStatus` had variants `Applied`, `AlreadyCompliant`, `Failed`, `Skipped`. Both derived `Debug, Clone, PartialEq, Eq`.)

#### Scenario: CheckStatus enum compiles with all variants

- GIVEN `CheckStatus` is defined in `vallumix_core::control`
- WHEN a caller constructs `CheckStatus::Compliant`, `NonCompliant`, `Error`, `Skipped`, `Warning(Some("msg".into()))`
- THEN all variants compile without errors

#### Scenario: ApplyStatus enum compiles with all variants

- GIVEN `ApplyStatus` is defined in `vallumix_core::control`
- WHEN a caller constructs `ApplyStatus::Applied`, `AlreadyCompliant`, `Failed`, `Skipped`, `PartialApply(Some("msg".into()))`
- THEN all variants compile without errors