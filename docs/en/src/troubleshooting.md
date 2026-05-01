# Troubleshooting

This page collects the most frequent issues reported by Vallumix users, along with their probable causes and solutions.

## Permission Denied

**Symptom:**

```text
Error: privilege check failed — effective UID is 1000, root required
```

**Cause:** Vallumix requires root privileges to read protected configuration files (`/etc/shadow`, `/etc/ssh/sshd_config`) and to modify system configurations.

**Solution:**

```bash
sudo vallumix apply --profile web
```

If you run Vallumix inside a container or virtual machine without root access, elevate privileges before executing it. Do not use `chmod 777` as a workaround on system files.

## Unsupported Distribution

**Symptom:**

```text
Error: distribution "Fedora 40" is not supported
Supported: debian-12, ubuntu-22.04, ubuntu-24.04, rhel-9, rocky-9, almalinux-9
```

**Cause:** Vallumix detects the distribution by reading `/etc/os-release`. If the distro is not in the supported list, it aborts to avoid applying controls that might not be compatible.

**Solution:**

- If it is a Debian/RHEL derivative that should work, force detection with the `VALLUMIX_DISTRO_OVERRIDE` environment variable (for testing only, no guaranteed support).
- If you need official support for a new distribution, open an issue in the repository.
- For containers or unconventional environments, evaluate whether Vallumix is the appropriate tool (see "Out of Scope" in the PRD).

## SSH Access Blocked After Hardening

**Symptom:** After applying the `bastion` profile or aggressive SSH controls, you cannot connect via SSH.

**Cause:** The bastion profile disables `PasswordAuthentication` and `PermitRootLogin`. If you were connecting by password or as root, you lost access.

**Prevention:**

```bash
# BEFORE applying bastion, verify key access
ssh -o PasswordAuthentication=no admin@server
```

**Recovery:**

- Access the physical console or the cloud administration panel (VNC, serial console, EC2 Instance Connect).
- Manually restore `/etc/ssh/sshd_config` from the backup in `/var/backups/vallumix/<timestamp>/5.2.4/`.
- Restart SSH: `sudo systemctl restart sshd`.

```danger
SSH lockout is the most serious incident that a remote hardening tool can cause. Always verify key access before applying SSH controls. On cloud servers, confirm that you have access to the serial console or VNC as a recovery path.
```

## Insufficient Disk Space for Backups

**Symptom:**

```text
Error: insufficient disk space for backups — 45 MB available, 100 MB required
```

**Cause:** Vallumix verifies free disk space before creating the backup session. If there is not enough space, it aborts to avoid getting halfway through without backups.

**Solution:**

```bash
# Free space in /var
df -h /var
sudo apt autoremove   # Debian/Ubuntu
sudo dnf autoremove   # RHEL/Rocky/Alma
sudo journalctl --vacuum-time=7d
```

If `/var` is on a separate small partition, consider mounting `/var/backups/vallumix` on a larger volume or configuring an alternative path via environment variable (if the project supports it).

## Failed Controls After Application

**Symptom:** A control appears as `Failed` in the report after `apply`.

**Common Causes:**

1. **Service blocks the file:** a daemon keeps the configuration file open with a write lock.
2. **Unexpected configuration:** the file has syntax that the control did not anticipate.
3. **Dependency on another control:** a previous control modified something that this control assumed was constant.

**Solution:**

- Review the log with `--verbose` to see the exact error.
- Verify if a service is blocking the file: `lsof /etc/ssh/sshd_config`.
- Apply the control manually following the CIS recommendation, then re-run Vallumix to confirm.

## Report Not Generated

**Symptom:** Execution finishes but you cannot find the report file.

**Cause:** `--output` did not specify an existing directory or permissions prevent writing.

**Solution:**

```bash
# Make sure the directory exists and is writable
mkdir -p /var/reports/vallumix
sudo vallumix audit --profile web --report html --output /var/reports/vallumix/audit
```

If you do not use `--output`, Vallumix uses `/tmp` by default. Some systems clean `/tmp` on every reboot.
