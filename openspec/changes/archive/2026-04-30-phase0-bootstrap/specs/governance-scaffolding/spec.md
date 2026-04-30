# governance-scaffolding Specification

## Purpose

Establish project governance files that define contribution policies, code ownership, issue/PR templates, dependency automation, and licensing.

## Requirements

### Requirement: Dual License

The project MUST include both `LICENSE-MIT` and `LICENSE-APACHE` files at the repository root, matching the dual-license declaration (MIT OR Apache-2.0) in Cargo.toml metadata.

#### Scenario: Both license files exist and are valid

- GIVEN the repository root
- WHEN `LICENSE-MIT` and `LICENSE-APACHE` are checked for existence
- THEN both files MUST be present with standard license text

#### Scenario: Cargo.toml references dual license

- GIVEN any crate's `Cargo.toml`
- WHEN the `license` field is inspected
- THEN it MUST equal `MIT OR Apache-2.0`

### Requirement: CODEOWNERS

A `.github/CODEOWNERS` file MUST exist that defines default ownership for the repository.

#### Scenario: CODEOWNERS defines a default owner

- GIVEN `.github/CODEOWNERS` exists
- WHEN its content is parsed
- THEN a default `*` rule assigns ownership to at least one maintainer

### Requirement: Issue Templates

The project MUST provide GitHub issue templates for bug reports and feature requests under `.github/ISSUE_TEMPLATE/`.

#### Scenario: Bug report template renders correctly

- GIVEN `.github/ISSUE_TEMPLATE/bug_report.yml` exists
- WHEN a contributor opens a new issue on GitHub
- THEN the bug report template appears as an option with required fields

#### Scenario: Feature request template renders correctly

- GIVEN `.github/ISSUE_TEMPLATE/feature_request.yml` exists
- WHEN a contributor opens a new issue on GitHub
- THEN the feature request template appears as an option with required fields

### Requirement: Pull Request Template

A `.github/PULL_REQUEST_TEMPLATE.md` MUST exist with sections for linked issue, summary, and test plan.

#### Scenario: PR template appears on new PR

- GIVEN `.github/PULL_REQUEST_TEMPLATE.md` exists
- WHEN a contributor opens a new pull request
- THEN the PR description is pre-filled with the template sections

### Requirement: Dependabot Configuration

A `.github/dependabot.yml` MUST exist that enables automatic dependency update PRs for Cargo packages on a weekly schedule.

#### Scenario: Dependabot runs weekly for Cargo

- GIVEN `.github/dependabot.yml` configures `package-ecosystem: cargo` with `interval: weekly`
- WHEN Dependabot evaluates the repository
- THEN it MUST create PRs for outdated Cargo dependencies

### Requirement: Contributing Guide

A `CONTRIBUTING.md` MUST exist at the repository root describing how to contribute, build, test, and submit PRs.

#### Scenario: CONTRIBUTING.md contains build and test instructions

- GIVEN `CONTRIBUTING.md` is read
- WHEN its sections are inspected
- THEN it MUST include instructions for `cargo build`, `cargo test`, `cargo clippy`, and `cargo fmt`

### Requirement: Changelog

A `CHANGELOG.md` MUST exist following the Keep a Changelog format with an initial `Unreleased` section.

#### Scenario: CHANGELOG.md has correct structure

- GIVEN `CHANGELOG.md` is parsed
- WHEN the heading structure is checked
- THEN it contains an `Unreleased` section and follows Keep a Changelog conventions

### Requirement: README

A `README.md` MUST exist at the repository root with project name, description, build instructions, license, and a link to CONTRIBUTING.md.

#### Scenario: README contains essential project information

- GIVEN `README.md` is read
- WHEN its content is checked
- THEN it MUST contain the project name, a description, build/test instructions, license reference, and link to CONTRIBUTING.md