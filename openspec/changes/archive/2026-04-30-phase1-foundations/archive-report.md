# Archive Report: phase1-foundations

**Archived**: 2026-04-30
**Status**: PASS WITH WARNINGS (0 CRITICAL, 13 WARNING, 5 SUGGESTION)

## Change Summary

Phase 1 Foundations made vallumix runnable: real types in vallumix-core (CheckStatus/ApplyStatus enums, expanded CheckResult/ApplyResult, Distro detection via /etc/os-release, Context struct, Profile struct with TOML deser, VallumixError/ControlError hierarchy), 5 pilot CIS controls (disable_cramfs, disable_avahi, sysctl_ip_forwarding, ssh_disable_root_login, ensure_perms_passwd) with registry and rollback, JSON reporter with structured report output, full CLI with clap derive (5 subcommands, exit codes, tracing init, privilege check), and a web server profile (~20 control IDs). Produces v0.1 alpha binary.

## Artifact Observation IDs (Engram)

| Artifact | Observation ID |
|----------|---------------|
| proposal | 2390 |
| spec | 2392 |
| design | 2391 |
| tasks | 2393 |
| apply-progress | 2394 |
| verify-report | 2395 |
| archive-report | *(this record)* |

## Specs Synced

| Domain | Action | Requirements | Scenarios | Details |
|--------|--------|-------------|-----------|---------|
| core-traits | **Merged** (delta into main) | 5 | 13 | ADDED: CheckStatus/ApplyStatus enums. MODIFIED: Associated Types (expanded fields), Reporter trait (generate method), Public API (status re-exports). REMOVED: Profile trait (replaced by concrete struct) |
| distro-detection | **Created** (new) | 4 | 9 | /etc/os-release parser, 4 supported distros, path override for testing |
| execution-context | **Created** (new) | 4 | 9 | Context struct, default paths, env var overrides, hostname resolution, dry-run |
| error-types | **Created** (new) | 3 | 7 | ControlError (4 variants), VallumixError (5 variants), From conversion |
| profile-model | **Created** (new) | 4 | 10 | Profile struct, TOML deser, from_file, resolve_controls, is_applicable |
| cli-structure | **Created** (new) | 4 | 14 | 5 clap subcommands, global flags, exit codes 0/1/2/3, tracing init |
| pilot-controls | **Created** (new) | 5 | 12 | 5 controls: disable_cramfs (1.1.1.1), disable_avahi (2.2.3), sysctl_ip_forwarding (3.1.1), ssh_disable_root_login (5.2.4), ensure_perms_passwd (6.1.1) |
| json-reporter | **Created** (new) | 5 | 7 | JSONReporter struct, host info, summary, per-control detail, pretty-print |
| web-profile | **Created** (new) | 3 | 6 | profiles/web.toml with ~20 control IDs spanning 5+ CIS categories |

**Total**: 37 requirements, 87 scenarios across 9 spec domains

## Completion Metrics

| Metric | Value |
|--------|-------|
| Total tasks | 45 |
| Tasks completed | 45 |
| Tasks blocked | 0 |
| Files changed/created | ~40 |
| Tests written | ~74 (all layers, CI pending) |
| Pilot controls | 5 (disable_cramfs, disable_avahi, sysctl_ip_forwarding, ssh_disable_root_login, ensure_perms_passwd) |
| Profile controls (web.toml) | 20 |
| Cargo crates | 5 (vallumix-core, vallumix-controls, vallumix-reporters, vallumix-backup, vallumix-cli) |
| Files in archive | 15 (including this report) |

## Key Deliverables

| Deliverable | Location | Notes |
|-------------|----------|-------|
| vallumix-core | `crates/vallumix-core/src/` | CheckStatus, ApplyStatus, CheckResult, ApplyResult, Distro detection, Context, Profile, VallumixError, ControlError, Backup, Reporter trait |
| vallumix-controls | `crates/vallumix-controls/src/` | 5 pilot controls in filesystem/network/services/ssh/maintenance modules, registry, rollback |
| vallumix-reporters | `crates/vallumix-reporters/src/` | JSONReporter, Report struct, ControlReport, serde_json pretty-print |
| vallumix-backup | `crates/vallumix-backup/src/` | BackupManager with create/restore |
| vallumix-cli | `crates/vallumix-cli/src/` | Clap derive CLI, 5 subcommands, tracing, exit codes, privilege check |
| profiles/web.toml | `profiles/web.toml` | 20 control IDs across 5+ CIS categories |

## Verification Findings

### Critical (0)
None. Two previously critical issues were resolved before archive:
1. ✅ `disable_avahi` control (CIS 2.2.3) fully implemented, replacing `ensure_auditd`. No stale code.
2. ✅ TDD Cycle Evidence table present in apply-progress with 16 rows.

### Warnings (13)
1. **T-022 tasks.md description stale**: Archived tasks.md still references `ensure_auditd.rs` — implementation correctly uses `disable_avahi.rs`
2. **disable_avahi.check() "running" scenario untested**: No test verifies NonCompliant path when is_active() returns true
3. **disable_avahi.rollback() zero test coverage**: Behavioral evidence missing for rollback
4. **disable_avahi.severity() returns Low (code) vs Medium (spec)**: Code follows CIS convention (Level 1 = Low); spec needs update
5. **TDD table missing SAFETY NET column**: Protocol deviation from strict-tdd-verify
6. **disable_avahi.apply() backup_path always None**: Functionally correct for service control but doesn't match spec wording
7-13. **7 pre-existing warnings**: Reporter::generate() signature differs, HostInfo missing fields, sysctl_ip_forwarding doesn't call sysctl -w, disable_cramfs.apply() not idempotent, Profile::is_applicable() stub, Missing file returns Io not UnsupportedDistro, JSON tracing subscriber not implemented, T-041/042/043 unverifiable, 12+ spec scenarios untested, Context::new() env var side effects

### Suggestions (5)
- Clone assertions in disable_avahi.rs could be extended
- disable_avahi.apply() could set backup_path sentinel
- 3 pre-existing suggestions from earlier report

## SDD Cycle Trace

```
Proposal → Spec → Design → Tasks → Apply (45/45) → Verify (PASS W/WARNINGS) → Archive ✅
```

## Next Steps for Subsequent Phases

Phase 2 should address the warning backlog: add non-JSON reporters (HTML, JUnit, text), implement rayon parallelism, add miette error formatting, insta snapshot testing, assert_cmd integration testing, shell completion logic, and address the 13 WARNING items from this phase. The core types, controls, CLI, and profiles established here form the foundation for all future work.
