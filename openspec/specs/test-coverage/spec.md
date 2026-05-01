# test-coverage Specification

## Purpose

Achieve ≥80% line coverage via cargo-tarpaulin by testing all 38 untested CIS controls and shared modules with check, apply, dry-run, and rollback scenarios.

## Requirements

### Requirement: Control Test Coverage Pattern

Every CIS control MUST have unit tests covering four scenarios: `check` (compliant and non-compliant), `apply`, `dry-run`, and `rollback`. Controls using `with_paths()` MUST use fixture-based paths; controls using macros MUST test the underlying common struct.

#### Scenario: Control check returns Compliant

- GIVEN a control with fixture state meeting the CIS benchmark
- WHEN `control.check(&ctx)` is called
- THEN it returns `CheckStatus::Compliant`

#### Scenario: Control check returns NonCompliant

- GIVEN a control with fixture state violating the CIS benchmark
- WHEN `control.check(&ctx)` is called
- THEN it returns `CheckStatus::NonCompliant` with descriptive evidence

#### Scenario: Control apply writes expected changes

- GIVEN a control in non-compliant state
- WHEN `control.apply(&ctx)` is called
- THEN it returns `ApplyStatus::Applied` with a backup path, and the file reflects the hardened state

#### Scenario: Control apply in dry-run mode

- GIVEN a control in non-compliant state
- WHEN `control.apply(&dry_run_ctx)` is called with dry-run context
- THEN it returns `ApplyStatus::Skipped` and no system changes occur

#### Scenario: Control rollback restores original state

- GIVEN a control has been applied with a backup
- WHEN `control.rollback(&ctx, &backup)` is called
- THEN the system file is restored to its pre-apply state

### Requirement: Auth Control Tests

The 9 auth controls (CIS 5.1–5.5) MUST each have check/apply/dry-run/rollback test suites using `with_paths()` to override `/etc/pam.d/`, `/etc/security/`, `/etc/login.defs`, `/etc/profile`, and `/etc/bashrc`.

#### Scenario: PAM minlen check with fixture

- GIVEN `pwquality.conf` fixture contains `minlen = 8`
- WHEN `EnsurePamMinlen::with_paths(paths).check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence `"minlen=8, required=14"`

### Requirement: Logging Control Tests

The 11 logging controls (CIS 4.1–4.2) MUST each have test suites. `with_paths()` MUST override paths to `/etc/rsyslog.conf`, `/etc/rsyslog.d/`, `/var/log/`, `/etc/systemd/journald.conf`, `/etc/audit/`, and `/etc/logrotate.d/`.

#### Scenario: Auditd installed check with fixture

- GIVEN `/usr/sbin/auditd` fixture path exists
- WHEN `EnsureAuditdInstalled::with_paths(paths).check(&ctx)` is called
- THEN it returns `Compliant`

### Requirement: Shared Struct Tests

`ServiceDisable` (services/common.rs), `SysctlControl` (network/common.rs), and SSH common helpers MUST have dedicated test suites covering `check`, `apply`, `dry-run`, and `rollback` for each shared struct, rather than per-control duplication.

#### Scenario: ServiceDisable check with active service

- GIVEN `ServiceDisable` constructed with service name and mock active state
- WHEN `check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence containing the service name

#### Scenario: SysctlControl apply writes sysctl config

- GIVEN `SysctlControl` for `net.ipv4.ip_forward` with mock proc path
- WHEN `apply(&ctx)` is called
- THEN sysctl override file is written and backup recorded

### Requirement: Error Path Coverage

Control tests MUST cover error scenarios: file not found, permission denied, and malformed content. Each error scenario MUST verify that `ControlError` is returned (not panic).

#### Scenario: Control check with missing config file

- GIVEN `with_paths()` fixture directory is empty (no config file)
- WHEN `control.check(&ctx)` is called
- THEN it returns `Ok(CheckResult)` (not error) — missing files are non-compliant, not errors

#### Scenario: Control apply with permission denied

- GIVEN `with_paths()` fixture file has read-only permissions
- WHEN `control.apply(&ctx)` is called
- THEN it returns `Err(ControlError)` (not panic)

### Requirement: Coverage Threshold

The workspace MUST achieve ≥80% line coverage as measured by `cargo-tarpaulin`. Coverage MUST be computed per-crate and as a workspace aggregate. Controls without `with_paths()` support that require root MUST be excluded via `#[cfg(target_os = "linux")]` or tarpaulin exclude.

#### Scenario: Tarpaulin reports ≥80% coverage

- GIVEN all test suites pass
- WHEN `cargo tarpaulin --workspace` is executed
- THEN the reported line coverage is ≥80%