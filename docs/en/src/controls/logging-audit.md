# Logging and Auditing Controls

The logging domain (CIS section 4.1.x) contains 11 controls that ensure events on the system are captured, retained, and protected from tampering. Without reliable logs, detecting intrusion, reconstructing incidents, and satisfying compliance requirements is impossible. This domain covers rsyslog, systemd-journald, auditd, log rotation, and log file permissions.

## `4.1.1.1` — Ensure rsyslog is installed

**What apply does:**
Verifies that the `rsyslog` package is installed. If missing, it does not auto-install (to avoid unexpected package manager changes) but reports the control as non-compliant.

**Manual verification:**

```bash
rpm -q rsyslog 2>/dev/null || dpkg -l rsyslog
```

**Security justification:**
rsyslog is the standard syslog daemon on most Linux distributions. It is the foundation for centralized log collection, SIEM integration, and forensic analysis. A missing rsyslog installation means many system events are never persisted to disk.

## `4.1.1.2` — Ensure rsyslog is configured

**What apply does:**
Checks `/etc/rsyslog.conf` and `/etc/rsyslog.d/*.conf` for standard logging rules (auth, authpriv, cron, kern, mail, user). If rules are missing, Vallumix adds them to `/etc/rsyslog.d/50-default.conf`.

**Manual verification:**

```bash
grep -r 'auth,authpriv' /etc/rsyslog.d/
grep -r 'cron\.' /etc/rsyslog.d/
```

**Security justification:**
Without proper rsyslog rules, authentication failures, cron job output, and kernel messages may be silently dropped. This control ensures the minimum set of rules recommended by CIS is present.

## `4.1.1.3` — Ensure rsyslog file permissions

**What apply does:**
Sets permissions on rsyslog configuration files to `640` and ownership to `root:adm` (or distribution-appropriate group).

**Manual verification:**

```bash
stat -c '%a %U:%G' /etc/rsyslog.conf
stat -c '%a %U:%G' /etc/rsyslog.d/
```

**Security justification:**
Log configuration files can redirect, suppress, or reroute log streams. If writable by non-root users, an attacker could disable logging for their activities or redirect logs to `/dev/null`.

## `4.1.2.1` — Ensure journald is configured

**What apply does:**
Verifies `/etc/systemd/journald.conf` contains `Storage=persistent` and `ForwardToSyslog=yes`. If missing, it adds them under the `[Journal]` section.

**Manual verification:**

```bash
grep -E '^Storage=|^ForwardToSyslog=' /etc/systemd/journald.conf
```

**Security justification:**
By default, journald stores logs in volatile memory (`/run/log/journal`). A persistent storage setting ensures logs survive reboots, which is essential for post-reboot incident investigation.

## `4.1.2.2` — Ensure journald override exists

**What apply does:**
Creates `/etc/systemd/journald.conf.d/99-vallumix.conf` with hardened settings (`Compress=yes`, `SystemMaxUse=500M`, `MaxFileSec=1week`).

**Manual verification:**

```bash
cat /etc/systemd/journald.conf.d/99-vallumix.conf
```

**Security justification:**
An override file prevents distribution upgrades from overwriting hardening settings. Limiting disk usage (`SystemMaxUse`) protects against log-filling denial-of-service attacks.

## `4.1.3.1` — Ensure auditd is installed

**What apply does:**
Verifies that `auditd` and `audispd-plugins` are installed. If missing, reports non-compliant.

**Manual verification:**

```bash
rpm -q audit 2>/dev/null || dpkg -l auditd
```

**Security justification:**
auditd captures security-relevant events at the kernel level: file access, syscall invocation, user logins, and privilege escalations. It is the backbone of Linux audit trails and is required by most compliance frameworks.

## `4.1.3.2` — Ensure auditd is configured

**What apply does:**
Checks that `/etc/audit/auditd.conf` contains `log_group = root` (or `adm`) and `max_log_file_action = ROTATE`.

**Manual verification:**

```bash
grep '^max_log_file_action' /etc/audit/auditd.conf
grep '^log_group' /etc/audit/auditd.conf
```

**Security justification:**
Proper auditd configuration ensures logs rotate automatically and are readable only by authorized administrators. Without rotation, the audit partition can fill up and crash the daemon.

## `4.1.4.1` — Ensure audit identity rules exist

**What apply does:**
Adds audit rules to monitor identity file changes: `/etc/group`, `/etc/passwd`, `/etc/gshadow`, `/etc/shadow`, `/etc/security/opasswd`.

**Manual verification:**

```bash
auditctl -l | grep -E '(/etc/passwd|/etc/shadow|/etc/group|/etc/gshadow)'
```

**Security justification:**
Unauthorized changes to identity files are a clear indicator of account creation, privilege escalation, or backdoor installation. Monitoring these files allows real-time detection of tampering.

## `4.1.4.2` — Ensure audit login events

**What apply does:**
Adds audit rules for `/var/log/lastlog`, `/var/run/faillock`, and login binaries (`/usr/bin/login`, `/usr/bin/su`).

**Manual verification:**

```bash
auditctl -l | grep -E '(lastlog|faillock|/usr/bin/login|/usr/bin/su)'
```

**Security justification:**
Login events are the primary data source for brute-force detection and access reviews. Without audit rules, failed and successful logins may only exist in short-lived buffers.

## `4.1.4.3` — Ensure audit session events

**What apply does:**
Adds audit rules for session initiation (`/usr/bin/sudo`, `/usr/bin/sudoedit`, `/usr/bin/ssh`) and termination.

**Manual verification:**

```bash
auditctl -l | grep -E '(/usr/bin/sudo|/usr/bin/ssh)'
```

**Security justification:**
Session-level auditing captures who executed privileged commands and when. This is critical for non-repudiation and for reconstructing the timeline of an incident.

## `4.1.7` — Ensure logrotate is configured

**What apply does:**
Verifies that `/etc/logrotate.conf` exists and that `/var/log` is covered by a logrotate rule with rotation period `weekly` or shorter and retention of at least 4 rotations.

**Manual verification:**

```bash
grep -E '^weekly|^rotate' /etc/logrotate.conf
ls /etc/logrotate.d/
```

**Security justification:**
Without log rotation, logs grow indefinitely and eventually exhaust disk space. Attackers can exploit this by flooding logs to cover their tracks or cause a denial of service. Rotation also bounds the retention window, which helps meet data-protection regulations.
