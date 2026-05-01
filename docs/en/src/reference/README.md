# Reference

This section exhaustively documents Vallumix's command-line interface: subcommands, global flags, exit codes, and expected behavior for each operation.

## Subcommands

Vallumix follows the subcommand pattern popularized by `git` and `cargo`:

| Subcommand | Description | Modifies the System |
|---|---|---|
| `apply` | Applies controls from the selected profile, generates backups and reports | **Yes** |
| `audit` | Evaluates state without applying changes, generates compliance report | No |
| `rollback` | Restores configurations from versioned backups | **Yes** (reverts) |
| `list` | Shows the catalog of available controls | No |
| `completion` | Generates autocompletion scripts for shells | No |

## Global Flags

These flags are available in all subcommands:

| Flag | Value | Description |
|---|---|---|
| `--profile` | `web`, `database`, `bastion` | Profile of controls to use |
| `--dry-run` | — | Simulates execution without modifying the system |
| `--verbose` | — | Shows detailed output for each control |
| `--quiet` | — | Suppresses console output (reports only) |
| `--threshold` | `0`–`100` | Minimum compliance percentage for exit code 0 |
| `--no-color` | — | Disables colors in terminal output |
| `--report` | `html`, `json`, `junit`, `text` | Report formats to generate (comma-separated) |
| `--output` | path | Path prefix for report files |
| `--help` | — | Shows subcommand help |
| `--version` | — | Shows Vallumix version |

## Exit Codes

Vallumix returns explicit exit codes that facilitate integration with scripts and pipelines:

| Code | Meaning | When It Occurs |
|---|---|---|
| `0` | Success | Compliance ≥ threshold (or no threshold configured); operation completed without errors |
| `1` | Threshold not met | Compliance rate is below `--threshold` |
| `2` | Configuration error | Invalid profile, unsupported distro, incorrect arguments |
| `3` | Privilege error | Operation requiring root executed without effective privileges |

```tip
Design your wrapper scripts to capture code `1` as "security policy violated" and code `2` as "review configuration". Do not treat both as generic "errors".
```

## Output Conventions

- In interactive mode with TTY, output uses colors, icons (✓ ✗ ⚠ ℹ), and progress bars.
- In non-TTY mode (pipes, redirects, CI), output simplifies to plain text without ANSI escape codes, respecting the `NO_COLOR` variable.
- Structured logging is controlled via `RUST_LOG` and the `--log-level` flag.
