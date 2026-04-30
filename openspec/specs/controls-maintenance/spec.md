# controls-maintenance Specification

## Purpose

Implement CIS 6.x file-permission and system-integrity controls, each implementing the `Control` trait with `category()` returning `Category::Maintenance`.

## Requirements

### Requirement: System File Permission Controls

The system MUST provide `ensure_perms_shadow` (CIS 6.1.2) checking `/etc/shadow` mode `0600`, `ensure_perms_group` (CIS 6.1.3) checking `/etc/group` mode `0644`, and `ensure_perms_gshadow` (CIS 6.1.4) checking `/etc/gshadow` mode `0600`. The existing `ensure_perms_passwd` (CIS 6.1.1) pilot MUST gain `category()` returning `Category::Maintenance`.

#### Scenario: shadow file with wrong permissions

- GIVEN `/etc/shadow` has mode `0640`
- WHEN `EnsurePermsShadow::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence `"mode=0640, expected=0600"`

#### Scenario: group file with correct permissions

- GIVEN `/etc/group` has mode `0644`
- WHEN `EnsurePermsGroup::check(&ctx)` is called
- THEN it returns `Compliant` with evidence `"mode=0644"`

### Requirement: World-Writable Files Control

The system MUST provide `audit_world_writable` (CIS 6.1.5) that scans for world-writable files. `check()` MUST find files with `o+w` permission. `apply()` SHOULD NOT automatically fix world-writable files — it MUST log a warning and return `ApplyStatus::Skipped` as some world-writable files are intentional.

#### Scenario: world-writable files found

- GIVEN `/tmp/testfile` has mode `0777`
- WHEN `AuditWorldWritable::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence listing world-writable files

#### Scenario: apply skips world-writable remediation

- GIVEN world-writable files exist
- WHEN `apply(&ctx)` is called
- THEN it returns `ApplyStatus::Skipped` with message advising manual review

### Requirement: SUID/SGID Audit Control

The system MUST provide `audit_suid_sgid` (CIS 6.1.6) that discovers files with SUID or SGID bits set outside an allowed list. `apply()` MUST return `Skipped` — this is an audit-only control.

#### Scenario: unexpected SUID binary found

- GIVEN `/usr/local/bin/custom` has SUID bit set
- WHEN `AuditSuidSgid::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence listing unauthorized SUID files

#### Scenario: SUID audit is read-only

- GIVEN unauthorized SUID files exist
- WHEN `apply(&ctx)` is called
- THEN it returns `Skipped` — SUID removal requires manual review

### Requirement: Unowned Files and Duplicate ID Controls

The system MUST provide `audit_unowned_files` (CIS 6.1.7) checking for files with no owner/group, and `audit_duplicate_ids` (CIS 6.1.8) checking for duplicate UIDs or GIDs in `/etc/passwd` and `/etc/group`.

#### Scenario: unowned file detected

- GIVEN a file exists with UID 65534 (nobody)
- WHEN `AuditUnownedFiles::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence listing unowned files

#### Scenario: duplicate UIDs found

- GIVEN `/etc/passwd` has two entries with UID `0`
- WHEN `AuditDuplicateIds::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence `"duplicate UID 0"`

### Requirement: Cron Permission Controls

The system SHOULD provide `ensure_cron_perms` (CIS 6.1.9) verifying `/etc/cron.d`, `/etc/cron.daily`, `/etc/cron.hourly`, `/etc/cron.weekly`, `/etc/cron.monthly` have appropriate permissions. `severity()` MUST return `Severity::Medium`.

#### Scenario: cron.daily with wrong permissions

- GIVEN `/etc/cron.daily` has mode `0777`
- WHEN `EnsureCronPerms::check(&ctx)` is called
- THEN it returns `NonCompliant`

### Requirement: Existing Pilot Control Category

The `ensure_perms_passwd` control already exists. It MUST receive `category()` returning `Category::Maintenance`.

#### Scenario: ensure_perms_passwd returns Maintenance category

- GIVEN `EnsurePermsPasswd` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Maintenance`

### Requirement: Maintenance Controls with_paths Pattern

Every maintenance control MUST implement `with_paths()` for testability, allowing override of `/etc/` paths, scan directories, and permission targets.

#### Scenario: with_paths overrides passwd path

- GIVEN `EnsurePermsShadow::with_paths(MockPaths { shadow_file })`
- WHEN `check(&ctx)` is called
- THEN it checks permissions on the mock shadow file

## Acceptance Criteria

- [ ] 6-8 maintenance controls implemented with CIS IDs
- [ ] Audit-only controls return `Skipped` from `apply()`
- [ ] All controls implement `Control` including `category()`
- [ ] `with_paths()` pattern for every maintenance control
- [ ] Idempotent apply for fixable controls; `Skipped` for audit-only