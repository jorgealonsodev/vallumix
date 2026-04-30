# controls-ssh Specification

## Purpose

Implement CIS 5.2.x SSH hardening controls that configure `/etc/ssh/sshd_config` and drop-in directories, each implementing the `Control` trait with `category()` returning `Category::Ssh`.

## Requirements

### Requirement: SSH Protocol and Logging Controls

The system MUST provide `ssh_ensure_protocol_2` (CIS 5.2.1) verifying `Protocol 2` or absence of `Protocol` (default is 2), and `ssh_set_loglevel` (CIS 5.2.2) verifying `LogLevel INFO` or `VERBOSE`. `severity()` MUST return `Severity::Medium`.

#### Scenario: protocol 2 is default — no directive is compliant

- GIVEN `sshd_config` has no `Protocol` directive
- WHEN `SshEnsureProtocol2::check(&ctx)` is called
- THEN it returns `Compliant` — OpenSSH defaults to protocol 2

#### Scenario: LogLevel set to DEBUG is non-compliant

- GIVEN `sshd_config` contains `LogLevel DEBUG`
- WHEN `SshSetLogLevel::check(&ctx)` is called
- THEN it returns `NonCompliant` — only INFO and VERBOSE are compliant

### Requirement: SSH Authentication Controls

The system MUST provide `ssh_disable_empty_passwords` (CIS 5.2.3), `ssh_disable_root_login` (already pilot — MUST gain `category()`), and `ssh_max_auth_tries` (CIS 5.2.5) setting `MaxAuthTries` to 4 or less.

#### Scenario: MaxAuthTries exceeds threshold

- GIVEN `sshd_config` contains `MaxAuthTries 6`
- WHEN `SshMaxAuthTries::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence `"MaxAuthTries=6, max=4"`

#### Scenario: Empty passwords disabled — compliant

- GIVEN `sshd_config` contains `PermitEmptyPasswords no`
- WHEN `SshDisableEmptyPasswords::check(&ctx)` is called
- THEN it returns `Compliant`

### Requirement: SSH Idle Timeout and Grace Controls

The system MUST provide `ssh_client_alive_interval` (CIS 5.2.6) setting `ClientAliveInterval 300` and `ClientAliveCountMax 0`, and `ssh_login_grace_time` (CIS 5.2.7) setting `LoginGraceTime 60`.

#### Scenario: ClientAliveInterval is missing — non-compliant

- GIVEN `sshd_config` has no `ClientAliveInterval` directive
- WHEN `SshClientAliveInterval::check(&ctx)` is called
- THEN it returns `NonCompliant`

#### Scenario: LoginGraceTime applies with backup

- GIVEN `sshd_config` has `LoginGraceTime 120`
- WHEN `apply(&ctx)` is called
- THEN `LoginGraceTime 60` is set and the original config is backed up

### Requirement: SSH Access Restriction Controls

The system MUST provide `ssh_limit_access` (CIS 5.2.8) verifying `AllowUsers` or `AllowGroups` is configured, and `ssh_set_banner` (CIS 5.2.9) setting a legal banner.

#### Scenario: AllowUsers with specific users — compliant

- GIVEN `sshd_config` contains `AllowUsers admin deploy`
- WHEN `SshLimitAccess::check(&ctx)` is called
- THEN it returns `Compliant`

#### Scenario: No AllowUsers or AllowGroups — non-compliant

- GIVEN `sshd_config` has neither `AllowUsers` nor `AllowGroups`
- WHEN `SshLimitAccess::check(&ctx)` is called
- THEN it returns `NonCompliant`

### Requirement: SSH Security Controls

The system MUST provide `ssh_disable_x11_forwarding` (CIS 5.2.10) setting `X11Forwarding no` and `ssh_set_crypto_policy` (CIS 5.2.11) verifying cryptographic policies. `severity()` MUST return `Severity::Medium` for X11, `Severity::High` for crypto.

#### Scenario: X11Forwarding yes — non-compliant

- GIVEN `sshd_config` contains `X11Forwarding yes`
- WHEN `SshDisableX11Forwarding::check(&ctx)` is called
- THEN it returns `NonCompliant`

### Requirement: SSH Config Drop-in Handling

SSH controls MUST handle drop-in directories (`/etc/ssh/sshd_config.d/`) and MUST NOT treat commented-out directives as compliant. A commented `#PermitRootLogin yes` is `NonCompliant`.

#### Scenario: Drop-in file overrides main config

- GIVEN `sshd_config` has `PermitRootLogin yes` but `sshd_config.d/50-vallumix.conf` has `PermitRootLogin no`
- WHEN `check(&ctx)` is called
- THEN it returns `Compliant` — drop-in takes precedence

### Requirement: Pilot Root Login Control Category

The existing `disable_root_login` pilot control MUST gain `category()` returning `Category::Ssh`.

#### Scenario: disable_root_login returns Ssh category

- GIVEN `SshDisableRootLogin` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Ssh`

## Acceptance Criteria

- [ ] 6-8 SSH controls implemented with CIS 5.2.x IDs
- [ ] All controls handle drop-in directories and commented directives
- [ ] `with_paths()` pattern for every SSH control (mock sshd_config paths)
- [ ] Idempotent apply — `AlreadyCompliant` on re-run
- [ ] Rollback restores original sshd_config from backup