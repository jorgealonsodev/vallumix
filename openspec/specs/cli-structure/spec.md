# cli-structure Specification

## Purpose

Define the CLI binary entry point using `clap` derive macros with 5 subcommands, global flags, and structured exit codes per PRD §5.4 and §7.3.

## Requirements

### Requirement: Clap Derive Subcommands

The CLI MUST define 5 subcommands via `clap` derive: `apply`, `audit`, `rollback`, `list`, `completion`. Each subcommand MUST have dedicated `#[derive(Subcommand)]` enum variants. The `apply` subcommand MUST accept `--profile <NAME>` (default `web`), `--dry-run`, `--report <FORMATS>` (comma-separated, values: `json`, `html`, `junit`, `text`), `--output <PATH>`. The `audit` subcommand MUST accept `--profile <NAME>`, `--report <FORMATS>`, `--output <PATH>`. The `rollback` subcommand MUST accept `--control-id <ID>` (optional). The `completion` subcommand MUST accept a required `shell` argument. All subcommands MUST be wired to their respective implementation functions in `vallumix-cli::commands`.

#### Scenario: Apply subcommand accepts required profile flag

- GIVEN the CLI is invoked as `vallumix apply --profile web`
- WHEN args are parsed
- THEN the `apply` subcommand is selected with `profile` set to `"web"`

#### Scenario: Audit subcommand accepts report formats

- GIVEN `vallumix audit --profile web --report html,json`
- WHEN args are parsed
- THEN `report_formats` is `["html", "json"]`

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

### Requirement: Report Format Flag Expansion

The `--report` flag MUST support `json`, `html`, `junit`, and `text` values. Multiple formats MAY be specified comma-separated (e.g., `--report html,json`). A corresponding `--output <PATH>` flag MUST direct report output to a file instead of stdout.

#### Scenario: Multiple report formats comma-separated

- GIVEN `vallumix audit --profile web --report html,json --output /tmp/report`
- WHEN args are parsed
- THEN `report_formats` is `["html", "json"]` and `output_path` is `Some("/tmp/report")`

#### Scenario: Single report format

- GIVEN `vallumix audit --profile web --report junit`
- WHEN args are parsed
- THEN `report_formats` is `["junit"]` and `output_path` is `None`

### Requirement: Rollback Subcommand Control-ID Optional

The `rollback` subcommand MUST accept an optional `--control-id <ID>` flag. When present, it rolls back only that specific control. When absent, it rolls back the most recent entire session.

#### Scenario: Rollback with control-id

- GIVEN `vallumix rollback --control-id 5.2.4`
- WHEN args are parsed
- THEN `control_id` is `Some("5.2.4")`

#### Scenario: Rollback without control-id (session rollback)

- GIVEN `vallumix rollback`
- WHEN args are parsed
- THEN `control_id` is `None` and the most recent session is targeted

### Requirement: Apply Dry-Run Flag

The `apply` subcommand MUST accept a `--dry-run` flag (already defined as a global flag) that suppresses all system modifications. In dry-run mode, `apply()` MUST NOT be called on any control; only `check()` is invoked and the results are reported as "what would change".

#### Scenario: Dry-run suppresses all modifications

- GIVEN `vallumix apply --profile web --dry-run`
- WHEN the command executes
- THEN no files are modified, no backups are created, and report shows projected changes

### Requirement: Completion Subcommand Shell Enum

The `completion` subcommand MUST use `clap_complete::Shell` directly, supporting Bash, Zsh, Fish, PowerShell, Elvish, and Nushell natively. The custom `Shell` enum in `main.rs` MUST be removed and replaced with `clap_complete::Shell`. Nushell MUST produce valid completion output (previously: custom Shell enum with Nushell stub returning "not yet supported by clap_complete").

#### Scenario: Nushell completion generates valid output

- GIVEN `vallumix completion nushell` is invoked
- WHEN the output is sourced in Nushell
- THEN completions for all subcommands and flags work without error

#### Scenario: All standard shells supported without stubs

- GIVEN `vallumix completion <shell>` is invoked
- WHEN `<shell>` is any of bash, zsh, fish, powershell, elvish, nushell
- THEN completion output is generated without error messages

#### Scenario: Existing shell completions unchanged

- GIVEN `vallumix completion bash` is invoked
- WHEN the output is compared to the previous implementation
- THEN the completion output format remains functionally equivalent (regression safety)
