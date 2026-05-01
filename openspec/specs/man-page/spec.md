# man-page Specification

## Purpose

Generate a Unix man page (`vallumix.1`) automatically from the CLI definition using `clap_mangen` at build time, per PRD §6.5 requirement for man page generation.

## Requirements

### Requirement: Build-Time Man Page Generation

A `build.rs` script in `vallumix-cli` MUST use `clap_mangen` to generate `man/vallumix.1` from the clap `App` struct at compile time. The generated file MUST be placed in the `OUT_DIR` and also written to `man/vallumix.1` in the project root for packaging.

#### Scenario: Man page generated on build

- GIVEN `cargo build` is invoked in `vallumix-cli`
- WHEN the build script executes
- THEN `man/vallumix.1` exists and is a valid roff file

#### Scenario: Man page reflects current CLI

- GIVEN a new subcommand or flag is added to the CLI
- WHEN `cargo build` runs
- THEN `man/vallumix.1` is regenerated with the new subcommand/flag

### Requirement: Man Page Content Coverage

The generated man page MUST include: NAME, SYNOPSIS, DESCRIPTION, SUBCOMMANDS (apply, audit, rollback, list, completion), OPTIONS (all global flags), and EXIT CODES (0–3) sections.

#### Scenario: All subcommands documented

- GIVEN the generated man page
- WHEN `man ./man/vallumix.1` is run
- THEN sections for apply, audit, rollback, list, and completion are present

#### Scenario: Exit codes documented

- GIVEN the generated man page
- WHEN the EXIT CODES section is read
- THEN codes 0, 1, 2, and 3 are documented with their meanings

### Requirement: Man Page Included in Packages

The man page MUST be included in `.deb` and `.rpm` packages at the standard path `/usr/share/man/man1/vallumix.1`.

#### Scenario: Man page available after .deb install

- GIVEN the `.deb` package is installed on Debian 12
- WHEN `man vallumix` is run
- THEN the man page displays correctly

#### Scenario: Man page available after .rpm install

- GIVEN the `.rpm` package is installed on Rocky 9
- WHEN `man vallumix` is run
- THEN the man page displays correctly