# Delta for cli-structure

## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Clap Derive Subcommands

The CLI MUST define 5 subcommands via `clap` derive: `apply`, `audit`, `rollback`, `list`, `completion`. Each subcommand MUST have dedicated `#[derive(Subcommand)]` enum variants. The `apply` subcommand MUST accept `--profile <NAME>` (default `web`), `--dry-run`, `--report <FORMATS>` (comma-separated, values: `json`, `html`, `junit`, `text`), `--output <PATH>`. The `audit` subcommand MUST accept `--profile <NAME>`, `--report <FORMATS>`, `--output <PATH>`. The `rollback` subcommand MUST accept `--control-id <ID>` (optional). The `completion` subcommand MUST accept a required `shell` argument. All subcommands MUST be wired to their respective implementation functions in `vallumix-cli::commands`.
(Previously: Subcommands were defined as stubs with `todo!()` implementations; `--report` only supported `json`; `--output` did not exist.)

#### Scenario: Apply subcommand accepts required profile flag

- GIVEN the CLI is invoked as `vallumix apply --profile web`
- WHEN args are parsed
- THEN the `apply` subcommand is selected with `profile` set to `"web"`

#### Scenario: Audit subcommand accepts report formats

- GIVEN `vallumix audit --profile web --report html,json`
- WHEN args are parsed
- THEN `report_formats` is `["html", "json"]`

#### Scenario: Rollback subcommand accepts optional control-id

- GIVEN `vallumix rollback --control-id 5.2.4`
- WHEN args are parsed
- THEN `control_id` is `Some("5.2.4")`

#### Scenario: List subcommand requires no mandatory flags

- GIVEN `vallumix list`
- WHEN args are parsed
- THEN the `list` subcommand is selected and executes without additional flags

#### Scenario: Completion subcommand accepts shell argument

- GIVEN `vallumix completion bash`
- WHEN args are parsed
- THEN `shell` is set to `bash`