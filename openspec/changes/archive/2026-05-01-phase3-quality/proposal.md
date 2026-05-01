# Proposal: Phase 3 — Quality

## Intent

Bring the vallumix codebase to production-grade quality: close test coverage gaps across 70 CIS controls, enforce deterministic reporter output via snapshots, expand CI to catch cross-architecture build failures and dependency vulnerabilities, and provide reproducible manual testing environments. Currently ~171 tests cover only 11 of ~22 control files, zero insta snapshots exist, CI is x86_64-only, and no manual VM fixtures are available.

## Scope

### In Scope
- Add fixture-based unit tests for all untested controls (auth, logging, network sysctl, maintenance, services)
- Generate insta snapshots for the 4 reporters
- Expand CI with aarch64 cross-build, cargo-tarpaulin, cargo-deny, cargo-audit, and snapshot enforcement
- Create Vagrant VMs for manual smoke testing on Debian 12, Ubuntu 24.04, and Rocky Linux 9

### Out of Scope
- Changing control logic or behavior (tests only)
- Performance optimization or benchmarking
- Adding new CIS controls or profiles
- Docker-based integration tests

## Capabilities

### New Capabilities
- `test-coverage`: Fixture-based unit tests, property-based parser tests, shared struct tests, error-path coverage
- `insta-snapshots`: Snapshot generation, content verification, CI enforcement for 4 reporters
- `vagrant-fixtures`: Multi-VM Vagrantfile, idempotent provisioning, baseline audit execution

### Modified Capabilities
- `ci-pipeline`: Add aarch64 cross-build, cargo-tarpaulin coverage gate, cargo-deny/advisory checks, insta snapshot validation

## Approach

Test common structs once (`ServiceDisable`, `SysctlControl`, `SshdConfigControl`, `def_perm_control!`) rather than duplicating per macro-generated control. Use `tempfile` + `Context::with_paths()` for filesystem injection. Add lightweight registry tests verifying ID/description for each macro invocation. Generate missing `.snap` files via `cargo insta test --accept`. Run `cross` for aarch64 build-only verification. Gate CI on 80% tarpaulin coverage with `--engine llvm` fallback. Run cargo-deny on PRs (advisories+licenses) and weekly (bans+sources).

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `crates/vallumix-controls/src/{auth,logging,network,maintenance,services,ssh}/` | Modified | Add `#[cfg(test)]` modules |
| `crates/vallumix-cli/tests/cli.rs` | Modified | Expand assert_cmd integration tests |
| `crates/vallumix-reporters/src/*/snapshots/` | Created | Initial `.snap` files |
| `.github/workflows/ci.yml` | Modified | New jobs: aarch64, tarpaulin, deny, audit, insta |
| `.github/workflows/deny-schedule.yml` | Created | Weekly full cargo-deny check |
| `.tarpaulin.toml` | Created | Coverage config |
| `Vagrantfile` + `scripts/` | Created | Multi-machine VM definitions |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| tarpaulin ptrace fails in CI containers | Med | Use `--engine llvm` fallback; run on bare `ubuntu-latest` |
| aarch64 cross-build increases CI time | Med | Separate job, build-only, cache target |
| Vagrant box download failures | Low | Pin `generic/*` boxes; document `vagrant box update` |
| Macro control tests become brittle | Low | Test struct behavior once; registry test only checks metadata |

## Rollback Plan

All changes are additive (tests, CI config, snapshots, Vagrant). Rollback: revert the commit range for this change. No production behavior or data migration is affected.

## Dependencies

- `cross` (for aarch64 builds)
- `cargo-tarpaulin`, `cargo-deny`, `cargo-audit`, `cargo-insta` (CI tools)
- Vagrant + VirtualBox/VMware (local manual testing)

## Success Criteria

- [ ] `cargo test --workspace` passes with ≥200 tests
- [ ] `cargo tarpaulin` reports ≥80% line coverage
- [ ] All 4 reporter `.snap` files generated and committed
- [ ] CI green on x86_64 matrix + aarch64 cross-build
- [ ] `cargo insta test --require-snapshots` passes in CI
- [ ] Vagrant `vallumix audit --profile web` succeeds on all 3 VMs
