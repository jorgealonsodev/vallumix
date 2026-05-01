# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Placeholder for upcoming features and improvements.

### Changed

- Placeholder for breaking or significant behavioral changes.

### Deprecated

- Placeholder for features scheduled for removal.

### Removed

- Placeholder for removed features.

### Fixed

- Placeholder for bug fixes.

### Security

- Placeholder for security-related changes.

## [1.0.0] - 2026-05-01

### Added

- **70+ CIS controls** covering user management, SSH hardening, networking, filesystem permissions, services, logging, authentication, and system maintenance.
- **4 report generators**: HTML (self-contained with embedded CSS), JSON (validable schema), JUnit XML (CI/CD ready), and plain text with optional terminal colors.
- **Full backup and rollback system**: Automatic versioned backups before every change. Rollback by session ID or individual control ID.
- **3 built-in profiles**: `web`, `database`, and `bastion`, each tailoring control selection and strictness to the server's role.
- **Multi-distribution support**: Debian 12, Ubuntu 22.04/24.04 LTS, RHEL 9, Rocky Linux 9, AlmaLinux 9.
- **Static musl binary**: Single self-contained executable with no runtime dependencies.
- **SLSA Level 3 provenance**: Supply-chain attestation via slsa-github-generator.
- **Cosign keyless signing**: All release artifacts signed with Sigstore cosign.
- **Cross-compilation**: `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` targets.
- **Bilingual documentation**: mdBook-based docs in English and Spanish, published to GitHub Pages.
- **API documentation**: docs.rs integration for all workspace crates.
- **Shell completions**: bash, zsh, fish, and nushell via clap_complete.
- **Man page generation**: via clap_mangen.
- **Debian and RPM packaging**: cargo-deb and cargo-generate-rpm metadata.
- **CycloneDX SBOM**: cargo-sbom integration for release transparency.
- **Packer example**: Automated VM hardening workflow.
- **CI/CD pipelines**: Build, test, coverage, audit, deny, docs, and release automation.
- **Workspace architecture**: 5 crates (core, controls, reporters, backup, cli) enabling library reuse.

### Changed

- Initial stable release. No prior versions to compare.

## [0.0.1] - 2026-04-30

### Added

- Initial workspace bootstrap with 5 crates and core traits (`Control`, `Reporter`, `Profile`, `VallumixError`).
- CLI structure with clap subcommands: `apply`, `audit`, `rollback`, `list`, `completion`.
- CI workflows for build, test, lint, and security audit.
