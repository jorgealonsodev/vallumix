# repo-metadata-fix Specification

## Purpose

Correct repository and package metadata in all Cargo.toml files to ensure accurate links, successful crates.io listing, and consistent project identity across all 5 workspace crates.

## Requirements

### Requirement: Repository URL Correction

The `repository` field in `workspace.package` MUST be updated from `https://github.com/vallumix/vallumix` to `https://github.com/jorgealonsodev/vallumix` to match the actual GitHub repository URL.

#### Scenario: Repository URL matches actual GitHub location

- GIVEN the workspace Cargo.toml is parsed
- WHEN the `repository` field is read
- THEN it contains `https://github.com/jorgealonsodev/vallumix`

#### Scenario: crates.io links resolve correctly

- GIVEN vallumix-cli is published to crates.io
- WHEN a user clicks the repository link on crates.io
- THEN they are directed to `https://github.com/jorgealonsodev/vallumix`

### Requirement: crates.io Metadata Fields

All publishable crates MUST include `description`, `keywords`, `categories`, and `homepage` fields. These fields MUST be defined at the workspace level in `[workspace.package]` and inherited by member crates via `*.workspace = true`.

#### Scenario: vallumix-cli has complete metadata

- GIVEN `crates/vallumix-cli/Cargo.toml` uses `description.workspace = true`, `keywords.workspace = true`, etc.
- WHEN `cargo publish --dry-run -p vallumix-cli` runs
- THEN no warnings about missing description, keywords, or categories appear

#### Scenario: Empty keywords array rejected by crates.io

- GIVEN a crate Cargo.toml has `keywords = []`
- WHEN `cargo publish` is attempted
- THEN crates.io rejects the crate with a validation error about empty keywords

#### Scenario: Homepage field points to documentation site

- GIVEN the `homepage` field is set
- WHEN a user views the crate on crates.io
- THEN the homepage link resolves to the mdBook documentation site or GitHub Pages URL

### Requirement: Workspace-Level Metadata Consistency

The workspace `Cargo.toml` MUST define `description`, `keywords`, `categories`, and `homepage` in `[workspace.package]` so that member crates can inherit them consistently.

#### Scenario: Member crates inherit workspace metadata

- GIVEN `[workspace.package]` defines `description`, `keywords`, `categories`, and `homepage`
- WHEN a member crate uses `description.workspace = true`
- THEN `cargo metadata` reports the inherited value from the workspace