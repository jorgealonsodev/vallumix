# System Maintenance Controls

The system maintenance domain (CIS section 6.1.x) contains 8 controls focused on file permissions, ownership, and periodic audit of system executables. These controls do not change running services or network behavior; instead, they ensure that critical files are not readable or writable by unauthorized users, and that no unexpected SUID/SGID binaries exist on the system.

## `6.1.1` — Ensure permissions on /etc/passwd

**What apply does:**
Sets `/etc/passwd` to mode `644` and ownership `root:root`.

**Manual verification:**

```bash
stat -c '%a %U:%G' /etc/passwd
```

Expected: `644 root:root`.

**Security justification:**
`/etc/passwd` contains usernames, UIDs, home directories, and default shells. While it does not contain password hashes (those are in `/etc/shadow`), it leaks user enumeration data and shell information that aids attackers in crafting targeted exploits.

## `6.1.2` — Ensure permissions on /etc/shadow

**What apply does:**
Sets `/etc/shadow` to mode `000` or `640` and ownership `root:shadow`.

**Manual verification:**

```bash
stat -c '%a %U:%G' /etc/shadow
```

**Security justification:**
`/etc/shadow` stores password hashes, password aging, and account lockout status. If readable by non-root users, an attacker can extract hashes for offline cracking with tools like Hashcat or John the Ripper.

## `6.1.3` — Ensure permissions on /etc/group

**What apply does:**
Sets `/etc/group` to mode `644` and ownership `root:root`.

**Manual verification:**

```bash
stat -c '%a %U:%G' /etc/group
```

**Security justification:**
Group membership reveals which users have access to shared resources. Keeping this file world-readable is generally acceptable, but it must not be writable by anyone other than root.

## `6.1.4` — Ensure permissions on /etc/gshadow

**What apply does:**
Sets `/etc/gshadow` to mode `000` or `640` and ownership `root:shadow`.

**Manual verification:**

```bash
stat -c '%a %U:%G' /etc/gshadow
```

**Security justification:**
`/etc/gshadow` stores group passwords and administrator lists. Like `/etc/shadow`, it must be protected from read access to prevent offline attacks on group credentials.

## `6.1.5` — Audit world-writable files

**What apply does:**
Runs `find` to locate files with mode `o+w` that do not have the sticky bit set on their directory. The control reports the list but does not auto-remediate, because world-writable files may be intentional (e.g., shared spool directories).

**Manual verification:**

```bash
find / -xdev -type f -perm -002 ! -perm -1000 -exec ls -l {} \; 2>/dev/null
```

**Security justification:**
World-writable files allow any user on the system to modify them. If such a file is executed by root or another privileged process, an attacker can inject malicious content and escalate privileges.

```tip
Review the list produced by this control carefully. Remove world-writable permissions where they are not required, and ensure the sticky bit is set on shared directories (e.g., `/tmp`).
```

## `6.1.6` — Audit SUID and SGID executables

**What apply does:**
Runs `find` to locate all files with the setuid or setgid bit set. The control reports them but does not remove the bits automatically, because many are required for basic system functionality (e.g., `/usr/bin/passwd`, `/usr/bin/sudo`).

**Manual verification:**

```bash
find / -xdev -type f \( -perm -4000 -o -perm -2000 \) -exec ls -l {} \; 2>/dev/null
```

**Security justification:**
SUID/SGID binaries run with the privileges of their owner or group. A vulnerability in such a binary is a direct privilege-escalation path. Regular audits ensure no unauthorized programs (e.g., compiled exploitation tools) have been given elevated permissions.

## `6.1.7` — Audit unowned files

**What apply does:**
Finds files whose UID or GID does not resolve to a valid user or group in `/etc/passwd` and `/etc/group`.

**Manual verification:**

```bash
find / -xdev -nouser -o -nogroup 2>/dev/null
```

**Security justification:**
Files owned by deleted users are often remnants of compromised accounts or orphaned attack tools. They may also indicate a misconfigured package installation. Cleaning them up reduces clutter and removes potential backdoors.

## `6.1.8` — Audit duplicate IDs

**What apply does:**
Checks `/etc/passwd`, `/etc/group`, `/etc/shadow`, and `/etc/gshadow` for duplicate UIDs, GIDs, or usernames. Reports duplicates but does not auto-fix.

**Manual verification:**

```bash
awk -F: '{print $3}' /etc/passwd | sort | uniq -d
awk -F: '{print $3}' /etc/group | sort | uniq -d
```

**Security justification:**
Duplicate UIDs can cause permission collisions: two different users may unintentionally share the same filesystem identity, allowing one to access the other's files.

## `6.1.9` — Ensure cron permissions

**What apply does:**
Sets `/etc/crontab`, `/etc/cron.d/`, `/etc/cron.daily/`, `/etc/cron.weekly/`, and `/etc/cron.monthly/` to mode `700` or `750` and ownership `root:root`.

**Manual verification:**

```bash
stat -c '%a %U:%G' /etc/crontab
stat -c '%a %U:%G' /etc/cron.d
```

**Security justification:**
Cron is a persistent execution mechanism. If an attacker can write to a system cron directory, they achieve persistent root access with every scheduled interval. Restricting these paths to root is essential for integrity.
