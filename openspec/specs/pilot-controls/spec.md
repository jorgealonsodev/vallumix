# pilot-controls Specification

## Purpose

Implement 5 pilot CIS controls spanning all 6 CIS domains, each implementing the `Control` trait with idempotent `check()`, `apply()` with backup, and `rollback()`.

## Requirements

### Requirement: disable_cramfs Control (CIS 1.1.1.1)

The `disable_cramfs` control MUST implement the `Control` trait with `id()` returning `"1.1.1.1"`, `severity()` returning `Severity::Low`, `applicable_distros()` returning all supported distros, and `category()` returning `Category::Filesystem`. `check()` MUST verify the `cramfs` filesystem type is not loaded (via `/proc/filesystems` or `modprobe --show`). `apply()` MUST add `install cramfs /bin/true` to `/etc/modprobe.d/` config. `rollback()` MUST remove the config entry.

#### Scenario: check returns Compliant when cramfs is absent

- GIVEN `cramfs` is not listed in `/proc/filesystems`
- WHEN `disable_cramfs.check(&ctx)` is called
- THEN it returns `Ok(CheckResult { status: Compliant, evidence: "cramfs not loaded", .. })`

#### Scenario: check returns NonCompliant when cramfs is loaded

- GIVEN `cramfs` appears in `/proc/filesystems`
- WHEN `disable_cramfs.check(&ctx)` is called
- THEN it returns `Ok(CheckResult { status: NonCompliant, evidence, .. })`

#### Scenario: apply is idempotent — running twice produces same result

- GIVEN `cramfs` config already exists in `/etc/modprobe.d/`
- WHEN `disable_cramfs.apply(&ctx)` is called
- THEN no duplicate entries are created and `ApplyStatus::AlreadyCompliant` is returned

#### Scenario: disable_cramfs returns Filesystem category

- GIVEN `DisableCramfs` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Filesystem`

### Requirement: disable_avahi Control (CIS 2.x)

The `disable_avahi` control MUST implement the `Control` trait with `id()` returning a CIS 2.x ID, `severity()` returning `Severity::Medium`, and `category()` returning `Category::Services`. `check()` MUST verify the Avahi daemon service is disabled and stopped. `apply()` MUST disable and stop the service using the distro-appropriate service manager. `rollback()` MUST re-enable the service.

#### Scenario: check detects Avahi is running

- GIVEN Avahi daemon is active and enabled
- WHEN `disable_avahi.check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence containing the service status

#### Scenario: apply disables Avahi and creates backup

- GIVEN Avahi daemon is active
- WHEN `disable_avahi.apply(&ctx)` is called
- THEN Avahi is disabled, a backup is created, and `ApplyResult` contains the backup path

#### Scenario: disable_avahi returns Services category

- GIVEN `DisableAvahi` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Services`

### Requirement: sysctl_ip_forwarding Control (CIS 3.x)

The `sysctl_ip_forwarding` control MUST implement `Control` with `id()` returning a CIS 3.x ID, `severity()` returning `Severity::Medium`, and `category()` returning `Category::Network`. `check()` MUST read `/proc/sys/net/ipv4/ip_forward` and verify value is `0`. `apply()` MUST set `net.ipv4.ip_forward = 0` in a sysctl drop-in file AND via the running kernel. `rollback()` MUST restore the previous value.

#### Scenario: check returns Compliant when ip_forward is 0

- GIVEN `/proc/sys/net/ipv4/ip_forward` contains `0`
- WHEN `sysctl_ip_forwarding.check(&ctx)` is called
- THEN it returns `CheckResult { status: Compliant, evidence: "ip_forward=0", .. }`

#### Scenario: apply writes sysctl config with backup

- GIVEN `ip_forward` is currently `1`
- WHEN `sysctl_ip_forwarding.apply(&ctx)` is called
- THEN a drop-in file is created, the running kernel value is set to `0`, and a backup is recorded

#### Scenario: sysctl_ip_forwarding returns Network category

- GIVEN `SysctlIpForwarding` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Network`

### Requirement: ssh_disable_root_login Control (CIS 5.2.4)

The `ssh_disable_root_login` control MUST implement `Control` with `id()` returning `"5.2.4"`, `severity()` returning `Severity::High`, `applicable_distros()` returning all supported distros, and `category()` returning `Category::Ssh`. `check()` MUST parse `/etc/ssh/sshd_config` (and drop-in dirs) for `PermitRootLogin no`. `apply()` MUST add or set `PermitRootLogin no` in the config. `rollback()` MUST restore the original value.

#### Scenario: check returns Compliant when PermitRootLogin is no

- GIVEN `/etc/ssh/sshd_config` contains `PermitRootLogin no`
- WHEN `ssh_disable_root_login.check(&ctx)` is called
- THEN it returns `Compliant` with evidence containing the matched line

#### Scenario: check handles commented-out directive

- GIVEN `sshd_config` has `#PermitRootLogin yes` (commented)
- WHEN `check()` is called
- THEN it returns `NonCompliant` — comments MUST NOT count as compliant

#### Scenario: apply edits config and backs up original

- GIVEN `sshd_config` does not contain `PermitRootLogin no`
- WHEN `apply(&ctx)` is called
- THEN the file is updated, a backup of the original is saved, and `ApplyResult` contains the backup path

#### Scenario: ssh_disable_root_login returns Ssh category

- GIVEN `SshDisableRootLogin` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Ssh`

### Requirement: ensure_perms_passwd Control (CIS 6.x)

The `ensure_perms_passwd` control MUST implement `Control` with `id()` returning a CIS 6.x ID, `severity()` returning `Severity::High`, and `category()` returning `Category::Maintenance`. `check()` MUST verify that `/etc/passwd` has permissions `0644` (owner rw, group r, others r). `apply()` MUST set permissions to `0644`. `rollback()` MUST restore original permissions from backup.

#### Scenario: check returns Compliant when perms are 0644

- GIVEN `/etc/passwd` has mode `0644`
- WHEN `ensure_perms_passwd.check(&ctx)` is called
- THEN it returns `Compliant` with evidence `"mode=0644"`

#### Scenario: apply corrects permissions to 0644

- GIVEN `/etc/passwd` has mode `0600`
- WHEN `ensure_perms_passwd.apply(&ctx)` is called
- THEN permissions are set to `0644` and a backup records the original mode

#### Scenario: ensure_perms_passwd returns Maintenance category

- GIVEN `EnsurePermsPasswd` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Maintenance`
