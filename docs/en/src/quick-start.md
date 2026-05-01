# Quick Start

This guide will help you run your first audit and apply your first hardening profile.

## Run an Audit

An audit checks your system against a profile without making any changes:

```bash
vallumix audit --profile web
```

This will output a compliance report to stdout. To save it to a file:

```bash
vallumix audit --profile web --report html --output report.html
```

## Preview Changes with Dry-Run

Before applying any hardening, preview what would change:

```bash
sudo vallumix apply --profile web --dry-run
```

## Apply Hardening

Once you are satisfied with the preview, apply the profile:

```bash
sudo vallumix apply --profile web
```

Vallumix will create a backup session automatically, allowing you to rollback later if needed.

## List Available Controls

To see all controls included in a profile:

```bash
vallumix list --profile database
```

## Rollback a Session

If you need to undo changes:

```bash
vallumix rollback --session <session-id>
```

You can find session IDs in the output of the apply command or by checking the backup directory.
