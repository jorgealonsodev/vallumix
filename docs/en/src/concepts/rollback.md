# Rollback

Vallumix's rollback system allows reverting hardening changes in a granular manner, either by individual control or by complete session. This transforms hardening from a risky operation into a reversible one.

## Backup Architecture

Every time Vallumix applies changes, it creates a versioned backup session in `/var/backups/vallumix/`:

```
/var/backups/vallumix/
├── 2026-04-30T14-22-18/
│   ├── manifest.json          # Session metadata
│   ├── 5.2.4/
│   │   └── sshd_config.bak    # File backed up by control 5.2.4
│   ├── 1.1.1.1/
│   │   └── modprobe.cramfs.bak
│   └── checksums.sha256       # SHA-256 hashes of all backed up files
└── 2026-04-30T15-07-03/
    └── ...
```

Each session contains:

- **Directory with ISO 8601 timestamp:** identifies when it was executed.
- **`manifest.json`:** lists applied controls, distribution, profile used, and the user who ran Vallumix.
- **Subdirectories by control ID:** each control that modified files has its own directory with backup copies.
- **`checksums.sha256`:** cryptographic hash of each backed up file, allowing detection of corruption or tampering.

## Rollback by Control

If you identify that a specific control caused a problem, revert it without affecting the others:

```bash
sudo vallumix rollback --control-id 5.2.4
```

This command:

1. Reads the `manifest.json` of the latest session.
2. Locates the backup for control `5.2.4`.
3. Verifies file integrity via SHA-256.
4. Restores the original file to its location.
5. Executes `check` to confirm the system returned to the previous state.
6. Logs the action and generates a rollback report.

```tip
Rollback by control is the preferred option when you know exactly which change broke something. It is fast, precise, and minimizes the risk of reverting correct controls that have nothing to do with the incident.
```

## Rollback by Session

If you are not sure which control caused the problem, or if several controls interacted unexpectedly, revert an entire session:

```bash
# Latest session automatically
sudo vallumix rollback --session last

# Specific session by timestamp
sudo vallumix rollback --session 2026-04-30T14-22-18
```

Session rollback restores all files backed up in that execution, in reverse order to how they were applied, to correctly handle dependencies between files.

```danger
Session rollback is a destructive operation that reverts multiple changes. Although backups are protected by checksums, a mass restoration can affect system stability if other administrators have made manual changes to the same files between the hardening session and the rollback. Always communicate before executing a rollback in production.
```

## Integrity Verification

Before restoring any file, Vallumix recalculates the SHA-256 hash and compares it with the one recorded in `checksums.sha256`. If they do not match, the rollback aborts for that file and marks it as `IntegrityCheckFailed` in the report.

## Cleaning Up Old Backups

Backups are not deleted automatically. To prevent `/var/backups/vallumix` from growing indefinitely, configure a cron task that keeps only the last N sessions:

```bash
# Keep only the last 10 sessions
0 2 * * * cd /var/backups/vallumix && ls -t | tail -n +11 | xargs rm -rf
```
