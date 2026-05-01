# deb-rpm-packages Specification

## Purpose

Provide native `.deb` and `.rpm` packages via `cargo-deb` and `cargo-generate-rpm`, enabling system-level installation on Debian/Ubuntu and RHEL/Rocky/Alma distributions per PRD §3.2 and §12.3 (item 9).

## Requirements

### Requirement: Debian Package Metadata

`vallumix-cli/Cargo.toml` MUST include a `[package.metadata.deb]` section specifying: maintainer, dependencies (`libc6`), section (`admin`), priority (`optional`), and assets mapping the binary to `/usr/bin/`, man page to `/usr/share/man/man1/`, and completion scripts to `/usr/share/bash-completion/` and `/usr/share/zsh/vendor-completions/`.

#### Scenario: cargo deb produces valid package

- GIVEN `cargo deb --no-build` is run
- WHEN the `.deb` package is produced
- THEN `dpkg-deb -I` shows correct maintainer, section (admin), and priority (optional)

#### Scenario: Package installs all assets

- GIVEN `dpkg -i vallumix_*.deb` on a clean Debian 12 VM
- WHEN `which vallumix`, `man vallumix`, and bash completion are checked
- THEN binary is at `/usr/bin/vallumix`, man page at `/usr/share/man/man1/`, and completions are installed

### Requirement: RPM Package Metadata

`vallumix-cli/Cargo.toml` MUST include `[package.metadata.generate-rpm]` with assets (binary, man page, completions), `auto-req = "no"`, and license fields matching the dual MIT/Apache-2.0 license.

#### Scenario: cargo generate-rpm produces valid package

- GIVEN `cargo generate-rpm` is run
- WHEN the `.rpm` package is produced
- THEN `rpm -qip` shows correct summary, license, and vendor

#### Scenario: RPM installs all assets

- GIVEN `rpm -i vallumix-*.rpm` on a clean Rocky 9 VM
- WHEN `which vallumix` and `man vallumix` are checked
- THEN the binary and man page are installed at their standard paths

### Requirement: Package Validation on Clean VMs

Both packages MUST install cleanly and `vallumix --help` MUST exit 0 on their respective target distributions without additional runtime dependencies beyond libc.

#### Scenario: Debian package validation

- GIVEN a clean Debian 12 VM with no Rust toolchain
- WHEN `dpkg -i vallumix_*.deb && vallumix --help`
- THEN the exit code is 0 and help text is displayed

#### Scenario: RPM package validation

- GIVEN a clean Rocky 9 VM with no Rust toolchain
- WHEN `rpm -i vallumix-*.rpm && vallumix --help`
- THEN the exit code is 0 and help text is displayed

### Requirement:CHANGELOG.md in Keep a Changelog Format

A `CHANGELOG.md` file MUST exist at the project root following the Keep a Changelog format with `[Unreleased]` and `v0.0.1` sections per PRD §6.4.

#### Scenario: CHANGELOG exists and follows format

- GIVEN the project root
- WHEN `CHANGELOG.md` is read
- THEN it contains `[Unreleased]` and `[0.0.1] - YYYY-MM-DD` sections with Added, Changed, Fixed subsections