# `audit` Command

The `audit` subcommand evaluates the security posture of the system without applying any changes. It is the primary tool for compliance audits, risk assessments, and security baselines.

## Syntax

```bash
vallumix audit [FLAGS] --profile <profile>
```

## audit-Specific Flags

| Flag | Description |
|---|---|
| `--profile` | **Required.** Profile of controls to audit. |
| `--threshold` | Minimum compliance threshold for success. |
| `--report` | Report formats to generate. |
| `--output` | Path prefix for reports. |
| `--verbose` | Shows detail of each verification. |
| `--quiet` | Silences console output. |

## Behavior

1. **Validation:** root, supported distro.
2. **Profile loading:** reads the TOML and resolves controls.
3. **Control iteration:** for each control, executes only `check`.
   - No backups are created.
   - No `apply` is executed.
   - No `post_check` is executed (there is no change to verify).
4. **Report generation:** with `Compliant` or `NonCompliant` states.
5. **Summary:** compliance rate.
6. **Exit code:** `0` if threshold is met, `1` if not.

## Parallelism with rayon

Unlike `apply`, `audit` mode parallelizes `check` execution using `rayon`. Controls that only read system state are independent of each other, so they can be safely evaluated concurrently.

```rust
// Simplification of the audit engine
use rayon::prelude::*;

let results: Vec<ControlResult> = controls
    .par_iter()   // rayon parallel iterator
    .map(|control| control.check(&ctx))
    .collect();
```

This significantly reduces audit time on systems with many controls. Parallelism is safe because `check` is read-only and each control implements `Send + Sync`.

## Examples

### Basic Audit

```bash
sudo vallumix audit --profile web
```

Evaluates all controls in the web profile and shows the summary in console.

### Audit with HTML and JSON Reports

```bash
sudo vallumix audit --profile database --report html,json --output /tmp/compliance-audit
```

Generates `/tmp/compliance-audit.html` and `/tmp/compliance-audit.json`.

### Audit with Threshold for Pipeline

```bash
sudo vallumix audit --profile web --threshold 85 --report junit --output /tmp/audit.xml
```

Returns `1` if the compliance rate is below 85%. Useful for quality gates in CI/CD.

### Verbose Audit for Diagnosis

```bash
sudo vallumix audit --profile bastion --verbose
```

Shows the exact commands executed to evaluate each control, useful for understanding false positives or negatives.

## Control States in audit

| State | Meaning |
|---|---|
| `Compliant` | The system complies with the CIS recommendation. |
| `NonCompliant` | The system does not comply; current evidence is documented. |
| `Skipped` | The control is not applicable to this distribution. |

```tip
Run `audit` periodically (weekly or monthly) as a cron task to detect configuration deviations caused by package updates, manual administrator changes, or application deployments. An `audit` that drops from 95% to 70% in a week is a clear alert signal.
```
