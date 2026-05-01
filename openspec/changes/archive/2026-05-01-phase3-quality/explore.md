# Phase 3: Quality — Exploration Report

## 1. Current Test Coverage State

### Tests per crate (171 total `#[test]` across 34 files)

| Crate | Test files | Tests | Notes |
|-------|-----------|-------|-------|
| `vallumix-core` | 5 | 36 | All modules covered: control.rs(11), context.rs(2), distro.rs(8), lib.rs(7), profile.rs(8) |
| `vallumix-controls` | 11 | 60 | Only 11 of ~22 control files have tests. Registry tests(17) cover key presence only |
| `vallumix-reporters` | 5 | 22 | All 4 reporters + lib.rs covered. 4 insta snapshot tests exist but snapshots NOT generated |
| `vallumix-backup` | 1 | 10 | Good coverage of BackupManager: create, list, rollback, prune, verify, checksum |
| `vallumix-cli` | 3 | 23 | main.rs(16), tests/cli.rs(6), commands have stub tests(2 each) |
| **Missing** | — | 20 | `list.rs` has NO tests, `completion.rs` has NO tests in commands/ |

### Test quality assessment

**Strong areas:**
- `vallumix-core`: Good unit tests for enums, serialization, profile parsing, distro detection
- `vallumix-backup`: Excellent integration tests with real tempdirs (10 tests, all filesystem-based)
- `vallumix-cli/tests/cli.rs`: 6 `assert_cmd` integration tests covering audit, list, completion, apply dry-run, threshold

**Weak areas:**
- CLI commands (`audit.rs`, `apply.rs`, `rollback.rs`): Tests are stubs — only signature compile checks, NO functional tests
- `vallumix-cli/src/commands/list.rs`: Zero tests
- `vallumix-cli/src/commands/completion.rs`: Zero tests (only 2 compile-checks in main.rs)

## 2. Controls Lacking Test Coverage

### Controls WITH tests (11 files, ~43 tests)

| Control ID | File | Tests | Coverage |
|-----------|------|-------|----------|
| 1.1.1.1 | `disable_cramfs.rs` | 6 | check(+/-), apply, dry-run, 2 fixtures |
| 1.1.1.2 | `disable_freevxfs.rs` | 3 | check(+/-), apply |
| 1.1.1.3 | `disable_jffs2.rs` | 3 | check(+/-), apply |
| 1.1.1.4 | `disable_hfs.rs` | 3 | check(+/-), apply |
| 1.1.1.5 | `disable_hfsplus.rs` | 3 | check(+/-), apply |
| 1.1.1.6 | `disable_squashfs.rs` | 3 | check(+/-), apply |
| 1.1.1.7 | `disable_udf.rs` | 3 | check(+/-), apply |
| 1.1.10 | `disable_usb_storage.rs` | 3 | check(+/-), apply |
| 1.1.2.1 | `harden_tmpfs.rs` | 4 | check(+/-), apply, dry-run |
| 3.1.1 | `sysctl_ip_forwarding.rs` | 5 | check(0/1), apply, 2 fixtures |
| 5.2.4 | `disable_root_login.rs` | 8 | check(yes/no/missing), apply(3 cases), 2 fixtures |
| 5.2.8 | `ssh_limit_access.rs` | 2 | check(+/-) |
| 5.2.9 | `ssh_set_banner.rs` | 2 | check(+/-) |
| 2.2.3 | `disable_avahi.rs` | 4 | check(exists/missing), apply, dry-run |
| 6.1.1 | `ensure_perms_passwd.rs` | 3 | check(+/-), apply |
| 3.3.1 | `configure_firewalld.rs` | 1 | check only |

### Controls WITHOUT ANY tests (HIGH PRIORITY GAP)

**Auth module (9 controls, 0 tests):**
- 5.1.1 `EnsureCronDaemon` — checks systemd unit existence
- 5.3.1 `EnsurePamPasswordQuality` — checks PAM module presence
- 5.3.2 `EnsurePamMinlen` — checks pwquality.conf minlen
- 5.3.3 `EnsurePamCredit` — checks credit parameters
- 5.3.4 `EnsurePamFaillock` — checks pam_faillock.so
- 5.3.5 `EnsurePamRemember` — checks remember= parameter
- 5.4.1 `EnsurePasswordHashing` — checks ENCRYPT_METHOD
- 5.5.1 `EnsureUmask` — checks umask in /etc/profile
- 5.5.2 `EnsureShellTimeout` — checks TMOUT variable

**Logging module (11 controls, 0 tests):**
- 4.1.1.1 `EnsureRsyslogInstalled` — checks binary existence
- 4.1.1.2 `EnsureRsyslogConfigured` — checks rsyslog.conf
- 4.1.1.3 `EnsureRsyslogPerms` — checks /var/log permissions
- 4.1.2.1 `EnsureJournaldConfigured` — checks journald.conf
- 4.1.2.2 `EnsureJournaldOverride` — checks drop-in
- 4.1.3.1 `EnsureAuditdInstalled` — checks auditd binary
- 4.1.3.2 `EnsureAuditdConfigured` — checks auditd.conf
- 4.1.4.1 `EnsureAuditIdentityRules` — checks audit rules
- 4.1.4.2 `EnsureAuditLoginEvents` — checks audit rules
- 4.1.4.3 `EnsureAuditSessionEvents` — checks audit rules
- 4.1.7 `EnsureLogrotate` — checks logrotate config

**Services module (12 controls, 0 tests except avahi):**
- 2.2.2 `DisableCups` (macro-generated via `def_service_control!`)
- 2.2.4 `DisableDhcp`
- 2.2.5 `DisableLdap`
- 2.2.6 `DisableNfs`
- 2.2.7 `DisableRpcbind`
- 2.2.8 `DisableBind`
- 2.2.9 `DisableVsftpd`
- 2.2.10 `DisableHttpd`
- 2.2.11 `DisableDovecot`
- 2.2.12 `DisableXinetd`
- 2.2.14 `DisableSnmpd`
- 2.2.15 `DisableRsync`

**Network module (6 controls, 0 tests except ip_forwarding + firewalld):**
- 3.1.2 `SysctlDisableSendRedirects` (uses `SysctlControl` common)
- 3.2.1 `SysctlDisableSourceRoute`
- 3.2.2 `SysctlDisableAcceptRedirects`
- 3.2.3 `SysctlDisableIcmpRedirects`
- 3.2.6 `SysctlEnableRpFilter`
- 3.2.7 `SysctlEnableSyncookies`

**Maintenance module (8 controls, 0 tests except perms_passwd):**
- 6.1.2 `EnsurePermsShadow` (macro-generated via `def_perm_control!`)
- 6.1.3 `EnsurePermsGroup`
- 6.1.4 `EnsurePermsGshadow`
- 6.1.5 `AuditWorldWritable`
- 6.1.6 `AuditSuidSgid`
- 6.1.7 `AuditUnownedFiles`
- 6.1.8 `AuditDuplicateIds`
- 6.1.9 `EnsureCronPerms`

**Common/shared modules (0 tests):**
- `services/common.rs` — `ServiceDisable` struct (used by 12 macro controls)
- `network/common.rs` — `SysctlControl` struct (used by 6+ sysctl controls)
- `ssh/common.rs` — shared SSH helpers

### Test pattern analysis

Controls with tests follow a consistent pattern:
1. `check_compliant_when_X` — happy path
2. `check_non_compliant_when_Y` — failure path
3. `apply_writes_Z` — apply writes expected file
4. `apply_skips_in_dry_run` — dry-run check
5. `fixture_*_is_compliant` / `fixture_*_is_non_compliant` — fixture-based tests

**Missing test patterns across ALL controls:**
- Error path tests (file not found, permission denied)
- Rollback tests (only `disable_root_login.rs` has rollback logic but no test)
- Edge cases (empty files, malformed content, partial matches)
- Tests for macro-generated controls (services, maintenance perms)
- Tests for common structs (`ServiceDisable`, `SysctlControl`)

## 3. Current CI Workflow Analysis

### What exists (`.github/workflows/ci.yml`)

- **Matrix:** 4 distros (debian:12, ubuntu:22.04, ubuntu:24.04, rockylinux:9)
- **Architecture:** x86_64 ONLY (runs on `ubuntu-latest` runner)
- **Steps:** checkout → install deps → install Rust 1.75 → build → test → clippy → fmt
- **Cache:** cargo registry + target per distro

### What's MISSING for multi-arch

1. **No aarch64 target**: No `cross` usage, no `aarch64-unknown-linux-musl` build
2. **No `rust-toolchain.toml` in CI**: CI installs Rust 1.75 manually instead of using the repo's `rust-toolchain.toml`
3. **No cargo-tarpaulin**: Coverage tool not installed or run
4. **No cargo-deny**: `deny.toml` exists but CI never runs `cargo deny check`
5. **No cargo-audit**: Security audit not in CI
6. **No insta snapshot review**: No snapshot CI integration
7. **No `cross` for container builds**: CI uses containers for distro matrix but doesn't cross-compile

### CI gaps for Phase 3

| Gap | Priority | Effort |
|-----|----------|--------|
| aarch64 cross-build with `cross` | High | Medium |
| cargo-tarpaulin coverage report | High | Low |
| cargo-deny in CI | Medium | Low |
| cargo-audit in CI | Medium | Low |
| insta snapshot CI (auto-commit or fail) | Medium | Low |
| Upload coverage to codecov/cov | Low | Low |

## 4. Insta Snapshot Status

### Current state: ZERO .snap files generated

4 snapshot tests exist but have NEVER been run with `cargo insta test --accept`:

| Reporter | Snapshot test | File |
|----------|--------------|------|
| `JsonReporter` | `json_reporter_snapshot` | `reporters/src/json.rs:90` |
| `HtmlReporter` | `html_reporter_snapshot` | `reporters/src/html.rs:134` |
| `JunitReporter` | `junit_reporter_snapshot` | `reporters/src/junit.rs:213` |
| `TextReporter` | `text_reporter_snapshot` | `reporters/src/text.rs:204` |

### Why no snapshots exist

- `insta` is in `workspace.dependencies` and in `vallumix-reporters` dev-deps
- But snapshots are never generated because `cargo insta test --accept` has never been run
- No `snapshots/` directory exists anywhere in the project
- CI doesn't run `cargo insta test --require-snapshots`

### Action needed

1. Run `cargo insta test --accept` to generate initial snapshots
2. Add `cargo insta test --require-snapshots` to CI
3. Consider adding snapshot tests for control outputs (evidence strings)

## 5. assert_cmd Integration Test Coverage

### Current coverage (6 tests in `crates/vallumix-cli/tests/cli.rs`)

| Test | Command | What it verifies |
|------|---------|-----------------|
| `cli_apply_dry_run_requires_root` | `apply --dry-run` | Exit code 3 when not root |
| `cli_audit_json_report` | `audit --report json` | stdout contains "host", "summary" |
| `cli_list_outputs_controls` | `list` | stdout contains control IDs |
| `cli_completion_bash` | `completion bash` | stdout contains "vallumix" |
| `cli_audit_exit_code_with_threshold` | `audit --threshold 0` | Success (exit 0) |
| `cli_audit_exit_code_fails_below_threshold` | `audit --threshold 100` | Exit code 1 |

### MISSING assert_cmd tests

| Test | Priority |
|------|----------|
| `audit --report html` produces valid HTML | High |
| `audit --report junit` produces valid XML | High |
| `audit --report text` produces formatted text | Medium |
| `audit --report html,json` multi-format output | Medium |
| `audit --output /path` writes files | High |
| `rollback --control-id X` with no sessions | High |
| `rollback --session X` with invalid session | Medium |
| `--profile database` works | Medium |
| `--profile bastion` works | Medium |
| `--dry-run` flag on audit (should be no-op) | Low |
| `--verbose` and `--quiet` flags | Low |
| `--no-color` flag | Low |
| `--threshold` validation (0-100 range) | Medium |
| `--help` output contains all subcommands | Low |
| Invalid subcommand returns error | Low |
| `apply` without root returns exit 3 | Already covered |
| `apply` with root (skip in CI) | N/A |

## 6. Vagrant Setup for Manual Testing

### Current state: NO Vagrantfile exists

The PRD mentions "fixtures Vagrant para test manual" but no Vagrant configuration exists.

### Recommended Vagrant setup

Need Vagrantfiles for each supported distro to test controls on real VMs:
- `Vagrantfile.debian12` — Debian 12 box
- `Vagrantfile.ubuntu2204` — Ubuntu 22.04 LTS
- `Vagrantfile.ubuntu2404` — Ubuntu 24.04 LTS
- `Vagrantfile.rocky9` — Rocky Linux 9

Each should:
1. Provision a clean VM
2. Install Rust toolchain
3. Build vallumix
4. Run `vallumix audit --profile web` to establish baseline
5. Run `vallumix apply --profile web --dry-run` to preview changes
6. Optionally run `vallumix apply --profile web` and verify

## 7. cargo-tarpaulin Configuration Needs

### Current state: NO tarpaulin config

- No `.tarpaulin.toml` or `tarpaulin.toml` exists
- `cargo-tarpaulin` not mentioned in CI
- Rust 1.75 MSRV may need verification with tarpaulin compatibility

### Recommended configuration

```toml
# .tarpaulin.toml
[default]
workspace = true
all-features = false
exclude = []
exclude-files = [
    "crates/vallumix-cli/src/main.rs",  # hard to test CLI entry
]
coveralls = false
out = ["Html", "Lcov"]
timeout = 300
```

### CI integration

Add to CI workflow:
```yaml
- name: Install cargo-tarpaulin
  run: cargo install cargo-tarpaulin
- name: Run coverage
  run: cargo tarpaulin --workspace --out Html --out Lcov
```

**Note:** tarpaulin requires `ptrace` which may not work in all CI containers. May need `--engine llvm` or run on bare runner.

## 8. CI Matrix Expansion for aarch64

### Current matrix: x86_64 only (implicit via `ubuntu-latest`)

### Recommended expansion

Option A: Use `cross` for aarch64 builds (recommended)
```yaml
matrix:
  target:
    - x86_64-unknown-linux-gnu
    - aarch64-unknown-linux-gnu
  distro:
    - debian:12
    - ubuntu:24.04
```

Option B: Use GitHub Actions ARM runners
```yaml
jobs:
  build-aarch64:
    runs-on: ubuntu-24.04-arm  # ARM runner
```

### Key considerations

1. **`cross` dependency**: Need to install `cross` in CI
2. **Rust target**: `rustup target add aarch64-unknown-linux-gnu`
3. **Test limitations**: Cannot run aarch64 tests on x86_64 runner (need `--target` for build only)
4. **Container compatibility**: `cross` uses Docker, conflicts with current container-based CI
5. **Recommended approach**: Separate job for aarch64 cross-build (build-only, no test execution)

## 9. cargo-deny and cargo-audit Status

### cargo-deny

- `deny.toml` EXISTS with configuration:
  - Licenses: MIT, Apache-2.0 allowed (confidence 0.8)
  - Advisories: v2, yanked = warn
  - Bans: multiple-versions = warn, wildcards = warn
  - Sources: unknown registry/git = warn
- **NEVER RUN IN CI** — no step executes `cargo deny check`
- May have issues: workspace uses `insta`, `assert_cmd`, `predicates` which need license verification

### cargo-audit

- **NOT CONFIGURED** — no `audit.toml` exists
- **NEVER RUN IN CI**
- PRD section 6.2 requires it

### Recommended CI additions

```yaml
- name: Check dependencies with cargo-deny
  run: cargo deny check

- name: Security audit with cargo-audit
  run: cargo audit
```

## 10. Summary: Phase 3 Gap Analysis

### Tests to reach 80% coverage

| Area | Current | Target | Gap |
|------|---------|--------|-----|
| Auth controls (9) | 0 tests | ~36 tests (4 each) | +36 |
| Logging controls (11) | 0 tests | ~44 tests (4 each) | +44 |
| Service controls (11 macro) | 4 tests (avahi) | ~44 tests | +40 |
| Network sysctl controls (6) | 5 tests (ip_forwarding) | ~24 tests | +19 |
| Maintenance controls (8) | 3 tests (perms_passwd) | ~32 tests | +29 |
| Common modules (3) | 0 tests | ~15 tests | +15 |
| CLI commands (list, completion) | 0 tests | ~8 tests | +8 |
| CLI integration (assert_cmd) | 6 tests | ~14 tests | +8 |
| **Total estimated new tests** | **171** | **~369** | **+198** |

### Estimated effort

| Task | Effort |
|------|--------|
| Add tests to auth controls (9) | 2-3 days |
| Add tests to logging controls (11) | 2-3 days |
| Add tests to service controls (11) | 1-2 days (test common.rs once) |
| Add tests to network sysctl (6) | 1 day (test common.rs once) |
| Add tests to maintenance controls (8) | 1-2 days |
| Test common modules (3) | 1 day |
| CLI integration tests | 1 day |
| Generate insta snapshots | 0.5 day |
| Configure tarpaulin | 0.5 day |
| CI multi-arch + tools | 1-2 days |
| Vagrant setup | 1 day |
| **Total** | **~12-16 days** (within 2-week phase) |

### Risks

1. **Controls that modify system state** (PAM, SSH, sysctl) are hard to test without root — many use `with_paths()` for testability but not all
2. **Macro-generated controls** (services, maintenance perms) need tests on the common struct, not each instance
3. **tarpaulin in containers** may not work — needs `--engine llvm` or bare runner
4. **aarch64 cross-build** adds CI complexity and time
5. **Rust 1.75 MSRV** may limit tool versions (tarpaulin, insta compatibility)
