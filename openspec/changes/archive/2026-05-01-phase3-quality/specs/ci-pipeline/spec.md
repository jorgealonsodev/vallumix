# Delta for ci-pipeline

## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Multi-Distro CI Matrix

The CI workflow MUST build and test the workspace on a matrix of x86_64 Linux distributions: Debian 12, Ubuntu 22.04, Ubuntu 24.04, and Rocky Linux 9. Additionally, an aarch64 cross-build job MUST compile (without testing) for `aarch64-unknown-linux-gnu`.
(Previously: Matrix only covered x86_64 distros without aarch64)

#### Scenario: CI runs on all four x86_64 distros

- GIVEN a push to `develop` or a pull request targeting `develop`
- WHEN the CI workflow triggers
- THEN four parallel jobs execute on x86_64 (one per distro) plus an aarch64 cross-build job

#### Scenario: Distro-specific container is used

- GIVEN the matrix includes `debian:12` as a container image
- WHEN the build step runs for that matrix entry
- THEN the build environment is Debian 12 with the specified Rust toolchain

## REMOVED Requirements

_None._