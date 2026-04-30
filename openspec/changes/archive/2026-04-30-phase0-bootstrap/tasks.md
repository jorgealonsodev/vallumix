# Tasks: phase0-bootstrap

## Phase 1: Workspace Foundation

- [x] T-001: Create root `Cargo.toml` as virtual workspace with `[workspace.dependencies]` section pinning clap=4.x, serde=1.x, thiserror=2.x, tracing=0.1.x, and listing all 5 crate members (core, controls, reporters, backup, cli)
- [x] T-002: Create `rust-toolchain.toml` pinning channel="1.75" and `deny.toml` with license allowlist (MIT, Apache-2.0), advisory checks, duplicate detection; create `.gitignore` excluding target/, .vscode/, *.swp, .DS_Store
- [x] T-003: Create profile placeholders `profiles/web.toml`, `profiles/database.toml`, `profiles/bastion.toml` — each as valid TOML with `name`, `description`, and empty `controls = []`

## Phase 2: Crate Stubs

- [x] T-004: Create `crates/vallumix-core/Cargo.toml` with thiserror, serde, tracing deps; create `crates/vallumix-core/src/lib.rs` declaring public modules: `pub mod control; pub mod error; pub mod context; pub mod distro; pub mod profile;`
- [x] T-005: Create `crates/vallumix-controls/Cargo.toml` (depends on vallumix-core via workspace = true) and `crates/vallumix-controls/src/lib.rs` (empty lib)
- [x] T-006: Create `crates/vallumix-reporters/Cargo.toml` (depends on vallumix-core) and `crates/vallumix-reporters/src/lib.rs` (empty lib stub)
- [x] T-007: Create `crates/vallumix-backup/Cargo.toml` (depends on vallumix-core) and `crates/vallumix-backup/src/lib.rs` (empty lib)
- [x] T-008: Create `crates/vallumix-cli/Cargo.toml` with clap derive dep and all vallumix crates as dependencies; create `crates/vallumix-cli/src/main.rs` with `fn main()` that prints "vallumix-cli" and exits

## Phase 3: Core Trait Definitions (vallumix-core)

- [x] T-009: Define `crates/vallumix-core/src/control.rs`: `pub enum Severity { Low, Medium, High }`, `pub struct CheckResult;`, `pub struct ApplyResult;`, `pub trait Control: Send + Sync { fn id(&self) -> &str; fn description(&self) -> &str; fn severity(&self) -> Severity; fn applicable_distros(&self) -> &[Distro]; fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError>; fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError>; fn rollback(&self, ctx: &Context, backup: &Backup) -> Result<(), ControlError>; }`
- [x] T-010: Define `crates/vallumix-core/src/error.rs`: `#[derive(thiserror::Error, Debug)] pub enum ControlError` with variants: `NotApplicable(String, Distro)`, `BackupFailed { path: PathBuf, source: IoError }`, `PostCheckFailed { expected: String, actual: String }`, `Io(#[from] IoError)`
- [x] T-011: Define `crates/vallumix-core/src/distro.rs`: `pub enum Distro { Debian12, Ubuntu2204, Ubuntu2404, Rocky9 }` and `pub fn detect() -> Distro { Rocky9 }` stub
- [x] T-012: Define `crates/vallumix-core/src/context.rs`: `pub struct Context;` stub and `crates/vallumix-core/src/profile.rs`: `pub struct Backup;`, `pub trait Reporter: Send + Sync { }`, `pub trait Profile: Send + Sync { }`

## Phase 4: CI Pipeline (GitHub Actions)

- [x] T-013: Create `.github/workflows/ci.yml` with container matrix: debian:12, ubuntu:22.04, ubuntu:24.04, rockylinux:9 on x86_64; steps: rustup/toolchain install, cargo build, cargo test, cargo clippy -- -D warnings, cargo fmt --check; fail-fast: false; triggers on push to develop and PRs targeting develop

## Phase 5: Governance Scaffolding

- [x] T-014: Create `.github/CODEOWNERS` with default `*` rule; create `.github/dependabot.yml` with package-ecosystem=cargo, interval=weekly; create `.github/ISSUE_TEMPLATE/bug_report.yml` and `.github/ISSUE_TEMPLATE/feature_request.yml`
- [x] T-015: Create `.github/PULL_REQUEST_TEMPLATE.md` with sections for linked issue, summary, test plan; create `LICENSE-MIT` and `LICENSE-APACHE` with standard text
- [x] T-016: Create `CONTRIBUTING.md` with build (`cargo build`), test (`cargo test`), lint (`cargo clippy`), format (`cargo fmt`) instructions; create `CHANGELOG.md` following Keep a Changelog with Unreleased section
- [x] T-017: Create `README.md` with project name "vallumix", one-line description, build/test instructions, license badge (MIT OR Apache-2.0), link to CONTRIBUTING.md

## Phase 6: Verification

- [x] T-018: Run `cargo build` in workspace root — all 5 crates must compile; run `cargo check -p vallumix-core` — zero warnings; verify `Box<dyn Control>` compiles (object safety); verify all 5 crates pass `cargo clippy -- -D warnings` and `cargo fmt --check`
- [x] T-019: Validate profile TOMLs parse with `serde`: web.toml, database.toml, bastion.toml each contain valid TOML with name, description, controls fields

## Test Expectations

| Command | Expected |
|---------|----------|
| `cargo build` | All 5 crates compile, zero errors |
| `cargo test` | Zero test failures (stubs pass vacuously) |
| `cargo clippy -- -D warnings` | Zero warnings across all crates |
| `cargo fmt --check` | Zero formatting violations |

## Implementation Order

Phase 1 (T-001–T-003) must complete before Phase 2 — workspace root must exist before crate stubs can be added. Phase 3 (T-004) creates the core module scaffold before trait definitions in Phase 3 tasks. Phase 4 CI can be written independently but benefits from having compilable crates first. Phase 5 governance files are independent of compilation. Phase 6 verification is last.