# backup-manager Specification

## Purpose

Expand the `BackupManager` from create-and-restore-only to support session tracking, listing, cleanup/pruning, and rollback by control-ID or session, per PRD §5.4 CU-04 and PRD §3.1.

## Requirements

### Requirement: Session-Based Backup Tracking

The `BackupManager` MUST support session-based backup tracking where each `apply` or `rollback` invocation creates a session timestamp. Backups MUST be organized under `/var/backups/vallumix/<timestamp>/` and MUST record session metadata including timestamp, profile name, and control IDs affected.

#### Scenario: Session directory created on apply

- GIVEN `BackupManager` is initialized with `/var/backups/vallumix`
- WHEN a session starts at timestamp `2026-04-30T14:30:00Z`
- THEN a directory `/var/backups/vallumix/20260430143000/` is created

#### Scenario: Session metadata records profile and controls

- GIVEN an apply session for profile "web" with controls ["1.1.1.1", "5.2.4"]
- WHEN the session completes
- THEN a `session.json` file is written containing the profile name, control IDs, and timestamp

### Requirement: List Backups Operation

The `BackupManager` MUST provide a `list()` method that returns all backup sessions, ordered by timestamp descending, with metadata (timestamp, profile, control count).

#### Scenario: List returns sessions in reverse chronological order

- GIVEN backup sessions exist at timestamps `20260430140000` and `20260430150000`
- WHEN `backup_mgr.list()` is called
- THEN it returns sessions ordered `[150000, 140000]` (newest first)

#### Scenario: List with no backups returns empty vec

- GIVEN no backup directories exist
- WHEN `backup_mgr.list()` is called
- THEN it returns an empty `Vec<SessionInfo>`

### Requirement: Rollback by Control ID

The `BackupManager` MUST support `rollback_by_control(control_id)` that finds the most recent backup for a specific control and restores it. If no backup exists for the control, it MUST return an error.

#### Scenario: Rollback specific control restores its file

- GIVEN control "5.2.4" was applied and backed up
- WHEN `backup_mgr.rollback_by_control("5.2.4")` is called
- THEN the original file for "5.2.4" is restored from the most recent backup

#### Scenario: Rollback nonexistent control returns error

- GIVEN no backup exists for control "9.9.9"
- WHEN `backup_mgr.rollback_by_control("9.9.9")` is called
- THEN it returns `Err(ControlError::BackupNotFound)`

### Requirement: Rollback by Session

The `BackupManager` MUST support `rollback_session(session_id)` that restores all controls from a specific session. This enables the CLI `rollback` command without `--control-id` to restore an entire apply run.

#### Scenario: Rollback entire session restores all controls

- GIVEN session `20260430150000` applied controls ["1.1.1.1", "5.2.4"]
- WHEN `backup_mgr.rollback_session("20260430150000")` is called
- THEN both control backups are restored in order

### Requirement: Prune Old Backups

The `BackupManager` MUST provide a `prune(keep_count)` method that removes the oldest backup sessions, keeping only the N most recent. This prevents unbounded disk usage.

#### Scenario: Prune keeps only 5 most recent sessions

- GIVEN 8 backup sessions exist
- WHEN `backup_mgr.prune(5)` is called
- THEN the 3 oldest sessions are deleted and 5 remain

#### Scenario: Prune with fewer sessions than keep_count

- GIVEN 3 backup sessions exist
- WHEN `backup_mgr.prune(5)` is called
- THEN no sessions are deleted (3 < 5)

### Requirement: Backup Integrity Verification

The `BackupManager` SHOULD provide a `verify()` method that checks backup files exist and have non-zero size. Corrupted or missing backups MUST be reported.

#### Scenario: Verify detects missing backup file

- GIVEN a backup entry references `/var/backups/vallumix/.../sshd_config` which has been deleted
- WHEN `backup_mgr.verify()` is called
- THEN it returns a list including the missing file as an integrity failure

## Acceptance Criteria

- [ ] Session-based organization under `/var/backups/vallumix/<timestamp>/`
- [ ] `list()` returns sessions ordered newest-first
- [ ] `rollback_by_control(id)` restores most recent backup for a control
- [ ] `rollback_session(id)` restores all controls from a session
- [ ] `prune(keep_count)` removes old sessions beyond the limit
- [ ] `verify()` detects corrupted or missing backup files