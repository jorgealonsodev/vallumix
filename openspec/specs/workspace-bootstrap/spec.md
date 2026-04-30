# workspace-bootstrap Specification

## Purpose

Establish the Cargo workspace structure with all 5 crates as compilable stubs, toolchain configuration, and project-level tooling.

## Requirements

### Requirement: Workspace Root Configuration

The root `Cargo.toml` MUST define a Cargo workspace with members `crates/vallumix-core`, `crates/vallumix-controls`, `crates/vallumix-reporters`, `crates/vallumix-backup`, and `crates/vallumix-cli`, and MUST include a `[workspace.dependencies]` section for shared dependency version pinning.

#### Scenario: Cargo build succeeds for full workspace

- GIVEN the workspace root Cargo.toml exists with all 5 members listed
- WHEN `cargo build` is executed from the workspace root
- THEN all 5 crates compile without errors

#### Scenario: Missing crate breaks workspace resolution

- GIVEN the workspace Cargo.toml lists `crates/vallumix-controls` as a member
- WHEN the `crates/vallumix-controls/` directory does not exist
- THEN `cargo build` MUST report a workspace error indicating the missing member

### Requirement: Crate Stubs Compile Independently

Each crate MUST have a valid `Cargo.toml` and a compilable `src/lib.rs` (or `src/main.rs` for `vallumix-cli`) that builds without warnings.

#### Scenario: Individual crate compiles with cargo check

- GIVEN `crates/vallumix-core/` contains `Cargo.toml` and `src/lib.rs`
- WHEN `cargo check -p vallumix-core` is executed
- THEN the crate compiles with zero errors and zero warnings

#### Scenario: CLI crate produces a binary entry point

- GIVEN `crates/vallumix-cli/` contains `Cargo.toml` with `[[bin]]` and `src/main.rs` with `fn main()`
- WHEN `cargo build -p vallumix-cli` is executed
- THEN a `vallumix-cli` binary is produced in `target/debug/`

### Requirement: MSRV and Toolchain Pinning

A `rust-toolchain.toml` MUST pin the minimum supported Rust version to 1.75 with edition 2021.

#### Scenario: Correct toolchain is selected automatically

- GIVEN `rust-toolchain.toml` specifies channel 1.75
- WHEN `cargo build` is run in the workspace
- THEN Rust 1.75 toolchain is used (or installed via rustup)

#### Scenario: Incompatible Rust version is rejected

- GIVEN a dependency requires a Rust version newer than 1.75
- WHEN `cargo build` is executed
- THEN the build MUST fail with an MSRV compatibility error

### Requirement: Cargo Deny Configuration

A `deny.toml` MUST exist at the workspace root with license allowlist (MIT, Apache-2.0), advisory checks enabled, and duplicate dependency detection.

#### Scenario: cargo deny check passes on empty workspace

- GIVEN `deny.toml` exists with default policies
- WHEN `cargo deny check` is executed
- THEN it completes without errors (advisory, license, and duplicate sources checks pass)

### Requirement: Git Ignore

A `.gitignore` MUST exclude Cargo build artifacts, IDE files, and OS-specific files.

#### Scenario: Build artifacts are ignored

- GIVEN `target/` directory exists after `cargo build`
- WHEN `git status` is run
- THEN `target/` files MUST NOT appear as untracked

### Requirement: Profile Placeholders

The workspace MUST include empty placeholder TOML files at `profiles/web.toml`, `profiles/database.toml`, and `profiles/bastion.toml`.

#### Scenario: Profile files exist and are valid TOML

- GIVEN the workspace is initialized
- WHEN `profiles/web.toml` is parsed as TOML
- THEN parsing succeeds (even if content is empty or minimal)