# ci-pipeline Specification

## Purpose

Establish a GitHub Actions CI workflow that validates the workspace builds, passes tests, and meets lint/format standards across multiple Linux distributions on x86_64, and provides cross-architecture build verification, coverage reporting, dependency auditing, and security scanning.

## Requirements

### Requirement: Multi-Distro CI Matrix

The CI workflow MUST build and test the workspace on a matrix of x86_64 Linux distributions: Debian 12, Ubuntu 22.04, Ubuntu 24.04, and Rocky Linux 9. Additionally, an aarch64 cross-build job MUST compile (without testing) for `aarch64-unknown-linux-gnu`.

#### Scenario: CI runs on all four x86_64 distros plus aarch64 cross-build

- GIVEN a push to `develop` or a pull request targeting `develop`
- WHEN the CI workflow triggers
- THEN four parallel x86_64 jobs execute (one per distro) plus an aarch64 cross-build job

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

### Requirement: aarch64 Cross-Build Job

The CI workflow MUST include a separate job that cross-compiles the workspace for `aarch64-unknown-linux-gnu` using the `cross` tool. This job MUST build all crates but SHALL NOT run tests (aarch64 tests cannot execute on x86_64 runners).

#### Scenario: aarch64 cross-build succeeds

- GIVEN a push to `develop` or a pull request targeting `develop`
- WHEN the CI workflow triggers
- THEN an `aarch64-build` job compiles the workspace for `aarch64-unknown-linux-gnu` using `cross build --target aarch64-unknown-linux-gnu`

#### Scenario: aarch64 build failure does not cancel x86_64 jobs

- GIVEN the aarch64 cross-build job fails
- WHEN other matrix jobs are running
- THEN the x86_64 distro jobs continue to completion

### Requirement: Coverage Report with cargo-tarpaulin

The CI workflow MUST include a `coverage` job that runs `cargo-tarpaulin --workspace --out Html --out Lcov`. Coverage results MUST be uploaded as artifacts. The job SHOULD fail if workspace line coverage is below 80%.

#### Scenario: Coverage job runs on CI

- GIVEN a push to `develop`
- WHEN the CI workflow triggers
- THEN a `coverage` job runs `cargo tarpaulin --workspace --out Html --out Lcov`

#### Scenario: Coverage below threshold fails the job

- GIVEN workspace line coverage is below 80%
- WHEN the `coverage` job completes
- THEN the job exits with a non-zero code

### Requirement: Dependency Audit with cargo-deny

The CI workflow MUST include a `deps-audit` job that runs `cargo deny check` using the existing `deny.toml` configuration. The job MUST run on every push to `develop` and on pull requests.

#### Scenario: cargo-deny passes with allowed licenses

- GIVEN all dependencies use MIT or Apache-2.0 licenses
- WHEN `cargo deny check` runs in CI
- THEN the step exits with code 0

#### Scenario: cargo-deny fails on banned dependency

- GIVEN a dependency uses a non-allowlisted license
- WHEN `cargo deny check` runs
- THEN the step exits with non-zero code and reports the offending crate

### Requirement: Security Audit with cargo-audit

The CI workflow MUST include a `security-audit` job that runs `cargo audit`. This job MUST run on every push to `develop` and on pull requests. Additionally, a weekly scheduled run MUST execute `cargo audit` every Monday at 00:00 UTC.

#### Scenario: Security audit passes with no vulnerabilities

- GIVEN no known vulnerabilities in dependencies
- WHEN `cargo audit` runs in CI
- THEN the step exits with code 0

#### Scenario: Security audit fails on known vulnerability

- GIVEN a dependency has a known RUSTSEC advisory
- WHEN `cargo audit` runs
- THEN the step exits with non-zero code and reports the advisory

#### Scenario: Weekly scheduled audit runs automatically

- GIVEN no push or PR activity
- WHEN Monday 00:00 UTC arrives
- THEN the `security-audit` workflow triggers and runs `cargo audit`

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

### Requirement: Release Workflow on Version Tags

A `.github/workflows/release.yml` workflow MUST trigger on pushes matching `v*` tags. It MUST build release binaries for `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`, generate `.deb` and `.rpm` packages, produce SHA256 checksums, and create a GitHub Release with all artifacts attached.

#### Scenario: Tag push triggers release build

- GIVEN a tag `v0.9.0` is pushed
- WHEN the release workflow triggers
- THEN two musl binaries are built: `vallumix-x86_64-musl` and `vallumix-aarch64-musl`

#### Scenario: Release artifacts include packages

- GIVEN the release workflow completes successfully
- WHEN the GitHub Release page is viewed
- THEN it contains tarballs for each architecture, `.deb`, `.rpm`, and `SHA256SUMS` file

#### Scenario: Release workflow runs independently of CI

- GIVEN the release workflow is triggered by a tag
- WHEN the main CI workflow is also running on `develop`
- THEN both workflows complete independently without conflicts

### Requirement: Release Binary Strip and Optimize

Release binaries MUST be stripped (`strip`) and compressed with UPX to meet the PRD §6.3 size target of <8 MB.

#### Scenario: Stripped binary meets size target

- GIVEN release binaries are built and stripped
- WHEN `ls -lh` is checked
- THEN each binary is <8 MB

### Requirement: Cross-Compilation via Cross

The release workflow MUST use the `cross` tool for aarch64 builds, matching the CI cross-build pattern already established in the existing CI workflow.

#### Scenario: aarch64 binary built via cross

- GIVEN the release workflow runs the aarch64 build step
- WHEN `cross build --target aarch64-unknown-linux-musl --release` executes
- THEN an aarch64 static binary is produced

### Requirement: Changelog Included in Release

The release workflow MUST include the contents of `CHANGELOG.md` in the GitHub Release body, extracting the relevant version section.

#### Scenario: Release notes include changelog

- GIVEN a `v0.9.0` tag is pushed and `CHANGELOG.md` has a `[0.9.0]` section
- WHEN the release is created on GitHub
- THEN the release body contains the changelog entry for that version