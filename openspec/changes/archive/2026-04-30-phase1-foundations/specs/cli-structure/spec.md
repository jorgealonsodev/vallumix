# cli-structure Specification

## Purpose

Define the CLI binary entry point using `clap` derive macros with 5 subcommands, global flags, and structured exit codes per PRD §5.4 and §7.3.

## Requirements

### Requirement: Clap Derive Subcommands

The CLI MUST define 5 subcommands via `clap` derive: `apply`, `audit`, `rollback`, `list`, `completion`. Each subcommand MUST have a dedicated `#[derive(Subcommand)]` enum variant with relevant per-command options.

#### Scenario: Apply subcommand accepts required profile flag

- GIVEN the CLI is invoked as `vallumix apply --profile web`
- WHEN args are parsed
- THEN the `apply` subcommand is selected with `profile` set to `"web"`

#### Scenario: Audit subcommand accepts profile and report flags

- GIVEN the CLI is invoked as `vallumix audit --profile web --report json`
- WHEN args are parsed
- THEN the `audit` subcommand is selected with `report_format` set to `json`

#### Scenario: Rollback subcommand accepts optional control-id

- GIVEN the CLI is invoked as `vallumix rollback --control-id 5.2.4`
- WHEN args are parsed
- THEN the `rollback` subcommand is selected with `control_id` set to `Some("5.2.4")`

#### Scenario: List subcommand requires no mandatory flags

- GIVEN the CLI is invoked as `vallumix list`
- WHEN args are parsed
- THEN the `list` subcommand is selected and executes without additional flags

#### Scenario: Completion subcommand accepts shell argument

- GIVEN the CLI is invoked as `vallumix completion bash`
- WHEN args are parsed
- THEN the `completion` subcommand is selected with `shell` set to `bash`

### Requirement: Global Flags

The CLI MUST support global flags: `--profile <NAME>` (default: `web`), `--dry-run`, `--verbose` (`-v`), `--quiet` (`-q`), `--threshold <N>` (default: `80`, range 0–100), `--no-color`, `--report <FORMAT>` (values: `json`, later `html`, `junit`, `text`). Flags `--verbose` and `--quiet` MUST be mutually exclusive.

#### Scenario: Default profile is web

- GIVEN `vallumix apply` is invoked without `--profile`
- WHEN args are parsed
- THEN profile defaults to `"web"`

#### Scenario: Threshold accepts numeric value

- GIVEN `vallumix apply --threshold 95` is invoked
- WHEN args are parsed
- THEN threshold is `95`

#### Scenario: Verbose and quiet are mutually exclusive

- GIVEN `vallumix --verbose --quiet apply` is invoked
- WHEN args are parsed
- THEN clap MUST return an error

### Requirement: Exit Codes

The CLI MUST exit with code `0` when all controls pass or compliance rate meets threshold, `1` when compliance rate is below threshold, `2` for configuration errors (invalid profile, malformed TOML), and `3` for privilege errors (not running as root when required).

#### Scenario: All controls pass — exit 0

- GIVEN `vallumix audit --profile web` runs and all controls are compliant
- WHEN the program exits
- THEN exit code is `0`

#### Scenario: Below threshold — exit 1

- GIVEN threshold is `95` and compliance rate is `80%`
- WHEN the program exits
- THEN exit code is `1`

#### Scenario: Invalid profile — exit 2

- GIVEN `vallumix apply --profile nonexistent` is invoked
- WHEN the profile file is not found
- THEN the program exits with code `2`

#### Scenario: Not running as root — exit 3

- GIVEN `vallumix apply` is invoked without root privileges
- WHEN the privilege check fails
- THEN the program exits with code `3`

### Requirement: Tracing Initialization

The CLI MUST initialize `tracing-subscriber` with the appropriate level based on `--verbose` (DEBUG), `--quiet` (WARN), or default (INFO). JSON output mode MUST use `tracing_subscriber::fmt::json()`.

#### Scenario: Verbose flag enables debug logging

- GIVEN `vallumix --verbose apply --profile web`
- WHEN tracing is initialized
- THEN the log level is set to `DEBUG`

#### Scenario: Quiet flag suppresses info logs

- GIVEN `vallumix --quiet apply --profile web`
- WHEN tracing is initialized
- THEN the log level is set to `WARN`