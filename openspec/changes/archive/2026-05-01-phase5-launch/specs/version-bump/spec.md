# version-bump Specification

## Purpose

Bump the workspace version from 0.0.1 to the target release version and ensure consistency between the Cargo.toml version, git tag, and changelog for the v1.0.0 GA release.

## Requirements

### Requirement: Workspace Version Update

The `version` field in `[workspace.package]` MUST be updated from `0.0.1` to the target release version. All 5 member crates MUST inherit this version via `version.workspace = true`.

#### Scenario: All crates reflect the new version

- GIVEN `[workspace.package].version` is set to `1.0.0`
- WHEN `cargo metadata` is queried for crate versions
- THEN all 5 crates report version `1.0.0`

#### Scenario: Member crate with hardcoded version is detected

- GIVEN a member crate has `version = "0.0.1"` instead of `version.workspace = true`
- WHEN `cargo check` runs
- THEN cargo reports no version mismatch because the hardcoded version still exists

### Requirement: Changelog Version Section

`CHANGELOG.md` MUST have a section for the new version containing all unreleased changes, and the `[Unreleased]` section MUST be reset to empty after the bump.

#### Scenario: Changelog matches release tag

- GIVEN a tag `v1.0.0` is pushed
- WHEN `CHANGELOG.md` is checked
- THEN a `## [1.0.0]` section exists with all changes previously under `[Unreleased]`

#### Scenario: Unreleased section is empty after bump

- GIVEN all unreleased changes are moved to the version section
- WHEN `CHANGELOG.md` is checked
- THEN the `[Unreleased]` section is empty, containing only the header and link definitions

### Requirement: Tag-Version Consistency Validation

The release workflow MUST validate that the git tag version matches `workspace.package.version` before proceeding with any release steps.

#### Scenario: Tag version matches workspace version

- GIVEN a tag `v1.0.0` is pushed and `workspace.package.version` is `1.0.0`
- WHEN the release workflow runs the version check step
- THEN the extracted tag version matches the Cargo.toml version and the workflow continues

#### Scenario: Version mismatch detected and release blocked

- GIVEN a tag `v1.0.1` is pushed but `workspace.package.version` is `1.0.0`
- WHEN the version check step runs
- THEN the workflow fails with a clear error: tag version 1.0.1 does not match Cargo.toml version 1.0.0