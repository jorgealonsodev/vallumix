# Exploration: Phase 1 — Cimientos (Foundations)

## Current State

The project is at **Phase 0 Bootstrap** — a Cargo workspace with 5 crates, all containing stub/placeholder code. The workspace compiles and has a CI pipeline, governance scaffolding, and core trait definitions as empty shells.

### What Exists Today

**Workspace root** (`Cargo.toml`):
- 5 crates: `vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`, `vallumix-cli`
- Workspace dependencies: `clap`, `serde`, `serde_json`, `thiserror`, `tracing`, `tracing-subscriber`, `toml`
- MSRV: Rust 1.75, edition 2021

**`vallumix-core`** (the most developed crate):
- `control.rs`: Trait `Control` defined with `id()`, `description()`, `severity()`, `applicable_distros()`, `check()`, `apply()`, `rollback()`. Enums `Severity` (Low/Medium/High), `CheckResult` (empty struct), `ApplyResult` (empty struct).
- `error.rs`: `ControlError` enum with `NotApplicable`, `BackupFailed`, `PostCheckFailed`, `Io` variants.
- `distro.rs`: `Distro` enum with `Debian12`, `Ubuntu2204`, `Ubuntu2404`, `Rocky9`. `detect()` function returns hardcoded `Distro::Rocky9`.
- `context.rs`: `Context` struct — empty, no fields.
- `profile.rs`: `Backup` struct (empty), `Reporter` trait (empty), `Profile` trait (empty).
- `lib.rs`: Re-exports all modules, has 7 placeholder tests (smoke test, trait object safety, enum existence, profile TOML parsing).

**`vallumix-cli`**: `main.rs` prints "vallumix-cli" — no clap setup, no subcommands.

**`vallumix-controls`**: Empty lib with a comment.

**`vallumix-reporters`**: Empty lib with a comment.

**`vallumix-backup`**: Empty lib with a comment.

**Profiles** (`profiles/*.toml`): All 3 profiles (web, database, bastion) exist with name/description but empty `controls = []`.

## Affected Areas

| File | Why It's Affected |
|------|-------------------|
| `crates/vallumix-core/src/control.rs` | `CheckResult` and `ApplyResult` need real fields (status, evidence, details) |
| `crates/vallumix-core/src/context.rs` | `Context` needs fields (hostname, distro, kernel, working paths, config) |
| `crates/vallumix-core/src/distro.rs` | `detect()` must parse `/etc/os-release` instead of returning hardcoded value |
| `crates/vallumix-core/src/profile.rs` | `Profile` trait needs concrete model (name, description, control list, TOML parsing) |
| `crates/vallumix-core/src/error.rs` | May need additional error variants (ProfileLoad, Privilege, etc.) |
| `crates/vallumix-core/Cargo.toml` | Needs new dependencies for distro detection, profile parsing |
| `crates/vallumix-cli/src/main.rs` | Complete rewrite: clap derive, 5 subcommands, global flags |
| `crates/vallumix-cli/Cargo.toml` | Needs `anyhow`, `tracing-subscriber`, `owo-colors`, `indicatif` |
| `crates/vallumix-controls/src/lib.rs` | Needs registry function + 5 pilot control implementations |
| `crates/vallumix-controls/Cargo.toml` | Needs `nix`, `walkdir`, `tempfile` |
| `crates/vallumix-reporters/src/lib.rs` | Needs JSON reporter implementation (Phase 1 scope) |
| `crates/vallumix-reporters/Cargo.toml` | Needs `serde_json` for JSON output |
| `profiles/web.toml` | Must list the 5 pilot control IDs |
| `Cargo.toml` (workspace) | Add missing workspace dependencies |

## Approaches

### 1. Approach: Incremental Module-by-Module (Recommended)

Build each crate to a "Phase 1 complete" state in dependency order:
1. `vallumix-core` → complete types, distro detection, profile model, context
2. `vallumix-reporters` → JSON reporter only
3. `vallumix-controls` → 5 pilot controls + registry
4. `vallumix-cli` → clap structure, subcommand stubs, wiring

- **Pros**: Clear dependency chain, each crate compiles before moving on, easy to verify
- **Cons**: Sequential, cannot parallelize work across crates
- **Effort**: Medium (3 weeks per PRD)

### 2. Approach: Skeleton-First, Fill-In Later

Create the full CLI structure with all subcommands wired to stub implementations, then fill in each crate.

- **Pros**: End-to-end structure visible early, easier to see integration points
- **Cons**: More refactoring needed as types evolve, risk of mismatched interfaces
- **Effort**: Medium-High

### 3. Approach: Vertical Slices

Implement one complete control end-to-end (core types → control → reporter → CLI), then repeat for remaining 4.

- **Pros**: Each slice is independently testable, validates the full pipeline early
- **Cons**: Requires more upfront type design, may lead to rework if types change
- **Effort**: High

## Recommendation

**Approach 1 (Incremental Module-by-Module)** is the best fit. The PRD already defines the architecture clearly, and the trait signatures are established. Building bottom-up ensures each layer compiles and is testable before the next layer depends on it.

## What Needs to Be Built (Phase 1 Scope)

### 1. `vallumix-core` — Complete the Foundation

| Module | Current | Phase 1 Target |
|--------|---------|----------------|
| `control.rs` | `CheckResult` and `ApplyResult` are empty structs | Add fields: `status` (enum: Compliant/NonCompliant/Error/Skipped), `evidence` (String), `details` (optional String), `duration` (Duration) |
| `context.rs` | Empty struct | Fields: `hostname`, `distro`, `kernel_version`, `working_dir`, `backup_dir`, `dry_run` flag |
| `distro.rs` | Hardcoded `detect()` | Parse `/etc/os-release` to detect actual distro; return `VallumixError` if unsupported |
| `profile.rs` | Empty traits | `Profile` struct with `name`, `description`, `controls: Vec<String>`, TOML deserialization via `serde` |
| `error.rs` | `ControlError` only | Add `VallumixError` (app-level) with variants: `Privilege`, `UnsupportedDistro`, `ProfileNotFound`, `ReportGeneration` |
| `lib.rs` | Re-exports + tests | Add `VallumixError` module, remove `#![allow(missing_docs)]`, add `#[deny(missing_docs)]` |

### 2. `vallumix-cli` — Clap Structure

Implement 5 subcommands per PRD section 5.4:
- `apply` — Apply controls from a profile (with `--dry-run`, `--profile`, `--report`, `--threshold`)
- `audit` — Evaluate without changes (with `--profile`, `--report`)
- `rollback` — Restore from backup (with `--control-id` or full rollback)
- `list` — Print catalog of available controls
- `completion` — Generate shell completion scripts

Global flags: `--verbose`, `--quiet`, `--no-color`, `--log-level`

### 3. `vallumix-controls` — 5 Pilot Controls

Based on PRD sections 5.3 (web profile) and 5.6 (categories), the 5 pilot controls should cover diverse categories and be relevant to the web profile:

| # | CIS ID | Category | Control | Complexity |
|---|--------|----------|---------|------------|
| 1 | `1.1.1.1` | Filesystem | Disable cramfs (unused filesystem) | Low — modprobe config |
| 2 | `3.1.1` | Network | Disable IP forwarding (sysctl) | Low — read/write sysctl |
| 3 | `4.1.1.1` | Logging | Ensure auditd is installed | Medium — package check |
| 4 | `5.2.4` | SSH | Disable SSH root login | Medium — parse/edit sshd_config |
| 5 | `6.1.1` | Maintenance | Ensure permissions on `/etc/passwd` | Low — stat + chmod |

These 5 cover: filesystem, network, logging, SSH/auth, and file maintenance — validating the trait works across all major categories.

### 4. `vallumix-reporters` — JSON Format

Per PRD section 5.5, the JSON report must include:
- Host identification (hostname, distro, kernel, timestamp, duration)
- Executive summary (compliance rate, pass/fail/skip counts)
- Per-control detail (ID, description, severity, status, evidence, justification)
- Cross-standard mapping (NIST 800-53, ISO 27001, PCI-DSS) — can be stubbed for Phase 1

### 5. Dependencies to Add

| Crate | Dependencies | Purpose |
|-------|-------------|---------|
| Workspace | `anyhow`, `nix`, `walkdir`, `tempfile`, `rayon`, `owo-colors`, `indicatif` | PRD section 7.1 |
| `vallumix-core` | `serde` (already), `thiserror` (already), `tracing` (already) | — |
| `vallumix-cli` | `clap` (already), `anyhow`, `tracing-subscriber`, `owo-colors`, `indicatif` | CLI UX |
| `vallumix-controls` | `nix`, `walkdir`, `tempfile` | System operations |
| `vallumix-reporters` | `serde_json` (workspace) | JSON serialization |

**Deferred to later phases**: `askama` (HTML), `quick-xml` (JUnit), `rayon` (parallelism), `miette` (rich errors), `insta` (snapshots), `assert_cmd` (integration tests).

## Risks

1. **MSRV 1.75 constraint**: Some newer crate versions may require Rust 1.76+. Need to verify all dependency versions are compatible with 1.75.
2. **`nix` crate API changes**: The `nix` crate has breaking changes between versions. Must pin a version compatible with MSRV 1.75.
3. **Root privilege requirement**: Controls that modify system files need root. Testing requires either mock contexts or running in containers/VMs.
4. **Idempotency complexity**: `apply()` must be idempotent — running twice should not change the system. This requires careful design in each control.
5. **Profile TOML loading**: Need to decide where profile files live (hardcoded paths vs. configurable). PRD implies `/etc/vallumix/profiles/` or bundled defaults.

## Ready for Proposal

**Yes** — the current state is well-understood, the PRD provides clear requirements, and the architecture is already defined. The exploration identifies:
- Exactly what needs to change in each crate
- The 5 pilot controls to implement
- The dependency additions needed
- The JSON report structure
- Key risks (MSRV, nix crate, root privileges, idempotency)

The orchestrator should proceed to **sdd-propose** to create a formal change proposal, then **sdd-spec** for detailed requirements.
