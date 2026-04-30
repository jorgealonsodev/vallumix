# Proposal: Phase 0 — Bootstrap

## Intent

Vallumix is a greenfield Rust project with zero code. Phase 0 establishes the complete workspace skeleton — all 5 crates as compilable stubs, CI pipeline, governance scaffolding, and core trait definitions — so Phase 1 can immediately begin implementing controls against a validated foundation.

## Scope

### In Scope
- Cargo workspace with 5 crates: `vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`, `vallumix-cli`
- Core trait stubs in `vallumix-core`: `Control`, `Reporter`, `Profile` + associated types (`Severity`, `Distro`, `CheckResult`, `ApplyResult`, `ControlError`, `Context`, `Backup`)
- CI pipeline (GitHub Actions): `cargo build`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check` — x86_64 only, multi-distro matrix (Debian 12, Ubuntu 22.04, Ubuntu 24.04, Rocky 9)
- Governance: `.github/CODEOWNERS`, issue templates (bug/feature), PR template, dependabot
- Tooling config: `rust-toolchain.toml` (MSRV 1.75), `deny.toml`, `.gitignore`
- Docs: `README.md`, `LICENSE-MIT`, `LICENSE-APACHE`, `CONTRIBUTING.md`, `CHANGELOG.md`
- Profile placeholders: `profiles/web.toml`, `profiles/database.toml`, `profiles/bastion.toml`

### Out of Scope
- aarch64 CI (deferred to Phase 3 per PRD)
- Docker files (deferred to Phase 4)
- Actual control implementations (Phase 1)
- `cargo-tarpaulin` coverage in CI (Phase 3)

## Capabilities

### New Capabilities
- `workspace-bootstrap`: Cargo workspace structure, crate stubs, toolchain config, `.gitignore`
- `ci-pipeline`: GitHub Actions workflow with build/test/lint/fmt matrix
- `governance-scaffolding`: CODEOWNERS, issue/PR templates, dependabot, licenses, contributing guide
- `core-traits`: Trait definitions (`Control`, `Reporter`, `Profile`) and associated types in `vallumix-core`

### Modified Capabilities
None — greenfield project, no existing specs.

## Approach

**Monolithic bootstrap** (Approach 1 from exploration): create everything in one change. All 5 crates are stubs (`lib.rs` with module declarations, `main.rs` for CLI with `fn main`). The `vallumix-core` crate contains the architectural foundation: `control.rs` with the `Control` trait from the PRD, `error.rs` with `ControlError` via `thiserror`, `context.rs` and `profile.rs` as empty modules, `distro.rs` as empty module.

CI uses `ubuntu-latest` runner with a container matrix for distro-specific builds. Each crate compiles independently; no inter-crate dependencies beyond `vallumix-core` at this stage.

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `Cargo.toml` (root) | New | Workspace definition with `[workspace.dependencies]` |
| `crates/vallumix-core/` | New | Core lib with trait stubs |
| `crates/vallumix-controls/` | New | Empty lib stub |
| `crates/vallumix-reporters/` | New | Empty lib stub |
| `crates/vallumix-backup/` | New | Empty lib stub |
| `crates/vallumix-cli/` | New | Binary stub with `fn main` |
| `.github/` | New | CI workflow, templates, CODEOWNERS, dependabot |
| `profiles/` | New | Placeholder TOML files |
| Root docs | New | README, licenses, CONTRIBUTING, CHANGELOG |
| `rust-toolchain.toml` | New | MSRV 1.75 pin |
| `deny.toml` | New | cargo-deny config |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| No local Rust toolchain — cannot verify builds | High | CI is sole validation; carefully construct Cargo.toml files |
| MSRV 1.75 constrains dependency versions | Medium | Pin deps compatible with 1.75; bump if justified |
| Workspace member misconfiguration | Low | Each crate tested independently in CI matrix |

## Rollback Plan

Greenfield — delete all created files and revert to the single-commit state. No existing code is modified.

## Dependencies

- GitHub repository must exist with `develop` branch as default
- GitHub Actions enabled on the repository

## Success Criteria

- [ ] `cargo build` succeeds for all 5 crates on CI
- [ ] `cargo test` passes (even if tests are trivial)
- [ ] `cargo clippy -- -D warnings` produces zero warnings
- [ ] `cargo fmt --check` passes
- [ ] CI matrix runs on all 4 distros (Debian 12, Ubuntu 22.04, Ubuntu 24.04, Rocky 9)
- [ ] All trait stubs in `vallumix-core` compile with correct signatures from PRD
- [ ] GitHub issue/PR templates render correctly
