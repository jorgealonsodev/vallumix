# cli-orchestration Specification

## Purpose

Wire the `apply`, `audit`, `rollback`, and `completion` CLI subcommands to real logic: profile loading, control resolution, parallel audit with rayon, report generation, and proper exit codes per PRD §5.4 and §7.3.

## Requirements

### Requirement: Apply Command Orchestration

The `apply` subcommand MUST execute the full apply workflow: load profile → resolve controls → create backup session → iterate controls (check → backup → apply → post-check) → generate report → determine exit code. Each control failure MUST be recorded but MUST NOT stop execution of remaining controls.

#### Scenario: Apply command end-to-end with profile web

- GIVEN `vallumix apply --profile web`
- WHEN the command runs
- THEN it loads `web.toml`, resolves 22 controls, creates a backup session, applies all controls sequentially, generates a report, and exits 0 if compliance ≥ threshold

#### Scenario: Apply with dry-run skips modifications

- GIVEN `vallumix apply --profile web --dry-run`
- WHEN the command runs
- THEN it runs `check()` on all controls but MUST NOT call `apply()` or create backups; it reports what WOULD change

#### Scenario: Individual control failure does not stop execution

- GIVEN control "5.2.4" fails during apply
- WHEN the remaining controls are iterated
- THEN all controls are still processed; the failure is recorded in the report

### Requirement: Audit Command with Parallel Execution

The `audit` subcommand MUST execute only `check()` on each resolved control, using `rayon` for parallel execution. The audit command MUST NOT modify the system.

#### Scenario: Audit runs checks in parallel

- GIVEN `vallumix audit --profile web` with 22 controls
- WHEN the command runs
- THEN `rayon::par_iter` is used to call `check()` on controls concurrently

#### Scenario: Audit does not modify the system

- GIVEN `vallumix audit --profile web`
- WHEN the command completes
- THEN no files are modified, no backups are created, and no services are changed

#### Scenario: Audit with specific report format

- GIVEN `vallumix audit --profile web --report junit`
- WHEN the command completes
- THEN a JUnit XML report is generated to stdout or the `--output` path

### Requirement: Rollback Command

The `rollback` subcommand MUST support two modes: `--control-id <ID>` to rollback a specific control, or no control-id to rollback the most recent session entirely.

#### Scenario: Rollback specific control

- GIVEN `vallumix rollback --control-id 5.2.4`
- WHEN the command runs
- THEN `BackupManager::rollback_by_control("5.2.4")` is called and the control's original state is restored

#### Scenario: Rollback most recent session

- GIVEN `vallumix rollback` with no `--control-id`
- WHEN the command runs
- THEN `BackupManager::rollback_session(most_recent)` is called and all controls from the last session are restored

### Requirement: Completion Command

The `completion` subcommand MUST generate shell completion scripts using `clap_complete`. It MUST support `bash`, `zsh`, `fish`, and `nushell` shells, writing to stdout.

#### Scenario: Generate bash completion

- GIVEN `vallumix completion bash`
- WHEN the command runs
- THEN bash completion script is written to stdout

### Requirement: Progress Bar Integration

The `apply` and `audit` commands MUST display a progress bar via `indicatif` during control iteration. The progress bar MUST show: current control ID, total count, and completion percentage. It MUST be suppressed when `--quiet` is set.

#### Scenario: Progress bar displays during apply

- GIVEN `vallumix apply --profile web` with 22 controls
- WHEN execution begins
- THEN an `indicatif` progress bar appears showing "[1/22] 1.1.1.1 Disable cramfs..."

#### Scenario: Progress bar suppressed with --quiet

- GIVEN `vallumix apply --profile web --quiet`
- WHEN the command runs
- THEN no progress bar is displayed

### Requirement: Report Generation Post-Run

After all controls are processed, `apply` and `audit` MUST generate reports in all requested formats (via `--report` flag). Supported formats: `json`, `html`, `junit`, `text`. Multiple formats MAY be specified comma-separated.

#### Scenario: Generate HTML and JSON reports

- GIVEN `vallumix audit --profile web --report html,json`
- WHEN the command completes
- THEN both an HTML file and a JSON string are generated containing the full report

### Requirement: Exit Codes

Exit codes MUST follow PRD §5.4: `0` when compliance rate ≥ threshold, `1` when below threshold, `2` for configuration errors, `3` for privilege errors.

#### Scenario: Below threshold exits 1

- GIVEN `vallumix audit --profile web --threshold 95` and compliance rate is `87%`
- WHEN the program exits
- THEN exit code is `1`

## Acceptance Criteria

- [ ] `apply` command runs full workflow: profile → resolve → backup → apply → report
- [ ] `audit` command runs parallel checks with `rayon`, no system modification
- [ ] `rollback` supports both single control and full session modes
- [ ] `completion` generates shell scripts using `clap_complete`
- [ ] Progress bars via `indicatif`, suppressed by `--quiet`
- [ ] Reports generated in all requested formats post-run
- [ ] Correct exit codes: 0/1/2/3 per PRD §5.4