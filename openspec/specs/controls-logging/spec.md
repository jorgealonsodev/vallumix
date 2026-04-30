# controls-logging Specification

## Purpose

Implement CIS 4.x logging and audit controls covering rsyslog, journald, and auditd configuration, each implementing the `Control` trait with `category()` returning `Category::Logging`.

## Requirements

### Requirement: Rsyslog Controls

The system MUST provide `ensure_rsyslog_installed` (CIS 4.1.1.1), `ensure_rsyslog_configured` (CIS 4.1.1.2), and `ensure_rsyslog_perms` (CIS 4.1.1.3). `check()` MUST verify installation, configuration, and file permissions respectively.

#### Scenario: rsyslog_installed checks package presence

- GIVEN `rsyslog` package is installed and service is active
- WHEN `EnsureRsyslogInstalled::check(&ctx)` is called
- THEN it returns `Compliant` with evidence "rsyslog installed and active"

#### Scenario: rsyslog_configured verifies logging directives

- GIVEN `/etc/rsyslog.conf` is missing `auth,authpriv.*` directive
- WHEN `EnsureRsyslogConfigured::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence about missing directives

#### Scenario: rsyslog_perms checks file ownership

- GIVEN `/etc/rsylog.conf` has mode `0640` and owner `root`
- WHEN `EnsureRsyslogPerms::check(&ctx)` is called
- THEN it returns `Compliant`

### Requirement: Journald Controls

The system MUST provide `ensure_journald_configured` (CIS 4.1.2.1) and `ensure_journald_override` (CIS 4.1.2.2) that verify and configure `/etc/systemd/journald.conf` and drop-in overrides. `severity()` MUST return `Severity::Medium`.

#### Scenario: journald_configured checks storage setting

- GIVEN `/etc/systemd/journald.conf` has `Storage=persistent`
- WHEN `check(&ctx)` is called
- THEN it returns `Compliant`

#### Scenario: journald_override creates drop-in

- GIVEN journald override file does not exist
- WHEN `apply(&ctx)` is called
- THEN a drop-in file is created under `/etc/systemd/journald.conf.d/` and backup is recorded

### Requirement: Auditd Installation and Configuration

The system MUST provide `ensure_auditd_installed` (CIS 4.1.3.1) and `ensure_auditd_configured` (CIS 4.1.3.2). `check()` MUST verify package installation and daemon configuration. `severity()` MUST return `Severity::High`.

#### Scenario: auditd_installed detects missing package

- GIVEN `auditd` package is not installed
- WHEN `EnsureAuditdInstalled::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence "auditd not installed"

#### Scenario: auditd_configured verifies max_log_file_action

- GIVEN `auditd.conf` has `max_log_file_action = keep_logs`
- WHEN `check(&ctx)` is called
- THEN it returns `Compliant` with evidence about the configuration value

### Requirement: Audit Rule Controls

The system MUST provide controls for audit rules covering identity (CIS 4.1.4.x), login events (CIS 4.1.5.x), and session changes (CIS 4.1.6.x). Each MUST verify that appropriate auditd rules exist in `/etc/audit/rules.d/`.

#### Scenario: audit_identity_rules checks for loginuid rule

- GIVEN `/etc/audit/rules.d/vallumix.rules` contains `-a exit,always -F arch=b64 -S setuid -F auid>=1000 -k identity`
- WHEN `EnsureAuditIdentityRules::check(&ctx)` is called
- THEN it returns `Compliant`

#### Scenario: audit_login_events applies rules with backup

- GIVEN no audit rules for login events exist
- WHEN `apply(&ctx)` is called
- THEN rules are appended to `/etc/audit/rules.d/vallumix.rules` and a backup is created

### Requirement: Logrotate Control

The system SHOULD provide `ensure_logrotate` (CIS 4.1.7) that verifies logrotate is configured for all active log files. `severity()` MUST return `Severity::Low`.

#### Scenario: logrotate checks rotation config exists

- GIVEN `/etc/logrotate.d/rsyslog` exists with weekly rotation
- WHEN `EnsureLogrotate::check(&ctx)` is called
- THEN it returns `Compliant`

### Requirement: Logging Controls with_paths Pattern

Every logging control MUST implement `with_paths()` for testability, allowing override of `/etc/rsyslog.conf`, `/etc/systemd/journald.conf`, `/etc/audit/`, and `/etc/logrotate.d/` paths.

#### Scenario: with_paths overrides audit rules directory

- GIVEN `EnsureAuditIdentityRules::with_paths(MockPaths { rules_dir, auditd_conf })`
- WHEN `check(&ctx)` is called
- THEN it reads from the mock rules directory

## Acceptance Criteria

- [ ] 8-10 logging controls implemented with CIS IDs
- [ ] All controls implement `Control` trait including `category()`
- [ ] `with_paths()` test pattern for every logging control
- [ ] Audit rules use drop-in file `/etc/audit/rules.d/vallumix.rules`
- [ ] Rollback restores original config files from backup