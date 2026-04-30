# controls-auth Specification

## Purpose

Implement CIS 5.1/5.3-5.5 authentication, PAM, and password policy controls, each implementing the `Control` trait with `category()` returning `Category::Auth`.

## Requirements

### Requirement: Cron Daemon Control

The system MUST provide `ensure_cron_daemon` (CIS 5.1.1) that verifies `cron` or `crond` service is enabled and active. `severity()` MUST return `Severity::Medium`.

#### Scenario: cron daemon is running — compliant

- GIVEN `cron` service is active and enabled
- WHEN `EnsureCronDaemon::check(&ctx)` is called
- THEN it returns `Compliant` with evidence "cron active and enabled"

#### Scenario: cron daemon not running — non-compliant

- GIVEN `cron` service is inactive
- WHEN `EnsureCronDaemon::check(&ctx)` is called
- THEN it returns `NonCompliant`

### Requirement: PAM Password Quality Controls

The system MUST provide `ensure_pam_password_quality` (CIS 5.3.1) verifying PAM `pam_pwquality` or `pam_cracklib` module is configured, `ensure_pam_minlen` (CIS 5.3.2) setting minimum password length ≥ 14, and `ensure_pam_credit` (CIS 5.3.x) enforcing complexity credit requirements (`dcredit`, `ucredit`, `ocredit`, `lcredit` all set to -1).

#### Scenario: pam_minlen below threshold

- GIVEN `/etc/security/pwquality.conf` has `minlen = 8`
- WHEN `EnsurePamMinlen::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence `"minlen=8, required=14"`

#### Scenario: pam_credit applies complexity settings

- GIVEN `pwquality.conf` has insufficient credit requirements
- WHEN `apply(&ctx)` is called
- THEN `dcredit=-1 ucredit=-1 ocredit=-1 lcredit=-1` is set with backup

### Requirement: PAM Lockout and History Controls

The system MUST provide `ensure_pam_faillock` (CIS 5.3.3) configuring account lockout after 5 failed attempts for 900 seconds, and `ensure_pam_remember` (CIS 5.3.4) enforcing password history of 5 or more.

#### Scenario: faillock not configured

- GIVEN `/etc/pam.d/password-auth` does not contain `pam_faillock`
- WHEN `EnsurePamFaillock::check(&ctx)` is called
- THEN it returns `NonCompliant`

#### Scenario: remember history below threshold

- GIVEN PAM `pam_pwhistory` has `remember=3`
- WHEN `EnsurePamRemember::check(&ctx)` is called
- THEN it returns `NonCompliant` — `remember` MUST be ≥ 5

### Requirement: Password Hashing Control

The system MUST provide `ensure_password_hashing` (CIS 5.4.x) that verifies `/etc/login.defs` has `ENCRYPT_METHOD SHA512` or `yescrypt`. `severity()` MUST return `Severity::High`.

#### Scenario: SHA512 hashing configured — compliant

- GIVEN `/etc/login.defs` contains `ENCRYPT_METHOD SHA512`
- WHEN `EnsurePasswordHashing::check(&ctx)` is called
- THEN it returns `Compliant`

### Requirement: Umask and Shell Timeout Controls

The system MUST provide `ensure_umask` (CIS 5.5.x) verifying `/etc/profile` and `/etc/bashrc` set umask to `0077` or more restrictive, and `ensure_shell_timeout` (CIS 5.5.x) verifying `TMOUT` is set to 300 seconds or less.

#### Scenario: umask is less restrictive than 0027

- GIVEN `/etc/profile` sets `umask 0022`
- WHEN `EnsureUmask::check(&ctx)` is called
- THEN it returns `NonCompliant` — umask `0022` is less restrictive than `0027`

#### Scenario: shell timeout not set

- GIVEN `/etc/profile` has no `TMOUT` variable
- WHEN `EnsureShellTimeout::check(&ctx)` is called
- THEN it returns `NonCompliant`

### Requirement: Auth Controls with_paths Pattern

Every auth/PAM control MUST implement `with_paths()` for testability, allowing override of `/etc/pam.d/`, `/etc/security/`, `/etc/login.defs`, `/etc/profile`, and `/etc/bashrc` paths.

#### Scenario: with_paths overrides PAM config directory

- GIVEN `EnsurePamFaillock::with_paths(MockPaths { pam_dir, pwquality_conf })`
- WHEN `check(&ctx)` is called
- THEN it reads from mock paths instead of `/etc/pam.d/`

## Acceptance Criteria

- [ ] 8-10 auth/PAM controls implemented with CIS IDs
- [ ] All controls implement `Control` trait including `category()`
- [ ] `with_paths()` pattern for every auth control
- [ ] PAM controls handle both `pam_pwquality` (Debian) and `pam_cracklib` (RHEL) paths
- [ ] Idempotent apply and rollback from backup