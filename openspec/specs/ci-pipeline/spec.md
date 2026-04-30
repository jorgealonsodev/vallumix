# ci-pipeline Specification

## Purpose

Establish a GitHub Actions CI workflow that validates the workspace builds, passes tests, and meets lint/format standards across multiple Linux distributions on x86_64.

## Requirements

### Requirement: Multi-Distro CI Matrix

The CI workflow MUST build and test the workspace on a matrix of x86_64 Linux distributions: Debian 12, Ubuntu 22.04, Ubuntu 24.04, and Rocky Linux 9.

#### Scenario: CI runs on all four distros

- GIVEN a push to `develop` or a pull request targeting `develop`
- WHEN the CI workflow triggers
- THEN four parallel jobs execute, one per distro in the matrix

#### Scenario: Distro-specific container is used

- GIVEN the matrix includes `debian:12` as a container image
- WHEN the build step runs for that matrix entry
- THEN the build environment is Debian 12 with the specified Rust toolchain

### Requirement: Build and Test Steps

The CI workflow MUST include steps for `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check`, and all four MUST pass.

#### Scenario: All quality gates pass on a valid codebase

- GIVEN all crates compile and have no clippy warnings
- WHEN the CI pipeline runs
- THEN `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all exit with code 0

#### Scenario: Clippy warning fails the pipeline

- GIVEN a clippy warning exists in any crate
- WHEN `cargo clippy -- -D warnings` runs in CI
- THEN the step exits with a non-zero code and the pipeline fails

### Requirement: Workflow Trigger Conditions

The CI workflow MUST trigger on pushes to `develop` and on pull requests targeting `develop`.

#### Scenario: Push to develop triggers CI

- GIVEN a commit is pushed to the `develop` branch
- WHEN GitHub receives the push event
- THEN the CI workflow runs automatically

#### Scenario: Pull request to develop triggers CI

- GIVEN a pull request is opened or updated targeting `develop`
- WHEN GitHub receives the pull request event
- THEN the CI workflow runs automatically

### Requirement: CI Uses Pinned Rust Version

The CI workflow MUST install and use Rust 1.75 as specified by `rust-toolchain.toml`.

#### Scenario: CI respects rust-toolchain.toml

- GIVEN `rust-toolchain.toml` specifies channel 1.75
- WHEN the CI job installs Rust via `rustup`
- THEN cargo commands use the 1.75 toolchain

### Requirement: Fail-Fast Disabled

The CI matrix MUST NOT cancel in-flight jobs when one job fails, so that all distro results are visible.

#### Scenario: One distro failure does not cancel others

- GIVEN the CI matrix has 4 distro jobs running in parallel
- WHEN the Rocky 9 job fails
- THEN the Debian 12, Ubuntu 22.04, and Ubuntu 24.04 jobs continue to completion