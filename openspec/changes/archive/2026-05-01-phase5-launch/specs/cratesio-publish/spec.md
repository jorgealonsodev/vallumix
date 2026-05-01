# cratesio-publish Specification

## Purpose

Publish the vallumix-cli crate to crates.io automatically when a version tag is pushed, ensuring the crate is installable via `cargo install vallumix-cli` per PRD §6.1.

## Requirements

### Requirement: crates.io Publish on Version Tag

The release workflow MUST publish `vallumix-cli` to crates.io when a version tag matching `v*` is pushed. Authentication MUST use the `CARGO_REGISTRY_TOKEN` repository secret.

#### Scenario: Successful publish on tag push

- GIVEN a version tag v1.0.0 is pushed and all release artifacts succeed
- WHEN the crates.io publish step runs
- THEN `cargo publish -p vallumix-cli` executes and the crate appears on crates.io

#### Scenario: Publish fails on duplicate version

- GIVEN vallumix-cli v1.0.0 is already published on crates.io
- WHEN `cargo publish` runs for the same version
- THEN the step exits non-zero and does not block the remaining release workflow

### Requirement: Dry-Run Validation Before Publish

The release workflow MUST run `cargo publish --dry-run` for vallumix-cli before the actual publish step to catch metadata or packaging errors early.

#### Scenario: Dry-run catches missing metadata

- GIVEN vallumix-cli Cargo.toml is missing the `description` field
- WHEN `cargo publish --dry-run -p vallumix-cli` runs
- THEN the step reports the missing field and fails before actual publish

#### Scenario: Dry-run passes with valid metadata

- GIVEN all required metadata fields are present in vallumix-cli Cargo.toml
- WHEN `cargo publish --dry-run -p vallumix-cli` runs
- THEN the step exits with code 0 and actual publish proceeds

### Requirement: Workspace Crate Publish Exclusion

`vallumix-core`, `vallumix-controls`, `vallumix-reporters`, and `vallumix-backup` MUST be marked with `publish = false` in their respective `[package]` sections. Only `vallumix-cli` is published for v1.0.0.

#### Scenario: Internal crates are excluded from publishing

- GIVEN vallumix-core, vallumix-controls, vallumix-reporters, and vallumix-backup have `publish = false`
- WHEN `cargo publish -p vallumix-core` is attempted
- THEN cargo refuses to publish and reports the crate is marked non-publishable