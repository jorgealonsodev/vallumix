# Design: Phase 0 — Bootstrap

## Technical Approach

Monolithic bootstrap per exploration recommendation: create all 5 crates as compilable stubs, CI pipeline, governance scaffolding, and core trait definitions in a single change. The workspace root defines shared dependency versions via `[workspace.dependencies]`. Each crate gets a minimal `Cargo.toml` and a `src/lib.rs` (or `src/main.rs` for CLI) that compiles cleanly. The `vallumix-core` crate hosts the architectural foundation: the `Control`, `Reporter`, and `Profile` traits with their associated types. CI runs on 4 distros via container matrix, validates build/test/clippy/fmt, and is the sole build validation (no local Rust toolchain).

## Architecture Decisions

### AD-1: Workspace Layout

| Option | Tradeoff | Decision |
|--------|----------|----------|
| Flat workspace (crates at root) | Simpler paths, but pollutes root | ✗ |
| `crates/` subdirectory | Standard Rust convention, clean root | ✅ Chosen |

Root `Cargo.toml` is virtual (`[workspace]` only, no `[package]`). Members listed explicitly. `[workspace.dependencies]` centralizes shared dep versions.

### AD-2: Crate Dependency Graph

| Crate | Depends On |
|-------|-----------|
| `vallumix-core` | — (no vallumix deps) |
| `vallumix-controls` | `vallumix-core` |
| `vallumix-reporters` | `vallumix-core` |
| `vallumix-backup` | `vallumix-core` |
| `vallumix-cli` | all other vallumix crates |

Rationale: core defines traits; other crates implement them. CLI orchestrates across all crates. Phase 0 adds only `vallumix-core` as a dependency — actual trait impls come in Phase 1.

### AD-3: Core Trait Signatures

Signatures match PRD §5.2 exactly. The `Control` trait is the central abstraction:

```rust
pub trait Control: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn severity(&self) -> Severity;
    fn applicable_distros(&self) -> &[Distro];
    fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError>;
    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError>;
    fn rollback(&self, ctx: &Context, backup: &Backup) -> Result<(), ControlError>;
}
```

Associated types are stubs (unit structs or empty enums) in Phase 0 — full implementation in Phase 1.

### AD-4: CI Matrix Strategy

| Option | Tradeoff | Decision |
|--------|----------|----------|
| Native runners per distro | No Docker needed, but limited to ubuntu-* | ✗ |
| Container matrix per distro | Tests actual target environments; Docker overhead | ✅ Chosen |

Matrix: `debian:12`, `ubuntu:22.04`, `ubuntu:24.04`, `rockylinux:9`. All x86_64. aarch64 deferred to Phase 3 per PRD §8. `fail-fast: false` ensures all distro results are visible.

### AD-5: Dependency Versions

| Dependency | Version | Notes |
|------------|---------|-------|
| `clap` | 4.x with `derive` feature | CLI argument parsing |
| `serde` + `serde_json` | 1.x | Serialization (profile loading) |
| `thiserror` | 2.x | Error types (MSRV 1.61+, compatible with 1.75) |
| `tracing` + `tracing-subscriber` | 0.1.x | Structured logging |

All pinned in `[workspace.dependencies]` with `version = "X"` and referenced by workspace crates as `dep.workspace = true`.

### AD-6: Profile File Format

| Option | Tradeoff | Decision |
|--------|----------|----------|
| Empty files | Minimal, but unclear intent | ✗ |
| TOML with name + description + empty list | Self-documenting, parseable | ✅ Chosen |

Each profile TOML contains `name`, `description`, and empty `controls = []`.

## Data Flow

Phase 0 has no runtime data flow — all stubs compile but don't execute logic. The structural flow is:

```
Cargo.toml (workspace root)
    │
    ├── vallumix-core  ──→ trait definitions (Control, Reporter, Profile + types)
    │
    ├── vallumix-controls ──→ depends on core (stub)
    ├── vallumix-reporters ──→ depends on core (stub)
    ├── vallumix-backup ──→ depends on core (stub)
    │
    └── vallumix-cli ──→ depends on all crates (stub main.rs)
```

CI validates: `cargo build` → `cargo test` → `cargo clippy -- -D warnings` → `cargo fmt --check`

## File Changes

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` | Create | Virtual workspace root with members + `[workspace.dependencies]` |
| `rust-toolchain.toml` | Create | Pin channel = "1.75", targets = ["x86_64-unknown-linux-musl"] |
| `deny.toml` | Create | cargo-deny config: licenses (MIT, Apache-2.0), advisories, duplicates |
| `.gitignore` | Create | Rust + IDE + OS exclusions |
| `crates/vallumix-core/Cargo.toml` | Create | Core lib manifest with `thiserror`, `serde`, `tracing` deps |
| `crates/vallumix-core/src/lib.rs` | Create | Module declarations (control, context, distro, error, profile) |
| `crates/vallumix-core/src/control.rs` | Create | `Control` trait + `Severity`, `CheckResult`, `ApplyResult` stubs |
| `crates/vallumix-core/src/error.rs` | Create | `ControlError` enum via `thiserror` |
| `crates/vallumix-core/src/context.rs` | Create | `Context` struct stub |
| `crates/vallumix-core/src/distro.rs` | Create | `Distro` enum + `detect()` stub |
| `crates/vallumix-core/src/profile.rs` | Create | `Profile` trait + `Backup` struct stub |
| `crates/vallumix-controls/Cargo.toml` | Create | Controls lib manifest (depends on core) |
| `crates/vallumix-controls/src/lib.rs` | Create | Empty lib stub |
| `crates/vallumix-reporters/Cargo.toml` | Create | Reporters lib manifest (depends on core) |
| `crates/vallumix-reporters/src/lib.rs` | Create | `Reporter` trait stub |
| `crates/vallumix-backup/Cargo.toml` | Create | Backup lib manifest (depends on core) |
| `crates/vallumix-backup/src/lib.rs` | Create | Empty lib stub |
| `crates/vallumix-cli/Cargo.toml` | Create | Binary manifest with `clap` dep + all vallumix crates |
| `crates/vallumix-cli/src/main.rs` | Create | Minimal `fn main()` with clap stub |
| `.github/workflows/ci.yml` | Create | Multi-distro container matrix, build/test/clippy/fmt |
| `.github/dependabot.yml` | Create | Cargo dependency updates |
| `.github/CODEOWNERS` | Create | Default codeowners |
| `.github/ISSUE_TEMPLATE/bug_report.yml` | Create | Bug report template |
| `.github/ISSUE_TEMPLATE/feature_request.yml` | Create | Feature request template |
| `.github/PULL_REQUEST_TEMPLATE.md` | Create | PR template |
| `profiles/web.toml` | Create | Web server profile placeholder |
| `profiles/database.toml` | Create | Database server profile placeholder |
| `profiles/bastion.toml` | Create | Bastion host profile placeholder |
| `README.md` | Create | Project README with build/usage instructions |
| `LICENSE-MIT` | Create | MIT license text |
| `LICENSE-APACHE` | Create | Apache-2.0 license text |
| `CONTRIBUTING.md` | Create | Contribution guidelines |
| `CHANGELOG.md` | Create | Empty changelog per Keep a Changelog format |

## Interfaces / Contracts

### vallumix-core public API (stubs)

```rust
// control.rs
pub enum Severity { Low, Medium, High }
pub struct CheckResult { /* stub */ }
pub struct ApplyResult { /* stub */ }

pub trait Control: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    fn severity(&self) -> Severity;
    fn applicable_distros(&self) -> &[Distro];
    fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError>;
    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError>;
    fn rollback(&self, ctx: &Context, backup: &Backup) -> Result<(), ControlError>;
}

// error.rs
#[derive(thiserror::Error, Debug)]
pub enum ControlError {
    #[error("control {0} not applicable to distribution {1}")]
    NotApplicable(String, Distro),
    #[error("backup failed for {path}: {source}")]
    BackupFailed { path: std::path::PathBuf, #[source] source: std::io::Error },
    #[error("post-check failed: expected {expected}, got {actual}")]
    PostCheckFailed { expected: String, actual: String },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// distro.rs
pub enum Distro { Debian12, Ubuntu2204, Ubuntu2404, Rocky9 }

// profile.rs
pub struct Backup { /* stub */ }
pub trait Reporter: Send + Sync { /* stub */ }
pub trait Profile: Send + Sync { /* stub */ }
```

## Testing Strategy

| Layer | What to Test | Approach |
|-------|-------------|----------|
| Unit | Core types compile, enum variants exist | `cargo check` per crate — no logic yet |
| Unit | Control trait is object-safe | Compile test: `Box<dyn Control>` |
| Unit | Profile TOML parsing | Deserialize placeholder files with `serde` |
| Integration | Full workspace builds | `cargo build` from root |
| CI | Cross-distro compilation | Container matrix in GitHub Actions |

## Migration / Rollback

Greenfield — no existing code to migrate. Rollback = delete all created files and revert to initial commit.

## Open Questions

- [ ] Verify `thiserror` v2.x MSRV is ≤ 1.75 (preliminary check says yes: MSRV 1.61+)
- [ ] Decide if `vallumix-cli` should use `clap` derive in Phase 0 or defer to Phase 1 (recommend: minimal derive with single `--version` flag)
- [ ] Confirm `rockylinux:9` container image availability on GitHub Actions runners