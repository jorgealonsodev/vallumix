# Tasks: Phase 1 — Foundations (Cimientos)

## Phase 1: Core Types (vallumix-core)

- [x] **T-001** `core-traits` — Add `CheckStatus` enum (`Compliant`, `NonCompliant`, `Error`, `Skipped`) and `ApplyStatus` enum (`Applied`, `AlreadyCompliant`, `Failed`, `Skipped`) to `crates/vallumix-core/src/control.rs`. Both derive `Debug, Clone, PartialEq, Eq`.
- [x] **T-002** `core-traits` — Expand `CheckResult` struct: add `status: CheckStatus`, `evidence: String`, `message: Option<String>`. Expand `ApplyResult` struct: add `status: ApplyStatus`, `backup_path: Option<PathBuf>`, `message: Option<String>`. Both derive `Debug, Clone`.
- [x] **T-003** `core-traits` — Add `Reporter::generate(&self, results: &[ControlResult], ctx: &Context) -> Result<String, ReportError>` method. Define `ReportError` enum with `thiserror`. Add `ControlResult` type alias `Result<CheckResult, ControlError>`.
- [x] **T-004** `error-types` — Add `VallumixError` enum to `crates/vallumix-core/src/error.rs` with variants: `UnsupportedDistro(String)`, `Privilege(String)`, `ProfileNotFound(PathBuf)`, `ReportGeneration(String)`, `Io(#[from] std::io::Error)`. Derive `thiserror::Error, Debug`.
- [x] **T-005** `error-types` — Add `From<ControlError>` impl for `VallumixError` in `error.rs`.
- [x] **T-006** `distro-detection` — Implement `detect_from_path(path: impl AsRef<Path>) -> Result<Distro, VallumixError>` in `crates/vallumix-core/src/distro.rs`. Parse `/etc/os-release` KEY=VALUE lines, handle quotes, case-insensitive keys. Map `debian/12`→`Debian12`, `ubuntu/22.04`→`Ubuntu2204`, `ubuntu/24.04`→`Ubuntu2404`, `rocky/9.x`/`almalinux/9.x`/`rhel/9.x`→`Rocky9`. Return `UnsupportedDistro` for unrecognized.
- [x] **T-007** `distro-detection` — Expose `detect() -> Result<Distro, VallumixError>` as public API that calls `detect_from_path("/etc/os-release")`.
- [x] **T-008** `execution-context` — Implement full `Context` struct in `crates/vallumix-core/src/context.rs` with fields: `hostname: String`, `distro: Distro`, `work_dir: PathBuf`, `backup_dir: PathBuf`, `profile_dir: PathBuf`, `dry_run: bool`. Derive `Debug, Clone`.
- [x] **T-009** `execution-context` — Implement `Context::new(distro: Distro) -> Result<Context, VallumixError>` using `nix::unistd::gethostname()` for hostname, defaults `/var/lib/vallumix`, `/var/backups/vallumix`, `/etc/vallumix/profiles`. Fall back to `"localhost"` on error with `tracing::warn!`.
- [x] **T-010** `execution-context` — Add env-var overrides for `VALLUMIX_WORK_DIR`, `VALLUMIX_BACKUP_DIR`, `VALLUMIX_PROFILE_DIR` in `Context::new()`.
- [x] **T-011** `execution-context` — Implement `Context::with_paths(hostname, distro, work_dir, backup_dir, profile_dir, dry_run)` constructor for testing.
- [x] **T-012** `profile-model` — Replace `trait Profile` with `struct Profile { name: String, description: String, controls: Vec<String> }` in `crates/vallumix-core/src/profile.rs`. Add `#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]`. Use `#[serde(default)]` on `controls`.
- [x] **T-013** `profile-model` — Add `Backup` struct to `profile.rs` with fields: `id: String`, `timestamp: DateTime<Utc>`, `original_path: PathBuf`, `backup_path: PathBuf`. Derive `Debug, Clone`.
- [x] **T-014** `profile-model` — Add `Profile::from_file(path: impl AsRef<Path>) -> Result<Profile, VallumixError>` using `std::fs::read_to_string` and `toml::from_str`. Return `VallumixError::ProfileNotFound` on `io::ErrorKind::NotFound`.
- [x] **T-015** `profile-model` — Add `Profile::resolve_controls(&self, registry: &ControlRegistry) -> Result<Vec<Box<dyn Control>>, VallumixError>` that maps control ID strings to `Box<dyn Control>`. Return error for missing IDs.
- [x] **T-016** `profile-model` — Add `Profile::is_applicable(&self, distro: &Distro) -> bool` — returns true if all resolved controls are applicable, or if controls list is empty.
- [x] **T-017** `core-traits` — Update `crates/vallumix-core/src/lib.rs` to re-export `CheckStatus`, `ApplyStatus`, `CheckResult`, `ApplyResult` at crate root. Add `ControlRegistry` type alias `HashMap<String, Box<dyn Control>>`.
- [x] **T-018** `core-traits` — Verify all existing tests in `lib.rs` still pass (`cargo test -p vallumix-core`).

## Phase 2: Backup & Controls Foundation

- [x] **T-019** `pilot-controls` — Add `nix` crate (0.27) and `walkdir` to `crates/vallumix-controls/Cargo.toml`. Add `anyhow` to workspace deps.
- [x] **T-020** `pilot-controls` — Create `crates/vallumix-controls/src/filesystem/mod.rs` and `crates/vallumix-controls/src/filesystem/disable_cramfs.rs` implementing `Control` trait. `id()` → `"1.1.1.1"`, `severity()` → `Severity::Low`, `applicable_distros()` → all 4 distros. `check()` reads `/proc/filesystems` for `cramfs`. `apply()` writes `install cramfs /bin/true` to `/etc/modprobe.d/`.
- [x] **T-021** `pilot-controls` — Create `crates/vallumix-controls/src/network/mod.rs` and `crates/vallumix-controls/src/network/sysctl_ip_forwarding.rs`. `id()` → `"3.1.1"`, `severity()` → `Severity::Medium`. `check()` reads `/proc/sys/net/ipv4/ip_forward` value `0`. `apply()` writes drop-in to `/etc/sysctl.d/` and calls `sysctl -w`.
- [x] **T-022** `pilot-controls` — Create `crates/vallumix-controls/src/services/mod.rs` and `crates/vallumix-controls/src/services/ensure_auditd.rs`. `id()` → `"4.1.1.1"`, `severity()` → `Severity::Medium`. `check()` verifies `auditd` package installed via distro package manager. `apply()` installs package.
- [x] **T-023** `pilot-controls` — Create `crates/vallumix-controls/src/ssh/mod.rs` and `crates/vallumix-controls/src/ssh/disable_root_login.rs`. `id()` → `"5.2.4"`, `severity()` → `Severity::High`. `check()` parses `/etc/ssh/sshd_config` for uncommented `PermitRootLogin no`. `apply()` adds/sets directive.
- [x] **T-024** `pilot-controls` — Create `crates/vallumix-controls/src/maintenance/mod.rs` and `crates/vallumix-controls/src/maintenance/ensure_perms_passwd.rs`. `id()` → `"6.1.1"`, `severity()` → `Severity::High`. `check()` verifies `/etc/passwd` mode `0644`. `apply()` sets mode.
- [x] **T-025** `pilot-controls` — Implement `rollback()` on all 5 controls — restore original file content or permissions from `Backup`.
- [x] **T-026** `pilot-controls` — Add `registry() -> ControlRegistry` function to `crates/vallumix-controls/src/lib.rs` registering all 5 pilot controls.
- [x] **T-027** `pilot-controls` — Write unit tests for each control's `check()` logic using fixture files in `crates/vallumix-controls/tests/fixtures/`.

## Phase 3: Reporters & Profile Data

- [x] **T-028** `json-reporter` — Add `serde_json` to `crates/vallumix-reporters/Cargo.toml`. Add `Report` struct with `host`, `summary`, `controls` fields. Add `ControlReport` struct with `id`, `description`, `severity`, `status`, `evidence`, `message`.
- [x] **T-029** `json-reporter` — Implement `JsonReporter` in `crates/vallumix-reporters/src/lib.rs` implementing `Reporter` trait. `generate()` produces pretty-printed JSON via `serde_json::to_string_pretty`. Field names use `snake_case`.
- [x] **T-030** `json-reporter` — Write tests for `JsonReporter` — verify output is valid JSON, correct field names, compliance rate calculation.
- [x] **T-031** `web-profile` — Populate `profiles/web.toml` with ~20 control IDs spanning CIS categories 1.x through 6.x. Must include pilot controls: `"1.1.1.1"`, `"3.1.1"`, `"5.2.4"`, `"6.1.1"`. Include at least one from category 2.x and 4.x.
- [x] **T-032** `web-profile` — Verify `web.toml` parses via `Profile::from_file()` and `profile.controls` is non-empty.

## Phase 4: CLI Wiring

- [x] **T-033** `cli-structure` — Add `clap`, `anyhow`, `owo-colors`, `tracing-subscriber` deps to `crates/vallumix-cli/Cargo.toml`. Add `nix` to workspace deps.
- [x] **T-034** `cli-structure` — Define `Cli` struct with `#[derive(Parser)]` in `crates/vallumix-cli/src/main.rs`. Global args: `--profile` (default "web"), `--dry-run`, `--verbose`/`-v`, `--quiet`/`-q` (mutually exclusive with verbose), `--threshold` (default 80, 0-100), `--no-color`, `--report` (json).
- [x] **T-035** `cli-structure` — Define `Commands` enum with `#[derive(Subcommand)]` variants: `apply`, `audit`, `rollback` (optional `--control-id`), `list`, `completion` (shell arg). Per-command options per PRD §5.4.
- [x] **T-036** `cli-structure` — Initialize `tracing-subscriber` in `main()` based on `--verbose` (DEBUG), `--quiet` (WARN), default (INFO). JSON output mode uses `fmt::json()`.
- [x] **T-037** `cli-structure` — Implement command dispatch: `apply` → `commands/apply.rs`, `audit` → `commands/audit.rs`, etc. Create `crates/vallumix-cli/src/commands/` directory with module files.
- [x] **T-038** `cli-structure` — Implement exit codes: 0=ok, 1=below threshold, 2=config error, 3=privilege error. Use `std::process::exit(code)`.
- [x] **T-039** `cli-structure` — Add root privilege check (via `nix::unistd::geteuid() == 0`) — exit code 3 if not root for `apply` subcommand.
- [x] **T-040** `cli-structure` — Write clap test harness: verify subcommands parse, flags work, mutual exclusivity enforced, defaults correct.

## Phase 5: Integration & Verification

- [x] **T-041** — Run `cargo build` on entire workspace — zero warnings across all 5 crates.
- [x] **T-042** — Run `cargo test` — all existing + new tests pass.
- [x] **T-043** — Run `cargo clippy -- -D warnings` — clean.
- [x] **T-044** — Verify `Profile::resolve_controls()` from `web.toml` with pilot registry returns 5 controls without error.
- [x] **T-045** — Verify exit codes 0/1/2/3 for audit command under different scenarios.

## Dependencies Summary

```
T-001 → T-002 → T-003 → T-004 → T-005 → T-006 → T-007
                                          ↓
T-008 → T-009 → T-010 → T-011 ← T-006
         ↓
T-012 → T-013 → T-014 → T-015 → T-016
         ↓
T-017 → T-018

T-019 → T-020
         ↓
T-021
         ↓
T-022
         ↓
T-023
         ↓
T-024 → T-025 → T-026 → T-027

T-028 → T-029 → T-030

T-031 → T-032

T-033 → T-034 → T-035 → T-036 → T-037 → T-038 → T-039 → T-040

T-041 → T-042 → T-043 → T-044 → T-045
```

## Implementation Order (per Design)

1. **Phase 1** — Core types first: T-001 through T-018. Order: control/status enums → error types → distro detection → context → profile struct.
2. **Phase 2** — Controls: T-019 through T-027. Each control module independent, can parallelize after T-019.
3. **Phase 3** — Reporters + profile data: T-028 through T-032. Profile data is last.
4. **Phase 4** — CLI: T-033 through T-040. Depends on all above.
5. **Phase 5** — Verification: T-041 through T-045.

## Test Expectations (TDD RED→GREEN→REFACTOR per task)

| Task | Test to Write (RED) | What to Implement (GREEN) |
|------|---------------------|---------------------------|
| T-001 | Construct `CheckStatus::NonCompliant` — test all 4 variants compile | Define enum variants |
| T-002 | `CheckResult::non_compliant("module loaded", None)` — test fields | Expand struct fields |
| T-006 | Load fixture `/etc/os-release` with `debian/12` → expect `Debian12` | Parse KEY=VALUE, map to enum |
| T-007 | `detect()` calls internal path default | Wrapper that calls `detect_from_path` |
| T-008 | Construct `Context` with all fields accessible | Full struct with fields |
| T-009 | `Context::new()` reads env vars, sets defaults | Constructor with hostname, paths |
| T-012 | `toml::from_str::<Profile>(...)` with empty controls → ok | Profile struct + serde derives |
| T-014 | `Profile::from_file()` with missing file → `VallumixError::ProfileNotFound` | `from_file` implementation |
| T-015 | `resolve_controls()` with unknown ID → error | Resolve IDs against registry |
| T-020 | `disable_cramfs.check()` when `cramfs` absent → `Compliant` | Read `/proc/filesystems`, match |
| T-028 | `serde_json::to_string_pretty(report)` valid JSON output | Report structs + derive Serialize |
| T-031 | `web.toml` has ≥20 controls, spans ≥5 CIS categories | TOML data with control IDs |