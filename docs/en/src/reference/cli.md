# Global CLI

This page documents the global flags available in all Vallumix subcommands, with usage examples and common combinations.

## `--profile <web|database|bastion>`

Selects the profile of controls to execute. It is required in `apply`, `audit`, and `list`.

```bash
vallumix apply --profile web
vallumix audit --profile database
vallumix list --profile bastion
```

## `--dry-run`

Simulates the complete execution without modifying the system. Works with `apply` and `rollback`.

```bash
vallumix apply --profile web --dry-run --verbose
```

In dry-run mode:
- `check` is executed on all controls.
- It reports what changes would be made.
- No backups are created and no files are modified.
- The exit code is `0` if argument validation is correct, regardless of compliance.

## `--verbose`

Shows detailed information for each control: executed commands, raw output, modified files.

```bash
vallumix apply --profile web --verbose
```

Useful for debugging and capturing technical traceability in audits.

## `--quiet`

Suppresses all console output except fatal errors. Reports are generated normally.

```bash
vallumix apply --profile web --quiet --report json --output /tmp/report
```

Ideal for pipelines where only the exit code and report artifact matter.

## `--threshold <0-100>`

Defines the minimum compliance percentage for the execution to return exit code `0`.

```bash
vallumix apply --profile web --threshold 95
vallumix audit --profile database --threshold 85
```

If the compliance rate is below the threshold, Vallumix returns `1`.

```warning
The threshold applies to the final execution result. A `Failed` control reduces the compliance rate the same as a `NonCompliant` one. Configure the threshold realistically according to the security maturity of your environment.
```

## `--no-color`

Disables colored terminal output, also respecting the `NO_COLOR` environment variable.

```bash
vallumix apply --profile web --no-color
NO_COLOR=1 vallumix audit --profile web
```

## `--report <formats>`

Specifies one or more report formats separated by commas.

```bash
vallumix apply --profile web --report html
vallumix audit --profile database --report html,json,junit
vallumix apply --profile bastion --report text
```

Supported formats:

| Format | Extension | Typical Use |
|---|---|---|
| `html` | `.html` | Executive report, audits, clients |
| `json` | `.json` | SIEM integration, dashboards, scripts |
| `junit` | `.xml` | Jenkins, GitLab CI, GitHub Actions |
| `text` | `.txt` | Quick terminal review, logs |

## `--output <prefix>`

Path prefix for report files. If not specified, Vallumix uses a temporary name in `/tmp`.

```bash
vallumix apply --profile web --report html,json --output /var/reports/web-$(date +%Y%m%d)
# Generates: /var/reports/web-20260501.html and /var/reports/web-20260501.json
```

## Common Combinations

```bash
# Silent audit with JSON report for dashboard
vallumix audit --profile web --quiet --report json --output /var/lib/metrics/vallumix

# Application with threshold validation and JUnit report for CI
vallumix apply --profile database --threshold 95 --report junit --output /tmp/results

# Verbose dry-run for manual review before production
vallumix apply --profile bastion --dry-run --verbose --report html --output /tmp/dryrun

# Full listing without colors for processing with awk/grep
vallumix list --profile web --no-color
```
