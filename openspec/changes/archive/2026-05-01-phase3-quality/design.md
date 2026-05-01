# Design: Phase 3 — Quality

## Technical Approach

Phase 3 brings test coverage to ≥80% by adding fixture-based unit tests for all 70 CIS controls, property-based tests for config parsers, insta snapshot generation, assert_cmd integration tests, CI multi-arch builds, cargo-tarpaulin coverage, cargo-deny enforcement, and Vagrant VMs for manual smoke testing. The strategy: test common structs once (ServiceDisable, SysctlControl, SshdConfigControl, def_perm_control!) rather than per-control, then add a lightweight 2-test harness per macro-generated control for ID/description verification only.

## Architecture Decisions

### Decision: Test Strategy Per Control

**Choice**: Fixture-based unit tests with `tempfile` + `Context::with_paths()` injection; property-based tests for config parsing
**Alternatives**: Mock filesystem (too complex), Docker integration tests (too slow per PR)
**Rationale**: Every control already exposes `with_path()`/`with_paths()` constructors — this is the DI seam. `tempfile::NamedTempFile` + `Context::with_paths()` is the existing pattern used by `ensure_perms_passwd`, `disable_root_login`, `sysctl_ip_forwarding`. Property-based (proptest) adds value for config parsers that must handle arbitrary content (PAM, sysctl, sshd_config).

### Decision: Macro-Generated Control Testing

**Choice**: Test the common struct thoroughly (ServiceDisable → ~8 tests, SysctlControl → ~10 tests, def_perm_control! → test EnsurePermsShadow as representative + macro ID verification loop). Each macro invocation gets 2 tests only (id + description).
**Alternatives**: 38 duplicated test suites (one per macro control) — rejected for maintenance burden
**Rationale**: The `def_service_control!` and `def_perm_control!` macros generate thin wrappers that delegate 100% to the common struct. Testing `ServiceDisable` covers check/apply/rollback/dry-run for ALL 12 service controls. A registry-level test verifies each macro invocation produces the correct ID and description.

### Decision: CI aarch64 Strategy

**Choice**: Separate `build-aarch64` job using `cross` (cross-rs) for compilation-only verification. No aarch64 test execution in CI.
**Alternatives**: GitHub ARM runners (requires paid team plan or self-hosted); QEMU emulation (too slow)
**Rationale**: `cross` builds aarch64 binaries inside Docker containers. We verify compilation and linking succeed, but cannot run tests on aarch64 without ARM hardware. This is standard for Rust cross-compilation projects.

### Decision: cargo-tarpaulin Configuration

**Choice**: Run tarpaulin on x86_64 only, output HTML + JSON, exclude `main.rs` entrypoint. Use `--engine llvm` fallback if ptrace fails in containers.
**Alternatives**: Codecov integration only (loses local HTML reports); lcov only (less readable)
**Rationale**: Tarpaulin's ptrace engine doesn't work in all CI containers. LLVM engine is more portable. HTML reports are human-readable; JSON enables CI threshold gates. Excluding CLI entrypoint avoids false negatives.

### Decision: cargo-deny CI Policy

**Choice**: Run on every PR (advisories + licenses) and weekly schedule (full check including bans + sources). Fail CI on advisories and unlicensed crates.
**Alternatives**: Run full check on every PR (too strict, blocks PRs on unrelated yanked warns); advisory-only (misses license issues)
**Rationale**: PR-blocking on advisories and licenses catches security issues early. Bans (duplicate versions) and sources (unknown registries) are warnings better reviewed weekly.

### Decision: Vagrant Configuration

**Choice**: Single `Vagrantfile` with multi-machine definitions using `generic/debian12`, `generic/ubuntu2404`, `generic/rocky9` boxes. Shell provisioner installs Rust via rustup, clones repo, builds, and runs `vallumix audit`.
**Alternatives**: Separate Vagrantfiles per distro (duplication); Ansible provisioner (overkill for smoke tests)
**Rationale**: A single multi-machine Vagrantfile is easier to maintain. `generic/*` boxes are well-maintained and track upstream. Shell provisioner is sufficient — Rust installation is deterministic via rustup.

### Decision: insta Snapshot Workflow

**Choice**: `cargo insta test` auto-generates missing `.snap` files. `cargo insta review` for human review. CI runs `cargo insta test --require-snapshots` to fail if snapshots are missing or changed without review.
**Alternatives**: Commit snapshots without review (defeats purpose); only CI check (developer UX poor)
**Rationale**: insta's workflow is: write test → `cargo insta test` (generates pending snapshots) → `cargo insta review` (accept/reject). CI `--require-snapshots` ensures no unreviewed changes slip in. Already 4 snapshot tests exist but have zero `.snap` files.

## Data Flow

```
Developer writes test
       │
       ▼
cargo test --workspace
  ├── Unit tests (tempfile fixtures + Context::with_paths)
  ├── Property tests (proptest::strategy for config parsing)
  ├── insta snapshots (reporters)
  └── assert_cmd (CLI integration)
       │
       ▼
cargo tarpaulin (x86_64) → coverage/ directory
       │
       ▼
CI Pipeline (GitHub Actions)
  ├── build-and-test (4 distros × x86_64)
  ├── build-aarch64 (cross compile only)
  ├── coverage (tarpaulin, x86_64)
  ├── deny (advisories + licenses, PR trigger)
  └── deny-full (bans + sources, weekly)

Vagrant (local manual testing)
  ├── debian12 VM
  ├── ubuntu2404 VM
  └── rocky9 VM
       │
       ▼
vallumix audit → verify controls on real systems
```

## File Changes

| File | Action | Description |
|------|--------|-------------|
| `crates/vallumix-controls/src/services/common.rs` | Modify | Add `#[cfg(test)]` module with 8 tests for ServiceDisable |
| `crates/vallumix-controls/src/network/common.rs` | Modify | Add `#[cfg(test)]` module with 10 tests for SysctlControl |
| `crates/vallumix-controls/src/ssh/common.rs` | Modify | Add `#[cfg(test)]` module with 8 tests for SshdConfigControl |
| `crates/vallumix-controls/src/maintenance/mod.rs` | Modify | Add 4 tests for def_perm_control! (representative) + 1 registry test |
| `crates/vallumix-controls/src/services/mod.rs` | Modify | Add registry test verifying all 12 macro controls have correct IDs |
| `crates/vallumix-controls/src/auth/mod.rs` | Modify | Add `#[cfg(test)]` module with ~36 tests across 9 controls |
| `crates/vallumix-controls/src/logging/mod.rs` | Modify | Add `#[cfg(test)]` module with ~44 tests across 11 controls |
| `crates/vallumix-controls/src/ssh/*.rs` | Modify | Add tests for disable_root_login edge cases, ssh_limit_access, ssh_set_banner |
| `crates/vallumix-controls/src/network/*.rs` | Modify | Add tests for remaining 7 sysctl controls |
| `crates/vallumix-controls/src/maintenance/*.rs` | Modify | Add tests for AuditWorldWritable, AuditSuidSgid, etc. |
| `crates/vallumix-cli/tests/cli.rs` | Modify | Add ~8 more assert_cmd integration tests |
| `crates/vallumix-cli/src/commands/list.rs` | Modify | Add unit tests for list command |
| `crates/vallumix-cli/src/commands/completion.rs` | Modify | Add unit tests for completion command |
| `crates/vallumix-reporters/src/*/snapshots/` | Create | Generate initial `.snap` files for 4 reporter snapshot tests |
| `.github/workflows/ci.yml` | Modify | Add tarpaulin, deny, aarch64 cross-build jobs |
| `.github/workflows/deny-schedule.yml` | Create | Weekly cargo-deny full check (bans + sources) |
| `.tarpaulin.toml` | Create | Coverage config: exclude main.rs, output Html+Json, timeout 300s |
| `Vagrantfile` | Create | Multi-machine VM definitions for debian12, ubuntu2404, rocky9 |
| `Cargo.toml` | Modify | Add proptest to workspace dev-dependencies |
| `crates/vallumix-controls/Cargo.toml` | Modify | Add proptest dev-dependency |

## Interfaces / Contracts

### Test Helper Pattern (common.rs modules)

```rust
// In each module's #[cfg(test)] mod tests:
fn test_context() -> Context {
    Context::with_paths(
        "testhost".into(), Distro::Debian12,
        "/tmp/vallumix-test".into(),
        "/tmp/vallumix-test/backup".into(),
        "/tmp/vallumix-test/profiles".into(),
        false, // dry_run = false
    )
}

fn dry_context() -> Context {
    Context::with_paths(
        "testhost".into(), Distro::Debian12,
        "/tmp/vallumix-test".into(),
        "/tmp/vallumix-test/backup".into(),
        "/tmp/vallumix-test/profiles".into(),
        true, // dry_run = true
    )
}
```

### Macro Control Verification Test

```rust
#[test]
fn all_service_controls_have_correct_ids() {
    let controls: Vec<Box<dyn Control>> = vec![
        Box::new(disable_cups::DisableCups::new()),
        Box::new(disable_dhcp::DisableDhcp::new()),
        // ... all 12
    ];
    let expected = vec!["2.2.2", "2.2.4", "2.2.5", ...];
    for (ctrl, id) in controls.iter().zip(expected.iter()) {
        assert_eq!(ctrl.id(), *id);
    }
}
```

## Testing Strategy

| Layer | What to Test | Approach |
|-------|-------------|-----------|
| Unit | Auth controls (9) | Fixture-based: create temp files with compliant/non-compliant content, use `with_path()` to inject paths |
| Unit | Logging controls (11) | Fixture-based: temp config files (syslog, journald, auditd, logrotate) |
| Unit | Service controls (12 macro) | Test `ServiceDisable` common struct (8 tests) + ID/description verification loop |
| Unit | Sysctl controls (7) | Test `SysctlControl` common struct (10 tests) + per-control fixture for check values |
| Unit | Maintenance controls (8) | Test `def_perm_control!` via EnsurePermsShadow (4 tests) + audit controls + EnsureCronPerms |
| Unit | SSH controls (3) | Test `SshdConfigControl` common struct (8 tests) + disable_root_login edge cases |
| Property | Config parsers (PAM, sysctl, sshd) | proptest: arbitrary file content → parse never panics, compliant ⇔ expected content |
| Integration | CLI commands via assert_cmd | ~14 total tests: audit report formats, apply dry-run, list, completion, threshold, error paths |
| Snapshot | Reporter output | 4 insta snapshots (already written, need .snap file generation) |
| CI | Multi-arch build | cross-rs aarch64 cross-compile (build-only, no test execution) |
| CI | Coverage threshold | cargo-tarpaulin → HTML + JSON, goal ≥80% |
| CI | Dependency audit | cargo-deny: advisories + licenses on PR, full check weekly |
| Manual | Vagrant smoke tests | 3 VMs: debian12, ubuntu2404, rocky9 |

## Migration / Rollout

No migration required. This phase adds tests and CI configuration without changing production code behavior.

## Open Questions

- [ ] Should proptest be a workspace-level dev-dependency or per-crate? (Recommendation: workspace-level for consistency)
- [ ] Minimum tarpaulin coverage threshold for CI gate: 70% initially, bump to 80% after all tests are added? (Recommendation: start at 70%, raise in separate commit)