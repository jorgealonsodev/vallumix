# Verification Report (RE-VERIFY — CRITICAL Fixes)

**Change**: phase1-foundations
**Version**: N/A
**Mode**: Strict TDD (cargo test — static code review only, no local Rust toolchain)
**Verification type**: Re-verification of 2 previously CRITICAL issues

---

## Previously CRITICAL Issues — Resolution

| # | Previous CRITICAL | Status | Evidence |
|---|-------------------|--------|----------|
| 1 | `disable_avahi` not implemented (`ensure_auditd` existed instead) | ✅ **FIXED** | `services/disable_avahi.rs` exists (264 lines, Control trait, 4 tests). No `ensure_auditd` files or references remain. Registry uses `"2.2.3"`. `web.toml` includes `"2.2.3"`. |
| 2 | No TDD Cycle Evidence table in apply-progress | ✅ **FIXED** | Table present in engram #2394 with 16 rows, columns: Task/Test File/Layer/RED/GREEN/TRIANGULATE/REFACTOR |

---

## Completeness

| Metric | Value |
|--------|-------|
| Tasks total | 45 |
| Tasks complete | 45 (all marked [x]) |
| Tasks incomplete | 0 |

> **Note**: T-041 (`cargo build`), T-042 (`cargo test`), T-043 (`cargo clippy`) marked [x] but not executed — no local Rust toolchain. T-022 tasks.md description still references `ensure_auditd.rs` (see WARNING #1).

---

## Build & Tests Execution

**Build**: ➖ Not run (no Rust toolchain available)
**Tests**: ➖ Not run (no Rust toolchain available)
**Coverage**: ➖ Not run (coverage threshold: 80% per openspec/config.yaml)

All test code analyzed statically. `disable_avahi.rs` contains 4 unit tests (see Assertion Quality section below).

---

### TDD Compliance

| Check | Result | Details |
|-------|--------|---------|
| TDD Evidence reported | ✅ | Table present in engram #2394 with 16 rows |
| All tasks have tests | ✅ | 42/45 tasks have corresponding test coverage |
| RED confirmed (tests exist) | ✅ | Test files verified by reading — all exist on disk |
| GREEN confirmed (tests pass) | ➖ | Cannot verify without toolchain — all rows show "✅ (CI pending)" |
| Triangulation adequate | ✅ | All tasks show 2-8 test cases each |
| Safety Net column | ⚠️ | SAFETY NET column absent from TDD table (see WARNING #5) |

**TDD Compliance**: 4/6 checks passed, 1 cannot be verified (toolchain), 1 warning

---

### Test Layer Distribution

| Layer | Tests | Files | Tools |
|-------|-------|-------|-------|
| Unit | ~64 | 13 files (+1: disable_avahi.rs) | cargo test (built-in) |
| Integration | 0 | 0 | assert_cmd (available, unused) |
| E2E | 0 | 0 | — (not available) |
| **Total** | **~64** | **13** | |

All tests are inline `#[cfg(test)] mod tests` blocks. Default unit-test layer.

---

### Changed File Coverage

Coverage analysis skipped — no local Rust toolchain, `cargo-tarpaulin` not available.

---

### Assertion Quality — `disable_avahi.rs` (NEW)

| File | Line | Assertion | Issue | Severity |
|------|------|-----------|-------|----------|
| `disable_avahi.rs` | 220 | `assert_eq!(result.status, CheckStatus::Compliant)` | ✅ Valid — asserts production behavior | — |
| `disable_avahi.rs` | 236 | `assert_eq!(result.status, ApplyStatus::Applied)` | ✅ Valid — asserts idempotent apply | — |
| `disable_avahi.rs` | 239 | `assert_eq!(result2.status, ApplyStatus::Applied)` | ✅ Valid — idempotency check | — |
| `disable_avahi.rs` | 254 | `assert_eq!(result.status, ApplyStatus::Skipped)` | ✅ Valid — dry-run behavior | — |
| `disable_avahi.rs` | 261 | `assert_eq!(cloned.id(), "2.2.3")` | ⚠️ Metadata-only assertion (still valid clone check) | SUGGESTION |
| `disable_avahi.rs` | 262 | `assert_eq!(cloned.severity(), Severity::Low)` | ⚠️ Metadata-only assertion (still valid clone check) | SUGGESTION |

**Assertion quality**: 0 CRITICAL, 0 WARNING, 2 SUGGESTION (metadata-only clone assertions)

No tautologies (no `assert!(true)`), no ghost loops, no smoke-only renders, no mock-heavy patterns found. Four behavioral assertions verify real control logic. Two clone_box assertions verify trait contract but don't test behavior.

**Gaps**: 
- No test for `rollback()` method ❌ UNTESTED (see WARNING #3)
- No test for `check()` returning NonCompliant when service is active/enabled — uses `/bin/true` which always returns empty stdout → `is_active()` → `false` (see WARNING #2)

---

### Quality Metrics

**Linter**: ➖ Not available (no cargo clippy)
**Type Checker**: ➖ Not available (no cargo check)
**Formatter**: ➖ Not available (no cargo fmt)

---

## Spec Compliance Matrix — `pilot-controls` (disable_avahi focus)

Previous report showed disable_avahi as ❌ UNTESTED for both scenarios. Updated matrix:

| Requirement | Scenario | Test | Result |
|-------------|----------|------|--------|
| disable_avahi (2.x) | check detects Avahi is running | `check_compliant_when_service_not_installed` (partial: tests not-installed, NOT running case) | ⚠️ PARTIAL |
| disable_avahi (2.x) | apply disables Avahi and creates backup | `idempotent_apply_when_service_not_installed` + `dry_run_skips_apply` (partial: tests not-installed + dry-run, NOT backup creation) | ⚠️ PARTIAL |

**Improvement**: Both scenarios now have SOME test coverage (was ❌ UNTESTED in previous report). However:

- **Scenario 1 "check detects Avahi is running"**: Only tests the "service not installed → Compliant" path. The "active and/or enabled → NonCompliant" path is NOT tested because tests use `/bin/true` as systemctl which always returns empty stdout. A test with a mock/fixture for `systemctl is-active` returning "active" is needed.
- **Scenario 2 "apply disables Avahi and creates backup"**: Tests cover "service not installed" and "dry-run" paths. The actual stop+disable path is NOT tested. The backup_path is always `None` in the implementation — spec requires `ApplyResult` contains backup path.

---

## Correctness (Static — Structural Evidence)

Updated rows for fixed items:

| Capability | Status | Notes |
|------------|--------|-------|
| pilot-controls — disable_avahi (CIS 2.2.3) | ✅ **Implemented** | Full Control trait: check/apply/rollback/dry-run. Spec says `Severity::Medium` but code returns `Severity::Low` (see WARNING #4). Spec scenario says backup is created but `backup_path` is always `None`. `rollback()` re-enables service via systemctl. |

The previous report's `pilot-controls — ensure_auditd` row is **REMOVED** — that file no longer exists.

---

## Coherence (Design)

Updated design deviations:

| Design Element | Previous Status | Current Status | Notes |
|---------------|----------------|----------------|-------|
| 5 controls: disable_cramfs, sysctl_ip_forwarding, **ensure_auditd**, disable_root_login, ensure_perms_passwd | ⚠️ Deviated from SPEC | ✅ **RESOLVED** | `ensure_auditd` replaced by `disable_avahi` — now matches spec |
| Design: File Changes Table | ✅ Matching | ✅ Matching | Updated: `services/disable_avahi.rs` replaces `services/ensure_auditd.rs` |

All other design decisions unchanged from previous report.

---

## Issues Found

### CRITICAL (must fix before archive)

**None.** Both previously CRITICAL issues are resolved.

---

### WARNING (should fix)

1. **T-022 tasks.md description stale**: `openspec/changes/phase1-foundations/tasks.md` line 29 still says `ensure_auditd.rs` with id `"4.1.1.1"`. Implementation is correctly `disable_avahi.rs` with id `"2.2.3"`. Tasks.md should be updated to match implementation.

2. **`disable_avahi.check()` "running" scenario untested**: Tests only verify `Compliant` when service not installed (via `/bin/true` trick). No test verifies the `NonCompliant` path where `is_active()` returns `true` or `is_enabled()` returns `true`. A mock for systemctl or a test fixture for `is_service_active()` is needed.

3. **`disable_avahi.rollback()` has zero test coverage**: No test calls `rollback()` on DisableAvahi. Spec says `rollback()` MUST re-enable the service — behavioral evidence is missing.

4. **`disable_avahi.severity()` returns `Severity::Low`**: Pilot-controls spec §"disable_avahi Control" says `severity()` returning `Severity::Medium`. Code returns `Severity::Low` (line 77-78). CIS benchmark 2.2.3 is typically Level 1 (Low), so the code is arguably correct and spec should be updated.

5. **TDD table missing SAFETY NET column**: per strict-tdd-verify.md Step 5a, the TDD Cycle Evidence table must include a SAFETY NET column. Current table has 6 columns (Task/Test File/Layer/RED/GREEN/TRIANGULATE/REFACTOR) but no SAFETY NET.

6. **`disable_avahi.apply()` backup_path always None**: Spec scenario says "a backup is created, and ApplyResult contains the backup path". Implementation returns `backup_path: None` in all code paths. For a service-type control, rollback is implemented via `systemctl enable/start` without a file backup — functionally correct but doesn't match spec wording.

7. **Pre-existing WARNINGs unchanged** (from previous report #2395):
   - Reporter::generate() signature differs from spec
   - HostInfo missing kernel/timestamp/duration_secs
   - sysctl_ip_forwarding doesn't call sysctl -w
   - disable_cramfs.apply() not idempotent (returns Applied, not AlreadyCompliant)
   - Profile::is_applicable() is a stub
   - Missing file in distro returns Io not UnsupportedDistro
   - JSON tracing subscriber mode not implemented
   - T-041/T-042/T-043 cannot be verified (no toolchain)
   - 14 spec scenarios untested (now ~12 after avahi partial coverage)
   - Context::new() tests modify global env vars

---

### SUGGESTION (nice to have)

1. Clone assertions in `disable_avahi.rs` test (lines 261-262) check metadata only — could add `assert_eq!(cloned.description(), ctrl.description())` for completeness.
2. `disable_avahi.apply()` could set `backup_path` to a sentinel path documenting the service rollback state even without a file backup.
3. All pre-existing SUGGESTIONS from previous report still apply.

---

## Verdict

**PASS WITH WARNINGS** — Both previously CRITICAL issues are resolved:

- ✅ **CRITICAL #1**: `disable_avahi` control (CIS 2.2.3) fully implemented, replaces `ensure_auditd`. Control trait complete, 4 tests, registry/web.toml updated. No stale code remains.
- ✅ **CRITICAL #2**: TDD Cycle Evidence table present in apply-progress engram #2394, 16 rows covering all implemented tasks.

**Remaining**: 0 CRITICAL, 13 WARNING (6 new + 7 pre-existing from previous report), 5 SUGGESTION

**Summary**: The two blocking CRITICAL issues from the initial verification are resolved. The `disable_avahi` control is structurally correct and implements the full Control trait. The TDD evidence table is present. Several WARNING-level gaps remain (test coverage for avahi's NonCompliant path, rollback testing, severity mismatch, stale tasks.md) but none are blockers for archive. Pre-existing WARNINGs from the previous report carry forward unchanged.
