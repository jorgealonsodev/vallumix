# Proposal: Phase 1 — Foundations (Cimientos)

## Intent

Phase 0 delivered stubs. Phase 1 makes vallumix *runnable*: real types, real CLI, real controls, real output. Produces v0.1 alpha — a binary that audits and applies 5 CIS controls on a live system.

## Scope

### In Scope
- **`vallumix-core`**: `CheckResult`/`ApplyResult` fields, `Context` struct, `distro::detect()` via `/etc/os-release`, `Profile` struct + TOML deser, `VallumixError`
- **`vallumix-cli`**: clap derive, 5 subcommands, global flags (`--dry-run`, `--verbose`, `--quiet`, `--threshold`, `--profile`)
- **`vallumix-controls`**: 5 pilots — `disable_cramfs`, `sysctl_ip_forwarding`, `ensure_auditd_installed`, `ssh_disable_root_login`, `ensure_perms_passwd`
- **`vallumix-reporters`**: JSON reporter via `serde_json`
- **`profiles/web.toml`**: 5 pilot control IDs

### Out of Scope
- HTML/JUnit/text reporters, `vallumix-backup`, `rayon` parallelism (Phase 2)
- `miette`, `insta`, `assert_cmd` (Phase 3)
- Shell completion logic (stub only)

## Capabilities

### New Capabilities
- `distro-detection`, `execution-context`, `error-types`, `profile-model`: core type completions
- `cli-structure`: clap derive + 5 subcommands + tracing init
- `pilot-controls`: 5 `Control` impls across all CIS domains
- `json-reporter`, `web-profile`: output and profile config

### Modified Capabilities
- `core-traits`: `CheckResult`/`ApplyResult` gain real fields; `Reporter` trait gains `generate()`; `Profile` trait → concrete struct

## Approach

**Incremental bottom-up**: core → reporters → controls → CLI → profile. Each crate compiles before the next depends on it.

## Affected Areas

| Area | Impact |
|------|--------|
| `crates/vallumix-core/src/{control,context,distro,profile,error,lib}.rs` | Modified |
| `crates/vallumix-cli/src/main.rs` | Rewritten |
| `crates/vallumix-controls/src/lib.rs` | 5 control modules + registry |
| `crates/vallumix-reporters/src/lib.rs` | JSON reporter |
| `profiles/web.toml` | Control IDs added |
| `Cargo.toml` (workspace) | +`anyhow`, `nix`, `walkdir`, `tempfile`, `owo-colors`, `indicatif` |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| MSRV 1.75 blocks newer deps | Medium | Pin `nix` 0.27.x, verify all deps |
| Root required for testing | High | Mock `Context` for unit tests; containers for integration |
| `nix` crate API instability | Medium | Pin version, thin wrapper |
| Idempotency bugs | Medium | Check state before modifying in each `apply()` |

## Rollback Plan

Per-crate `git checkout`. Workspace dep additions are additive (safe to remove). Profile TOMLs are standalone files.

## Dependencies

- Phase 0 complete ✅
- MSRV-compatible: `nix` 0.27, `walkdir`, `tempfile`, `owo-colors`, `indicatif`

## Success Criteria

- [ ] `cargo build` — zero warnings across 5 crates
- [ ] `cargo test` — all existing + new tests pass
- [ ] `cargo clippy -- -D warnings` — clean
- [ ] `vallumix audit --profile web` — produces JSON on real system
- [ ] 5 pilot controls pass `check()` and `apply()` independently
- [ ] `profiles/web.toml` loads and resolves 5 control IDs
