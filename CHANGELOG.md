# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial workspace bootstrap with 5 crates: `vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`, `vallumix-cli`.
- Core trait definitions: `Control`, `Reporter`, `Profile`, `Severity`, `Distro`, `ControlError`.
- Profile placeholders: `web`, `database`, `bastion`.
- CI pipeline with multi-distro matrix (Debian 12, Ubuntu 22.04, Ubuntu 24.04, Rocky Linux 9).
- Governance scaffolding: issue/PR templates, `CODEOWNERS`, `dependabot.yml`, dual license (MIT OR Apache-2.0).
