# Design: Phase 1 — Foundations (Cimientos)

## Technical Approach

Incremental bottom-up implementation matching the proposal: complete `vallumix-core` types first, then `vallumix-reporters` (JSON), then `vallumix-controls` (5 pilots), then `vallumix-cli` (clap wiring), and finally the web profile. Each crate compiles before the next depends on it. The existing trait signatures from Phase 0 remain stable — we fill in the stubs with real fields and logic.

## Architecture Decisions

### AD-1: CheckResult and ApplyResult — Structs with Inner Status Enum

| Option | Tradeoff | Decision |
|--------|----------|----------|
| Flat enums only | Simpler, but no room for evidence/message | ✗ |
| Structs with inner enum | Rich data, type-safe status, idiomatic Rust | ✅ Chosen |
| Enums carrying data | Rust allows, but inconsistent ergonomics | ✗ |

```rust
pub enum CheckStatus { Compliant, NonCompliant, Skipped, Error }
pub struct CheckResult { pub status: CheckStatus, pub evidence: String, pub message: String }

pub enum ApplyStatus { Applied, Skipped, Failed }
pub struct ApplyResult { pub status: ApplyStatus, pub backup_path: Option<PathBuf>, pub message: String }
```

### AD-2: Distro Detection — Own Parser

| Option | Tradeoff | Decision |
|--------|----------|----------|
| `os-release` crate | Less code, but adds dep & MSRV risk | ✗ |
| Own `/etc/os-release` parser | Zero new dep, simple KEY=VALUE parse, MSRV-safe | ✅ Chosen |

Parse `/etc/os-release` line-by-line, extract `ID` and `VERSION_ID`, map `(ID, VERSION_ID)` to `Distro` enum. Return `CoreError::UnsupportedDistro` for unrecognized combos.

### AD-3: Context Struct — Eager Construction

| Option | Tradeoff | Decision |
|--------|----------|----------|
| Builder pattern | Flexible, but over-engineered for fixed fields | ✗ |
| Eager `Context::new()`` | Simple, deterministic, testable | ✅ Chosen |

Fields: `hostname: String`, `distro: Distro`, `work_dir: PathBuf`, `backup_dir: PathBuf`, `profile_dir: PathBuf`, `dry_run: bool`, `verbose: bool`. Constructor detects hostname via `nix::unistd::gethostname()`, calls `distro::detect()`, creates backup dir at `/var/backups/vallumix/<timestamp>/`.

### AD-4: Error Hierarchy — Two-Level

| Option | Tradeoff | Decision |
|--------|----------|----------|
| Single error enum | Simpler, but conflates core vs control errors | ✗ |
| `CoreError` + `ControlError` | Clean separation, each crate owns its domain | ✅ Chosen |

Add `CoreError` enum in `vallumix-core::error` for: `UnsupportedDistro`, `DistroDetectionFailed`, `ContextInitFailed`, `ProfileLoadFailed`, `PrivilegeRequired`. Keep `ControlError` as-is. `ProfileError` stays in `profile.rs` as a variant of `CoreError`, not a separate type.

### AD-5: Profile — Concrete Struct, Not Trait

| Option | Tradeoff | Decision |
|--------|----------|----------|
| `trait Profile` | Extensible, but Phase 1 needs one model | ✗ |
| `struct Profile` with `Deserialize` | TOML-native, pragmatic, YAGNI | ✅ Chosen |

```rust
#[derive(Debug, Deserialize)]
pub struct Profile { pub name: String, pub description: String, pub controls: Vec<String> }
```

Replace `trait Profile` with this struct. Add `resolve_controls()` method that takes a registry and returns resolved references, validating all IDs exist and no duplicates.

### AD-6: CLI — Clap Derive with Command Modules

| Option | Tradeoff | Decision |
|--------|----------|----------|
| `clap` builder API | More control, verbose | ✗ |
| `clap` derive | Idiomatic, less boilerplate, PRD-specified | ✅ Chosen |

`Cli` struct with `#[derive(Parser)]`, `Commands` enum with `#[derive(Subcommand)]`. Global args on `Cli`: `--profile`, `--dry-run`, `--verbose`, `--quiet`, `--threshold`, `--no-color`, `--report`. Each variant dispatches to `commands/{apply,audit,rollback,list,completion}.rs`. Exit codes: 0=ok, 1=below threshold, 2=config error, 3=privilege error.

### AD-7: Control Modules — One Module per CIS Domain

5 pilot controls, each in its own module under `vallumix-controls/src/`:

| Module | Control | CIS ID |
|--------|---------|--------|
| `filesystem/disable_cramfs` | Disable cramfs | 1.1.1.1 |
| `network/sysctl_ip_forwarding` | Disable IP forwarding | 3.1.1 |
| `services/ensure_auditd` | Ensure auditd installed | 4.1.1.1 |
| `ssh/disable_root_login` | Disable SSH root login | 5.2.4 |
| `maintenance/ensure_perms_passwd` | Ensure /etc/passwd perms | 6.1.1 |

Each struct implements `Control`. Idempotency: `check()` first, skip `apply()` if `Compliant`.

### AD-8: Backup — Minimal in Phase 1

`BackupManager` in `vallumix-backup`: `create_backup(path) -> Result<Backup>` copies file to `/var/backups/vallumix/<ts>/<control_id>/`. `restore(backup) -> Result<()>` copies back. `Backup` struct: `id`, `timestamp`, `original_path`, `backup_path`. Full versioned rollback deferred to Phase 2.

### AD-9: JSON Reporter — Report Struct with Serde

`Report` struct (serializable): `host_info`, `summary` (total/passed/failed/skipped/compliance_rate), `controls: Vec<ControlReport>`. `ControlReport`: id, description, severity, status, evidence, message. `JsonReporter` implements `Reporter` trait. Output to stdout or file via `--output`.

### AD-10: Phase 1 Dependencies — Lean

| Dep | Crate | Purpose |
|-----|-------|---------|
| `nix` | `vallumix-controls`, `vallumix-cli` | Privilege check, hostname |
| `walkdir` | `vallumix-controls` | File traversal |
| `owo-colors` | `vallumix-cli` | Terminal colors |
| `tempfile` | dev-dep, workspace | Test fixtures |
| `anyhow` | `vallumix-cli` | Top-level error handling |

**Deferred**: `rayon`, `indicatif`, `miette`, `askama`, `quick-xml`, `insta`, `assert_cmd`.

## Data Flow

```
CLI (clap) ─parse─→ Context::new() ─detect─→ Distro + Hostname
     │                      │
     │              Profile::load(web.toml)
     │                      │
     │              resolve_controls(registry)
     │                      │
     ▼                      ▼
  ┌────────── per-control loop ──────────────┐
  │  control.check(ctx) → CheckResult         │
  │  if NonCompliant && !dry_run:             │
  │    backup_mgr.create_backup()              │
  │    control.apply(ctx) → ApplyResult       │
  │    control.check(ctx) → post-check        │
  │  collect ControlReport                    │
  └───────────────────────────────────────────┘
                     │
                     ▼
         JsonReporter::generate(Report) → stdout/file
                     │
                     ▼
           Exit code (0/1/2/3)
```

## File Changes

| File | Action | Description |
|------|--------|-------------|
| `crates/vallumix-core/src/control.rs` | Modify | Add `CheckStatus`, `ApplyStatus` enums; expand `CheckResult`/`ApplyResult` structs |
| `crates/vallumix-core/src/context.rs` | Modify | Full `Context` struct with constructor |
| `crates/vallumix-core/src/distro.rs` | Modify | Real `detect()` parsing `/etc/os-release` |
| `crates/vallumix-core/src/error.rs` | Modify | Add `CoreError` enum |
| `crates/vallumix-core/src/profile.rs` | Modify | Replace traits with `Profile` struct + serde; `Backup` struct; `Reporter` trait with `generate()` |
| `crates/vallumix-core/src/lib.rs` | Modify | Re-export new types, tighten lints |
| `crates/vallumix-core/Cargo.toml` | Modify | Add `serde` derive feature |
| `crates/vallumix-controls/src/lib.rs` | Modify | Registry fn + module declarations |
| `crates/vallumix-controls/src/filesystem/` | Create | `mod.rs` + `disable_cramfs.rs` |
| `crates/vallumix-controls/src/network/` | Create | `mod.rs` + `sysctl_ip_forwarding.rs` |
| `crates/vallumix-controls/src/services/` | Create | `mod.rs` + `ensure_auditd.rs` |
| `crates/vallumix-controls/src/ssh/` | Create | `mod.rs` + `disable_root_login.rs` |
| `crates/vallumix-controls/src/maintenance/` | Create | `mod.rs` + `ensure_perms_passwd.rs` |
| `crates/vallumix-controls/Cargo.toml` | Modify | Add `nix`, `walkdir` deps |
| `crates/vallumix-reporters/src/lib.rs` | Modify | `JsonReporter` impl + `Report`/`ControlReport` structs |
| `crates/vallumix-reporters/src/json.rs` | Create | JSON serialization logic |
| `crates/vallumix-reporters/Cargo.toml` | Modify | Add `serde_json` dep |
| `crates/vallumix-backup/src/lib.rs` | Modify | `BackupManager` + `Backup` struct |
| `crates/vallumix-cli/src/main.rs` | Modify | Clap derive + tracing init + command dispatch |
| `crates/vallumix-cli/src/commands/` | Create | `apply.rs`, `audit.rs`, `rollback.rs`, `list.rs`, `completion.rs` |
| `crates/vallumix-cli/src/ui.rs` | Create | `owo-colors` output formatting |
| `crates/vallumix-cli/Cargo.toml` | Modify | Add `anyhow`, `owo-colors`, `tracing-subscriber` |
| `profiles/web.toml` | Modify | Add 5 pilot control IDs |
| `Cargo.toml` (workspace) | Modify | Add `nix`, `walkdir`, `owo-colors`, `anyhow`, `tempfile` to workspace deps |

## Interfaces / Contracts

### Reporter Trait (replaces empty trait in profile.rs)

```rust
pub trait Reporter: Send + Sync {
    fn generate(&self, report: &Report, output: &mut dyn std::io::Write) -> Result<(), CoreError>;
}
```

### Control Registry (new in vallumix-controls)

```rust
pub fn registry() -> HashMap<String, Box<dyn Control>> {
    let mut m = HashMap::new();
    m.insert("1.1.1.1".into(), Box::new(disable_cramfs::DisableCramfs));
    m.insert("3.1.1".into(), Box::new(sysctl_ip_forwarding::SysctlIpForwarding));
    m.insert("4.1.1.1".into(), Box::new(ensure_auditd::EnsureAuditd));
    m.insert("5.2.4".into(), Box::new(disable_root_login::SshDisableRootLogin));
    m.insert("6.1.1".into(), Box::new(ensure_perms_passwd::EnsurePermsPasswd));
    m
}
```

## Testing Strategy

| Layer | What to Test | Approach |
|-------|-------------|----------|
| Unit | `CheckStatus`/`ApplyStatus` enum variants, `Distro` mapping | Direct enum construction |
| Unit | `/etc/os-release` parsing | Fixture files in `tests/fixtures/os-release/` |
| Unit | `Profile` TOML deserialization | Load `web.toml` with real IDs, validate |
| Unit | `Context::new()` | Mock hostname/distro via dependency injection |
| Unit | Each control's `check()` logic | Mock `Context`, test fixture files |
| Integration | `cargo build` workspace | Full workspace compiles clean |
| Integration | CLI parsing | `clap` test harness for subcommands |
| CI | Cross-distro | Container matrix (Debian 12, Ubuntu 22/24, Rocky 9) |

## Migration / Rollback

No migration required — this is greenfield filling Phase 0 stubs. Rollback = `git checkout` per crate. Workspace dependency additions are additive.

## Open Questions

- [ ] Pin `nix` crate version — 0.27.x has MSRV 1.75 compatibility; verify before adding
- [ ] Decide profile search path: `/etc/vallumix/profiles/` vs bundled defaults vs configurable `--profile-dir`
- [ ] `indicatif` progress bars deferred to Phase 2 — acceptable for MVP?