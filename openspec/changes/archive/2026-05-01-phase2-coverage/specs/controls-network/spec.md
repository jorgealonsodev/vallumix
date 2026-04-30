# controls-network Specification

## Purpose

Implement CIS 3.x network kernel parameter controls via sysctl, each implementing the `Control` trait with `category()` returning `Category::Network`.

## Requirements

### Requirement: IP Forwarding Control

The system MUST provide `sysctl_disable_ip_forwarding` (CIS 3.1.1) that verifies and sets `net.ipv4.ip_forward = 0`. This control already exists as a pilot; it MUST receive `category()` returning `Category::Network`.

#### Scenario: Existing ip_forwarding control returns Network category

- GIVEN `SysctlIpForwarding` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Network`

### Requirement: ICMP and Redirect Controls

The system MUST provide controls for: `sysctl_disable_send_redirects` (CIS 3.1.2) setting `net.ipv4.conf.all.send_redirects = 0` and `net.ipv4.conf.default.send_redirects = 0`; `sysctl_disable_accept_redirects` (CIS 3.2.2) for `accept_redirects`; `sysctl_disable_source_route` (CIS 3.2.1) for `accept_source_route`.

#### Scenario: disable_send_redirects checks all interfaces

- GIVEN `net.ipv4.conf.all.send_redirects` is `1`
- WHEN `SysctlDisableSendRedirects::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence showing the current value

#### Scenario: disable_source_route checks both all and default

- GIVEN `net.ipv4.conf.all.accept_source_route = 0` but `net.ipv4.conf.default.accept_source_route = 1`
- WHEN `check(&ctx)` is called
- THEN it returns `NonCompliant` — both `all` and `default` MUST be `0`

### Requirement: Reverse Path Filtering Control

The system MUST provide `sysctl_enable_rp_filter` (CIS 3.2.6) that sets `net.ipv4.conf.all.rp_filter = 1` and `net.ipv4.conf.default.rp_filter = 1`. `severity()` MUST return `Severity::Medium`.

#### Scenario: rp_filter applies sysctl drop-in

- GIVEN `rp_filter` is `0`
- WHEN `apply(&ctx)` is called
- THEN a drop-in file is created at `/etc/sysctl.d/99-vallumix.conf` and the running kernel value is updated

### Requirement: TCP SYN Cookies and Broadcast Logging

The system MUST provide `sysctl_enable_syncookies` (CIS 3.2.7) setting `net.ipv4.tcp_syncookies = 1` and `sysctl_disable_icmp_redirects` (CIS 3.2.3) for `icmp_redirects`. `severity()` MUST be `Severity::Medium`.

#### Scenario: syncookies check with correct value

- GIVEN `/proc/sys/net/ipv4/tcp_syncookies` contains `1`
- WHEN `check(&ctx)` is called
- THEN it returns `Compliant` with evidence `"tcp_syncookies=1"`

#### Scenario: syncookies rollback restores original value

- GIVEN `apply()` has set `tcp_syncookies = 1` from `0`
- WHEN `rollback(&ctx, &backup)` is called
- THEN the original value `0` is restored from backup

### Requirement: Sysctl with_paths Testability

Every network sysctl control MUST implement `with_paths()` that allows injecting mock `/proc/sys/` and `/etc/sysctl.d/` paths for testing, consistent with the pilot pattern.

#### Scenario: with_paths overrides proc and sysctl paths

- GIVEN `SysctlDisableSendRedirects::with_paths(MockPaths { proc_sys, sysctl_dir })`
- WHEN `check(&ctx)` is called
- THEN it reads from mock paths instead of `/proc/sys/net/ipv4/`

### Requirement: Firewall Rules Control

The system SHOULD provide `configure_firewalld` (CIS 3.3.x) that verifies a firewall is active (firewalld or nftables) and default zone drops incoming traffic. `check()` MUST detect which firewall backend is in use.

#### Scenario: firewalld active with drop default

- GIVEN `firewalld` is active with default zone set to drop
- WHEN `ConfigureFirewalld::check(&ctx)` is called
- THEN it returns `Compliant`

## Acceptance Criteria

- [ ] 8-10 network controls implemented with CIS IDs
- [ ] All controls implement `Control` trait including `category()`
- [ ] Sysctl controls write drop-in files, not direct `/etc/sysctl.conf` edits
- [ ] `with_paths()` pattern for every sysctl control
- [ ] Both `all` and `default` interface variants checked where required