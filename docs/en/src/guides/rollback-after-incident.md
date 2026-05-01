# Rollback After Incident (UC-04)

This guide describes how to respond when a hardening control applies a change that breaks a critical service. It covers both rolling back a specific control and rolling back an entire session.

## Scenario

- **System:** Ubuntu 24.04 web server with Nginx and PHP-FPM.
- **Action:** `vallumix apply --profile web` was executed during a maintenance window.
- **Symptom:** After execution, users report 502 Bad Gateway errors. PHP-FPM cannot write to `/tmp` to manage sessions.
- **Hypothesis:** A filesystem control mounted `/tmp` with `noexec`, which is correct from the CIS perspective but breaks PHP-FPM functionality that uses session files in `/tmp`.

## Step 1: Identify the Responsible Control

The execution report is your starting point. Locate the generated file:

```bash
ls -lt /var/backups/vallumix/
# Identify the timestamp of the most recent session
# Example: 2026-05-01T09-15-33
```

If you still have the JSON report from the execution:

```bash
jq '.controls[] | select(.status == "Remediated") | {id, description, severity}' /tmp/hardening-report.json
```

In this scenario, you identify that control `1.1.2.2` (Ensure nodev option set on /tmp partition) and `1.1.2.3` (Ensure nosuid option set on /tmp partition) were applied, and possibly `1.1.2.4` (Ensure noexec option set on /tmp partition) is the direct cause of the problem.

## Step 2: Verify the Backups

```bash
ls /var/backups/vallumix/2026-05-01T09-15-33/
# Should contain: manifest.json, checksums.sha256, and subdirectories by control

# Verify backup integrity
cd /var/backups/vallumix/2026-05-01T09-15-33
sha256sum -c checksums.sha256
# All should report OK
```

```danger
If `sha256sum -c` reports FAIL for any backup file, DO NOT execute rollback for that control. The backup may be corrupt and restoring it could leave the system in an inconsistent state. Contact support or restore manually from another medium.
```

## Step 3: Rollback the Specific Control

If you are certain that only `1.1.2.4` caused the problem, revert it individually:

```bash
sudo vallumix rollback --control-id 1.1.2.4
```

### Expected Output

```text
[INFO] Restoring backup for control 1.1.2.4
[INFO] Source: /var/backups/vallumix/2026-05-01T09-15-33/1.1.2.4/fstab.bak
[INFO] Target: /etc/fstab
[INFO] Integrity check: SHA-256 OK
[INFO] Remounting /tmp with original options
[INFO] Post-check: /tmp is now compliant with pre-hardening state
[SUCCESS] Rollback of control 1.1.2.4 completed
```

## Step 4: Verify Service Restoration

```bash
# Verify /tmp has the original options
findmnt /tmp
# Should show: /tmp tmpfs tmpfs rw,nosuid,nodev (without noexec)

# Restart PHP-FPM so it recreates its session files
sudo systemctl restart php8.3-fpm

# Verify Nginx responds correctly
curl -I https://localhost/
# Should return HTTP 200
```

## Step 5: Alternative — Full Session Rollback

If you are not sure which control caused the problem, or if several filesystem controls interacted badly, revert the entire session:

```bash
# Rollback the latest session automatically
sudo vallumix rollback --session last

# Or by specific timestamp
sudo vallumix rollback --session 2026-05-01T09-15-33
```

Session rollback restores all files in reverse order to how they were modified, minimizing circular dependencies between configurations.

```warning
Full session rollback reverts ALL applied controls, not just the problematic one. This can leave the server with a weaker security posture than it had before hardening. Only use it when individual rollback is not viable.
```

## Step 6: Document the Exception

After resolving the incident, document the problematic control for future executions:

1. Edit the TOML profile (`/etc/vallumix/profiles/web.toml` or the local profile) and add the control to the exclusion list.
2. Run a new dry-run to confirm that the control is skipped.
3. Report the conflict to the Vallumix project if you believe the control should be smarter (for example, detecting if `/tmp` is being used by active processes before applying `noexec`).

```tip
Include a post-hardening functional validation step in your runbooks: after `vallumix apply`, run a smoke test of your critical services (health checks, main endpoints, database connections). Detect problems in minutes, not hours.
```
