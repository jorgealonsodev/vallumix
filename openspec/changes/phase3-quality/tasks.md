# Tasks: phase3-quality — Test Coverage, Insta Snapshots, CI Expansion, Vagrant Fixtures

**Change**: phase3-quality
**Total Tasks**: 33
**Mode**: hybrid (Engram + file)
**Skill Resolution**: none

## Phase 1: Test Infrastructure — Shared Helpers

### T-001 — test-coverage
**Capability**: test-coverage
**Description**: Create `services/common.rs` unit tests — test `ServiceDisable::new()`, `check()` compliant/non-compliant, `apply()` writes disable drop-in, `rollback()` restores
**TDD Expectations**:
- RED: Test `ServiceDisable` check when systemctl returns inactive → Compliant
- RED: Test `apply()` creates `/etc/systemd/system/<svc>.service.d/99-vallumix.conf`
- GREEN: Implement `with_paths()` override for testable paths
**Dependencies**: None
**Files affected**: `crates/vallumix-controls/src/services/common.rs`

### T-002 — test-coverage
**Capability**: test-coverage
**Description**: Create `network/common.rs` unit tests — test `SysctlControl::new()`, `check()` reads /proc/sys correctly, `apply()` writes sysctl drop-in, idempotent apply
**TDD Expectations**:
- RED: Test `SysctlControl` with `with_paths()` overriding /proc/sys
- RED: Test check returns Compliant when value matches expected
- GREEN: Implement test helpers for sysctl path mocking
**Dependencies**: T-001
**Files affected**: `crates/vallumix-controls/src/network/common.rs`

### T-003 — test-coverage
**Capability**: test-coverage
**Description**: Create `auth/common.rs` shared test helpers — `AuthContext` struct for PAM config testing, helper to create temp pwquality.conf
**TDD Expectations**:
- RED: Test auth helper writes PAM config with minlen=14
- GREEN: Implement `with_pam_config()` fixture helper
**Dependencies**: T-001
**Files affected**: `crates/vallumix-controls/src/auth/common.rs`

### T-004 — test-coverage
**Capability**: test-coverage
**Description**: Create `logging/common.rs` shared test helpers — `LoggingContext` struct for rsyslog/journald/auditd config testing
**TDD Expectations**:
- RED: Test logging helper creates temp /etc/rsyslog.conf
- GREEN: Implement `with_rsyslog_config()`, `with_journald_config()` helpers
**Dependencies**: T-001
**Files affected**: `crates/vallumix-controls/src/logging/common.rs`

### T-005 — test-coverage
**Capability**: test-coverage
**Description**: Create `maintenance/common.rs` shared test helpers — `PermsContext` struct, `with_passwd_shadow()` fixture helper, file permission test utilities
**TDD Expectations**:
- RED: Test perms helper creates temp /etc/passwd with mode 0644
- GREEN: Implement `with_file_perms()` helper
**Dependencies**: T-001
**Files affected**: `crates/vallumix-controls/src/maintenance/common.rs`

### T-006 — test-coverage
**Capability**: test-coverage
**Description**: Create `ssh/common.rs` unit tests — test `SshConfig` parse helper, `check_sshd_config()` function with fixture sshd_config content
**TDD Expectations**:
- RED: Test parse of sshd_config with `Port 22`, `Protocol 2`
- GREEN: Implement `parse_sshd_value()` helper
**Dependencies**: T-001
**Files affected**: `crates/vallumix-controls/src/ssh/common.rs`

---

## Phase 2: Controls Tests — Auth Module (CIS 5.1, 5.3–5.5)

### T-007 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsureCronDaemon` (5.1.1) — check when systemd unit exists/inactive, apply creates cron daemon enable
**TDD Expectations**:
- RED: `check_compliant_when_unit_absent()` → Compliant
- RED: `check_non_compliant_when_unit_active()` → NonCompliant
- RED: `apply_writes_systemd_drop_in()` → verify enable content
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_cron_daemon.rs`

### T-008 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsurePamPasswordQuality` (5.3.1) — check when pam_pwquality.so present/absent, apply adds module to system-auth
**TDD Expectations**:
- RED: `check_compliant_when_module_present()` → Compliant
- RED: `apply_adds_pam_pwquality()` → verify content
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_pam_password_quality.rs`

### T-009 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsurePamMinlen` (5.3.2) — check pwquality.conf minlen ≥ 14, apply updates /etc/security/pwquality.conf
**TDD Expectations**:
- RED: `check_compliant_when_minlen_14()` → Compliant
- RED: `check_non_compliant_when_minlen_8()` → NonCompliant
- RED: `apply_writes_minlen()` → verify file content
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_pam_minlen.rs`

### T-010 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsurePamCredit` (5.3.x) — check dcredit/ucredit/ocredit/lcredit all = -1 in pwquality.conf
**TDD Expectations**:
- RED: `check_compliant_when_credits_negative_one()` → Compliant
- RED: `check_non_compliant_when_credits_zero()` → NonCompliant
- RED: `apply_writes_credit_settings()` → verify all 4 params
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_pam_credit.rs`

### T-011 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsurePamFaillock` (5.3.4) — check pam_faillock.so entries with deny=5 unlock_time=900
**TDD Expectations**:
- RED: `check_compliant_when_faillock_configured()` → Compliant
- RED: `apply_adds_faillock_entries()` → verify content
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_pam_faillock.rs`

### T-012 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsurePamRemember` (5.3.5) — check remember≥5 in pam_unix.so, apply updates
**TDD Expectations**:
- RED: `check_compliant_when_remember_5()` → Compliant
- RED: `check_non_compliant_when_no_remember()` → NonCompliant
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_pam_remember.rs`

### T-013 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsurePasswordHashing` (5.4.1) — check ENCRYPT_METHOD SHA512/yescrypt in /etc/login.defs
**TDD Expectations**:
- RED: `check_compliant_when_sha512()` → Compliant
- RED: `check_non_compliant_when_md5()` → NonCompliant
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_password_hashing.rs`

### T-014 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsureUmask` (5.5.1) — check umask 0077 in /etc/profile and /etc/bashrc
**TDD Expectations**:
- RED: `check_compliant_when_umask_077()` → Compliant
- RED: `check_non_compliant_when_umask_022()` → NonCompliant
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_umask.rs`

### T-015 — controls-auth
**Capability**: test-coverage
**Description**: Add tests for `EnsureShellTimeout` (5.5.2) — check TMOUT=300 in /etc/profile, apply sets TMOUT
**TDD Expectations**:
- RED: `check_compliant_when_tmout_300()` → Compliant
- RED: `check_non_compliant_when_tmout_600()` → NonCompliant
- RED: `apply_writes_tmout()` → verify content
**Dependencies**: T-003
**Files affected**: `crates/vallumix-controls/src/auth/ensure_shell_timeout.rs`

---

## Phase 3: Controls Tests — Logging Module (CIS 4.1)

### T-016 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureRsyslogInstalled` (4.1.1.1) — check binary /usr/sbin/rsyslogd exists
**TDD Expectations**:
- RED: `check_compliant_when_binary_exists()` → Compliant
- RED: `check_non_compliant_when_binary_absent()` → NonCompliant
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_rsyslog_installed.rs`

### T-017 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureRsyslogConfigured` (4.1.1.2) — check auth,authpriv.* directive in rsyslog.conf
**TDD Expectations**:
- RED: `check_compliant_when_directive_present()` → Compliant
- RED: `check_non_compliant_when_directive_absent()` → NonCompliant
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_rsyslog_configured.rs`

### T-018 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureRsyslogPerms` (4.1.1.3) — check /var/log permissions mode 0640, owner root
**TDD Expectations**:
- RED: `check_compliant_when_perms_0640()` → Compliant
- RED: `apply_corrects_permissions()` → verify mode
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_rsyslog_perms.rs`

### T-019 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureJournaldConfigured` (4.1.2.1) — check Storage=persistent in journald.conf
**TDD Expectations**:
- RED: `check_compliant_when_storage_persistent()` → Compliant
- RED: `apply_creates_drop_in()` → verify content
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_journald_configured.rs`

### T-020 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureJournaldOverride` (4.1.2.2) — check drop-in /etc/systemd/journald.conf.d/ exists with override
**TDD Expectations**:
- RED: `check_compliant_when_dropin_exists()` → Compliant
- RED: `apply_creates_dropin_with_compress()` → verify
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_journald_override.rs`

### T-021 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureAuditdInstalled` (4.1.3.1) — check auditd binary /sbin/auditd exists
**TDD Expectations**:
- RED: `check_compliant_when_binary_exists()` → Compliant
- RED: `check_non_compliant_when_absent()` → NonCompliant
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_auditd_installed.rs`

### T-022 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureAuditdConfigured` (4.1.3.2) — check max_log_file action = keep_logs in auditd.conf
**TDD Expectations**:
- RED: `check_compliant_when_keep_logs()` → Compliant
- RED: `apply_updates_auditd_conf()` → verify
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_auditd_configured.rs`

### T-023 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureAuditIdentityRules` (4.1.4.1) — check -w /etc/passwd -p wa exists in audit rules
**TDD Expectations**:
- RED: `check_compliant_when_rule_exists()` → Compliant
- RED: `apply_appends_rules_with_backup()` → verify
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_audit_identity_rules.rs`

### T-024 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureAuditLoginEvents` (4.1.4.2) — check session rules in audit rules
**TDD Expectations**:
- RED: `check_compliant_when_login_rules_present()` → Compliant
- RED: `apply_appends_session_rules()` → verify
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_audit_login_events.rs`

### T-025 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureAuditSessionEvents` (4.1.4.3) — check user session events rules
**TDD Expectations**:
- RED: `check_compliant_when_session_rules_present()` → Compliant
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_audit_session_events.rs`

### T-026 — controls-logging
**Capability**: test-coverage
**Description**: Add tests for `EnsureLogrotate` (4.1.7) — check /etc/logrotate.d/rsyslog with weekly rotation
**TDD Expectations**:
- RED: `check_compliant_when_config_exists()` → Compliant
- RED: `apply_creates_logrotate_config()` → verify
**Dependencies**: T-004
**Files affected**: `crates/vallumix-controls/src/logging/ensure_logrotate.rs`

---

## Phase 4: Controls Tests — Network & Maintenance Modules

### T-027 — controls-network
**Capability**: test-coverage
**Description**: Add tests for `SysctlDisableSendRedirects` (3.1.2) using SysctlControl common — check all+default send_redirects==0
**TDD Expectations**:
- RED: `check_compliant_when_both_zero()` → Compliant
- RED: `check_non_compliant_when_one_is_one()` → NonCompliant
- RED: `apply_writes_sysctl_drop_in()` → verify content
**Dependencies**: T-002
**Files affected**: `crates/vallumix-controls/src/network/sysctl_disable_send_redirects.rs`

### T-028 — controls-network
**Capability**: test-coverage
**Description**: Add tests for `SysctlDisableSourceRoute` (3.2.1) and `SysctlDisableAcceptRedirects` (3.2.2)
**TDD Expectations**:
- RED: `check_compliant_when_all_zero()` → Compliant
- RED: `apply_writes_correct_values()` → verify
**Dependencies**: T-002
**Files affected**: `crates/vallumix-controls/src/network/sysctl_disable_source_route.rs`, `crates/vallumix-controls/src/network/sysctl_disable_accept_redirects.rs`

### T-029 — controls-network
**Capability**: test-coverage
**Description**: Add tests for `SysctlEnableRpFilter` (3.2.6) and `SysctlEnableSyncookies` (3.2.7)
**TDD Expectations**:
- RED: `check_compliant_when_rp_filter_one()` → Compliant
- RED: `check_compliant_when_syncookies_one()` → Compliant
**Dependencies**: T-002
**Files affected**: `crates/vallumix-controls/src/network/sysctl_enable_rp_filter.rs`, `crates/vallumix-controls/src/network/sysctl_enable_syncookies.rs`

### T-030 — controls-maintenance
**Capability**: test-coverage
**Description**: Add tests for `EnsurePermsShadow` (6.1.2) and `EnsurePermsGroup` (6.1.3) — mode 0600 and 0644
**TDD Expectations**:
- RED: `check_compliant_when_correct_mode()` → Compliant
- RED: `apply_corrects_mode()` → verify
**Dependencies**: T-005
**Files affected**: `crates/vallumix-controls/src/maintenance/ensure_perms_shadow.rs`, `crates/vallumix-controls/src/maintenance/ensure_perms_group.rs`

### T-031 — controls-maintenance
**Capability**: test-coverage
**Description**: Add tests for `EnsurePermsGshadow` (6.1.4) — mode 0600, owner root
**TDD Expectations**:
- RED: `check_compliant_when_perms_0600()` → Compliant
- RED: `apply_corrects_gshadow_perms()` → verify
**Dependencies**: T-005
**Files affected**: `crates/vallumix-controls/src/maintenance/ensure_perms_gshadow.rs`

### T-032 — controls-maintenance
**Capability**: test-coverage
**Description**: Add tests for `AuditWorldWritable` (6.1.5) and `AuditSuidSgid` (6.1.6) — audit-only controls returning Skipped
**TDD Expectations**:
- RED: `check_finds_world_writable_files()` → NonCompliant with evidence
- RED: `apply_returns_skipped_with_warning()` → Skipped
**Dependencies**: T-005
**Files affected**: `crates/vallumix-controls/src/maintenance/audit_world_writable.rs`, `crates/vallumix-controls/src/maintenance/audit_suid_sgid.rs`

### T-033 — controls-maintenance
**Capability**: test-coverage
**Description**: Add tests for `AuditUnownedFiles` (6.1.7), `AuditDuplicateIds` (6.1.8), `EnsureCronPerms` (6.1.9)
**TDD Expectations**:
- RED: `check_finds_unowned()` → NonCompliant
- RED: `check_finds_duplicate_ids()` → NonCompliant
- RED: `check_cron_dirs_correct_perms()` → Compliant/NonCompliant
**Dependencies**: T-005
**Files affected**: `crates/vallumix-controls/src/maintenance/audit_unowned_files.rs`, `crates/vallumix-controls/src/maintenance/audit_duplicate_ids.rs`, `crates/vallumix-controls/src/maintenance/ensure_cron_perms.rs`

---

## Phase 5: Insta Snapshots — Reporter Output Verification

### T-034 — insta-snapshots
**Capability**: insta-snapshots
**Description**: Generate TextReporter insta snapshot — run `cargo insta test --accept` for text_reporter_snapshot test
**TDD Expectations**:
- RED: `cargo insta test` → snapshot not found, new snapshot needed
- GREEN: Run `cargo insta accept` → snapshots/text_reporter_snapshot.snap created
**Dependencies**: None (reporter already has test)
**Files affected**: `crates/vallumix-reporters/src/text.rs` (snapshots/ dir created)

### T-035 — insta-snapshots
**Capability**: insta-snapshots
**Description**: Generate JsonReporter insta snapshot — run `cargo insta test --accept` for json_reporter_snapshot test
**TDD Expectations**:
- RED: `cargo insta test` → snapshot not found
- GREEN: Run `cargo insta accept` → snapshots/json_reporter_snapshot.snap created
**Dependencies**: None
**Files affected**: `crates/vallumix-reporters/src/json.rs` (snapshots/ dir)

### T-036 — insta-snapshots
**Capability**: insta-snapshots
**Description**: Generate HtmlReporter insta snapshot — run `cargo insta test --accept` for html_reporter_snapshot test
**TDD Expectations**:
- RED: `cargo insta test` → snapshot not found
- GREEN: Run `cargo insta accept` → snapshots/html_reporter_snapshot.snap created
**Dependencies**: None
**Files affected**: `crates/vallumix-reporters/src/html.rs` (snapshots/ dir)

### T-037 — insta-snapshots
**Capability**: insta-snapshots
**Description**: Generate JunitReporter insta snapshot — run `cargo insta test --accept` for junit_reporter_snapshot test
**TDD Expectations**:
- RED: `cargo insta test` → snapshot not found
- GREEN: Run `cargo insta accept` → snapshots/junit_reporter_snapshot.snap created
**Dependencies**: None
**Files affected**: `crates/vallumix-reporters/src/junit.rs` (snapshots/ dir)

---

## Phase 6: CLI Integration Tests — assert_cmd Expansion

### T-038 — cli-integration
**Capability**: test-coverage
**Description**: Add `cli_audit_html_report` test — `audit --report html` produces valid HTML with DOCTYPE and control elements
**TDD Expectations**:
- RED: `assert_cmd` on `vallumix audit --profile web --report html` → stdout contains `<!DOCTYPE html>`
- GREEN: Implement HTML reporter output (or verify existing implementation produces valid HTML)
**Dependencies**: T-036
**Files affected**: `crates/vallumix-cli/tests/cli.rs`

### T-039 — cli-integration
**Capability**: test-coverage
**Description**: Add `cli_audit_junit_report` test — `audit --report junit` produces valid XML with testsuite element
**TDD Expectations**:
- RED: `assert_cmd` on `vallumix audit --profile web --report junit` → stdout contains `<?xml` and `<testsuite`
- GREEN: Verify JunitReporter generates valid XML
**Dependencies**: T-037
**Files affected**: `crates/vallumix-cli/tests/cli.rs`

### T-040 — cli-integration
**Capability**: test-coverage
**Description**: Add `cli_audit_text_report` test — `audit --report text` produces formatted text with ✓/✗ icons
**TDD Expectations**:
- RED: `assert_cmd` on `vallumix audit --profile web --report text` → stdout contains "Pass:" and "Fail:"
- GREEN: Verify TextReporter outputs correctly to stdout
**Dependencies**: T-034
**Files affected**: `crates/vallumix-cli/tests/cli.rs`

### T-041 — cli-integration
**Capability**: test-coverage
**Description**: Add `cli_audit_multi_report` test — `audit --report html,json` produces both files (or output to stdout)
**TDD Expectations**:
- RED: `assert_cmd` on `vallumix audit --profile web --report html,json` → both formats included in output
- GREEN: Implement multi-format output or --output flag to file
**Dependencies**: T-038, T-039
**Files affected**: `crates/vallumix-cli/tests/cli.rs`

### T-042 — cli-integration
**Capability**: test-coverage
**Description**: Add `cli_audit_output_file` test — `audit --output /tmp/vallumix-report.html` writes file to disk
**TDD Expectations**:
- RED: `assert_cmd` on `vallumix audit --profile web --report html --output /tmp/report.html` → file exists after run
- GREEN: Implement --output flag in CLI
**Dependencies**: T-038
**Files affected**: `crates/vallumix-cli/tests/cli.rs`

### T-043 — cli-integration
**Capability**: test-coverage
**Description**: Add `cli_rollback_no_session` test — `rollback --control-id X.XX.X` when no backup exists returns exit code 2
**TDD Expectations**:
- RED: `assert_cmd` on `vallumix rollback --control-id 9.9.9.9` → failure code 2 (no session found)
- GREEN: Implement rollback error handling for missing backup
**Dependencies**: None
**Files affected**: `crates/vallumix-cli/tests/cli.rs`

---

## Phase 7: CI Workflow Expansion

### T-044 — ci-expansion
**Capability**: ci-expansion
**Description**: Add aarch64 cross-build job to CI — install cross, add `aarch64-unknown-linux-gnu` target, build-only job (no test execution on aarch64)
**TDD Expectations**:
- RED: CI runs `cross build --target aarch64-unknown-linux-gnu` → compilation succeeds
- GREEN: Add separate `build-aarch64` job in CI matrix
**Dependencies**: None
**Files affected**: `.github/workflows/ci.yml`

### T-045 — ci-expansion
**Capability**: ci-expansion
**Description**: Add cargo-tarpaulin to CI — install tarpaulin, run `cargo tarpaulin --workspace --out Html --out Lcov`, upload coverage
**TDD Expectations**:
- RED: CI runs tarpaulin and produces HTML/Lcov coverage report
- GREEN: Add tarpaulin step, configure .tarpaulin.toml
**Dependencies**: T-044
**Files affected**: `.github/workflows/ci.yml`, `.tarpaulin.toml`

### T-046 — ci-expansion
**Capability**: ci-expansion
**Description**: Add cargo-deny to CI — run `cargo deny check` in CI, verify deny.toml configuration
**TDD Expectations**:
- RED: CI runs `cargo deny check` → no ban warnings
- GREEN: Add deny step to CI after build
**Dependencies**: None
**Files affected**: `.github/workflows/ci.yml`

### T-047 — ci-expansion
**Capability**: ci-expansion
**Description**: Add cargo-audit to CI — run `cargo audit` after dependency resolution, fail on vulnerabilities
**TDD Expectations**:
- RED: CI runs `cargo audit` → no vulnerabilities found (or warn only)
- GREEN: Add audit step to CI
**Dependencies**: None
**Files affected**: `.github/workflows/ci.yml`

### T-048 — ci-expansion
**Capability**: ci-expansion
**Description**: Add insta snapshot CI check — run `cargo insta test --require-snapshots` in CI to fail on new snapshots
**TDD Expectations**:
- RED: CI runs insta test → all snapshots present, no pending
- GREEN: Add insta test step, configure snapshot location
**Dependencies**: T-034 through T-037
**Files affected**: `.github/workflows/ci.yml`

---

## Phase 8: Vagrant Fixtures

### T-049 — vagrant-fixtures
**Capability**: vagrant-fixtures
**Description**: Create `Vagrantfile` with base config — define 4 VMs (debian12, ubuntu2204, ubuntu2404, rockylinux9), synced folder, memory 2GB
**TDD Expectations**:
- RED: `vagrant up` provisions all 4 VMs without error
- GREEN: Write Vagrantfile with base box definitions and provisioning script paths
**Dependencies**: None
**Files affected**: `Vagrantfile` (project root)

### T-050 — vagrant-fixtures
**Capability**: vagrant-fixtures
**Description**: Create `scripts/provision-debian.sh` — install Rust toolchain, build vallumix, run `vallumix audit --profile web`
**TDD Expectations**:
- RED: VM boots, script installs Rust, builds project, runs audit command successfully
- GREEN: Write provision script with Rust installation and project build
**Dependencies**: T-049
**Files affected**: `scripts/provision-debian.sh`

### T-051 — vagrant-fixtures
**Capability**: vagrant-fixtures
**Description**: Create `scripts/provision-ubuntu.sh` — same as debian but for Ubuntu (use apt-get)
**TDD Expectations**:
- RED: Ubuntu VM provisions correctly with Rust and vallumix
- GREEN: Write provision script for Ubuntu
**Dependencies**: T-049
**Files affected**: `scripts/provision-ubuntu.sh`

### T-052 — vagrant-fixtures
**Capability**: vagrant-fixtures
**Description**: Create `scripts/provision-rocky.sh` — use dnf for Rocky Linux 9, install Rust via rustup
**TDD Expectations**:
- RED: Rocky VM provisions correctly with Rust and vallumix
- GREEN: Write provision script for Rocky
**Dependencies**: T-049
**Files affected**: `scripts/provision-rocky.sh`

---

## Phase 9: Verification

### T-053 — verification
**Capability**: test-coverage
**Description**: Run `cargo test --workspace` — verify all 200+ tests pass including new controls tests
**TDD Expectations**:
- RED: `cargo test --workspace` → all tests pass
- GREEN: Fix any test failures from new test additions
**Dependencies**: T-001 through T-043
**Files affected**: All crates — workspace-wide test run

### T-054 — verification
**Capability**: ci-expansion
**Description**: Verify coverage threshold — ensure tarpaulin shows ≥80% coverage across all crates
**TDD Expectations**:
- RED: `cargo tarpaulin --workspace` → overall coverage ≥ 80%
- GREEN: Identify and address coverage gaps in untested modules
**Dependencies**: T-045, T-053
**Files affected**: `.tarpaulin.toml` (coverage configuration)

---

## Summary

| Phase | Tasks | Focus |
|-------|-------|-------|
| Phase 1 | T-001–T-006 | Shared test helpers: 6 common.rs modules |
| Phase 2 | T-007–T-015 | Auth controls: 9 controls × 3-4 scenarios |
| Phase 3 | T-016–T-026 | Logging controls: 11 controls × 3-4 scenarios |
| Phase 4 | T-027–T-033 | Network + Maintenance: 10 controls |
| Phase 5 | T-034–T-037 | Insta snapshots: 4 reporters |
| Phase 6 | T-038–T-043 | CLI integration: 6 new assert_cmd tests |
| Phase 7 | T-044–T-048 | CI expansion: aarch64, tarpaulin, deny, audit, insta |
| Phase 8 | T-049–T-052 | Vagrant fixtures: Vagrantfile + 3 scripts |
| Phase 9 | T-053–T-054 | Verification: test run + coverage check |
| **Total** | **54** | |

**Implementation Order Rationale**:
1. Shared helpers (T-001–T-006) first — all controls depend on them
2. Controls tests (T-007–T-033) next — auth/logging before network/maintenance (reverse dependency order)
3. Snapshots (T-034–T-037) independent — can parallelize with controls tests
4. CLI integration (T-038–T-043) depends on snapshots and CLI wiring
5. CI (T-044–T-048) independent — uses existing code
6. Vagrant (T-049–T-052) independent setup
7. Verification (T-053–T-054) last — everything must be done

**Files Created/Modified**: ~60 files across 5 crates + CI + Vagrant
**New Dependencies**: insta (dev-deps already present), cross, cargo-tarpaulin, cargo-deny, cargo-audit