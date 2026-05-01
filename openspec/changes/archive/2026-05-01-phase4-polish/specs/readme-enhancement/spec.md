# readme-enhancement Specification

## Purpose

Transform the minimal 39-line README into a professional, comprehensive project landing page with badges, features, architecture diagram, usage examples, and comparison table per PRD §2.2, §3.2, §6.5.

## Requirements

### Requirement: Badges and Status Indicators

README.md MUST include CI status, license (MIT OR Apache-2.0), crates.io version, MSRV (1.75), and coverage badges at the top of the file, rendered correctly on GitHub.

#### Scenario: Badges render on GitHub

- GIVEN README.md is viewed on GitHub
- WHEN the page loads
- THEN five badges are visible: CI (build passing), license, crates.io version, MSRV, and coverage

### Requirement: Features Table

README MUST include a features table listing 8+ key capabilities: idempotent execution, dry-run mode, rollback, 3 profile presets, 4 report formats, 70+ CIS controls, multi-distro support, static musl binary.

#### Scenario: Features table lists all capabilities

- GIVEN a viewer reads the Features section
- WHEN they scan the table
- THEN each capability has a row with name and one-line description

### Requirement: Architecture Diagram

README MUST include an SVG architecture diagram showing the workspace crate graph: `vallumix-cli` → `vallumix-core` ← `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`.

#### Scenario: Diagram renders inline

- GIVEN README.md is rendered on GitHub
- WHEN the Architecture section is viewed
- THEN an SVG diagram displays crate relationships with directional arrows

### Requirement: Usage Examples for All Subcommands

README MUST include copy-paste-ready code blocks for `apply`, `audit`, `rollback`, `list`, and `completion` subcommands with `--profile`, `--dry-run`, `--report`, and `--threshold` flags.

#### Scenario: New user runs first command from README

- GIVEN a new user reads Quick Start
- WHEN they copy the first example
- THEN `sudo vallumix audit --profile web --report html` runs successfully on a supported distro

#### Scenario: All subcommands are demonstrated

- GIVEN the Usage section exists
- WHEN scanned by a reader
- THEN examples for apply, audit, rollback, list, and completion are present

### Requirement: Comparison Table from PRD

README MUST reproduce the PRD §2.2 comparison table: Vallumix vs OpenSCAP vs Lynis vs Ansible Lockdown, comparing language, distribution, applies changes, profiles, rollback, and binary categories.

#### Scenario: Comparison table differentiates Vallumix

- GIVEN README is viewed
- WHEN the comparison section is reached
- THEN a table with 4 tools × 7 attributes is present