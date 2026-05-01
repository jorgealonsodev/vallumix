# `apply` Command

The `apply` subcommand is Vallumix's main operation. It evaluates each control in the selected profile and, if the system does not comply, applies the corresponding remediation after creating a backup.

## Syntax

```bash
vallumix apply [FLAGS] --profile <profile>
```

## apply-Specific Flags

In addition to global flags, `apply` supports:

| Flag | Description |
|---|---|
| `--profile` | **Required.** Profile of controls to apply. |
| `--dry-run` | Simulates execution without modifying anything. |
| `--threshold` | Minimum compliance threshold for success. |
| `--report` | Report formats to generate. |
| `--output` | Path prefix for reports. |
| `--verbose` | Shows detail of each control operation. |
| `--quiet` | Silences console output. |

## Behavior

1. **Validation:** checks root, supported distro, disk space.
2. **Profile loading:** reads the profile TOML and resolves the control list.
3. **Backup session creation:** versioned directory in `/var/backups/vallumix/<timestamp>/`.
4. **Control iteration:** for each control:
   - `check`: does it comply? → `Compliant`, skip to next.
   - `backup`: copies files to the backup session.
   - `apply`: executes the remediation.
   - `post_check`: verifies the change took effect.
5. **Report generation:** according to requested formats.
6. **Summary:** compliance rate, statuses, report path.
7. **Exit code:** `0` if threshold is met, `1` if not, `2` or `3` if there are errors.

## Examples

### Basic Application

```bash
sudo vallumix apply --profile web
```

Applies the web profile with default configuration. Generates an HTML report in `/tmp`.

### Application with Threshold and JSON Report

```bash
sudo vallumix apply --profile database --threshold 90 --report json --output /tmp/db-hardening
```

If the compliance rate is below 90%, the command returns `1`.

### Dry-Run Before Application

```bash
sudo vallumix apply --profile bastion --dry-run --verbose
```

Shows all changes that would be made without executing them.

### Silent Application in Pipeline

```bash
sudo vallumix apply --profile web --quiet --threshold 95 --report junit --output /tmp/results
```

Useful in scripts where only the exit code and JUnit artifact matter.

## Control States in apply

In an `apply` execution report, each control appears with one of these states:

| State | Meaning |
|---|---|
| `Compliant` | The system already complied before execution. |
| `Remediated` | It did not comply; remediation was applied and post-check passed. |
| `Failed` | It did not comply; remediation was attempted but post-check failed. |
| `Skipped` | The control is not applicable to this distribution or was excluded from the profile. |
| `SkippedAlreadyCompliant` | Explicit variant of `Compliant` due to idempotency. |

```danger
A control in `Failed` state indicates that the system remains in an insecure configuration for that control. Review the detailed report to understand why the remediation did not work: it could be due to an unexpected configuration file, a service blocking the modification, or a bug in the control implementation.
```
