# Design: Phase 2 Coverage — 60+ Controls, Reporters, Backup/Rollback

## Technical Approach

Incremental delivery in 5 sub-phases (2A–2E per proposal), each producing a compilable increment. Infrastructure first (reporters, backup expansion, CLI wiring, `Category` trait addition), then controls by CIS domain. New controls follow the established pattern: `new()` + `with_paths()` for testability, `check()` reads-only, `apply()` respects `dry_run`, `rollback()` restores from backup. The `Control` trait gains a `category()` method with a default implementation returning `Category::Uncategorized` to minimize breakage on existing pilot controls.

## Architecture Decisions

| Decision | Choice | Rejected | Rationale |
|----------|--------|----------|-----------|
| Control trait evolution | Add `fn category(&self) -> Category` with default impl | Macro-based registration, build script | Default impl means no breakage on 5 existing controls; manual registry keeps compile times fast and debuggable at 60 controls |
| Parallel audit | `rayon::par_iter()` on resolved control list in `audit` command only | Tokio async, sequential for all | Read-only checks are safe to parallelize; apply MUST stay sequential due to filesystem mutations |
| HTML reporter | askama with embedded CSS, template at `templates/report.html` | Tera, tera + external CSS | askama compiles templates at build time, zero runtime deps, self-contained output |
| JUnit XML | quick-xml with manual element construction | serde-xml-rs, quick-xml with derive | JUnit schema is tiny and fixed; manual construction avoids serde-xml pitfalls and gives full control over element ordering |
| Text reporter | owo-colors with conditional color via `set_override` | termcolor, colored | Already declared in workspace; `OwoColorize` trait matches existing CLI usage in `main.rs` |
| Backup sessions | Versioned directory structure: `/var/backups/vallumix/<session>/<control_id>/v<N>/` with checksum sidecar | SQLite catalog, single flat dir | Pure filesystem, no DB dependency, easy manual inspection and cleanup; checksum stored as `<file>.sha256` |
| Profile population | 25 controls each for database/bastion; expand web to 25 | Arbitrary per-benchmark coverage | Equal coverage per profile ensures balanced testing; specific control IDs mapped from CIS Benchmark Level 1 |
| Error handling in CLI | `miette` for user-facing errors wrapped around `VallumixError` | Keep `anyhow`, add `miette` only in main | `miette` gives rich diagnostics with source snippets; wrap at CLI boundary only |

## Data Flow

```
CLI parse (clap)
  │
  ├─► detect distro (Context::new)
  ├─► load profile (Profile::from_file)
  ├─► resolve controls (registry → Vec<Box<dyn Control>>)
  │
  ├─► [audit]: rayon::par_iter → check() → collect ControlReports
  │                                    │
  │                                    └─► build_report → reporter.generate
  │
  └─► [apply]: sequential for each control:
        check() → if NonCompliant →
          BackupManager.create_backup() →
          apply() →
          check() (post-verify) →
          collect result
                    │
                    └─► build_report → reporter.generate

BackupManager
  create_backup(id, path) → /var/backups/vallumix/<session>/<id>/v1/<file>
  list(session) → Vec<BackupMeta>
  restore(backup) → copy backup_path → original_path
  prune(session, keep: usize) → remove oldest versions
```

## File Changes

| File | Action | Description |
|------|--------|-------------|
| `crates/vallumix-core/src/control.rs` | Modify | Add `Category` enum + `fn category(&self) -> Category` with default `Uncategorized` |
| `crates/vallumix-core/src/lib.rs` | Modify | Export `Category` |
| `crates/vallumix-core/src/context.rs` | Modify | Add `session_id: String` field |
| `crates/vallumix-core/src/profile.rs` | Modify | Implement real `is_applicable()` with distro filtering |
| `crates/vallumix-reporters/src/lib.rs` | Modify | Refactor as module; move `build_report` + `JsonReporter` to `json.rs` |
| `crates/vallumix-reporters/src/html.rs` | Create | Askama-based HTML reporter |
| `crates/vallumix-reporters/src/junit.rs` | Create | quick-xml JUnit XML reporter |
| `crates/vallumix-reporters/src/text.rs` | Create | owo-colors text reporter |
| `crates/vallumix-reporters/templates/report.html` | Create | Askama template with embedded CSS |
| `crates/vallumix-backup/src/lib.rs` | Modify | Add `BackupSession`, `list()`, `prune()`, session-based paths, checksum |
| `crates/vallumix-cli/src/commands/apply.rs` | Modify | Full orchestration: load profile, resolve, backup, apply per control, report |
| `crates/vallumix-cli/src/commands/audit.rs` | Modify | Full orchestration with `rayon::par_iter` for check phase |
| `crates/vallumix-cli/src/commands/rollback.rs` | Modify | Accept `--session` and `--control-id`; use `BackupManager` |
| `crates/vallumix-cli/src/commands/completion.rs` | Modify | Wire `clap_complete` shell generation |
| `crates/vallumix-controls/src/auth/mod.rs` | Create | Auth/PAM controls module (14 controls) |
| `crates/vallumix-controls/src/logging/mod.rs` | Create | Logging controls module (10 controls) |
| `crates/vallumix-controls/src/lib.rs` | Modify | Register all 60+ controls; add `auth` and `logging` module declarations |
| `profiles/database.toml` | Modify | Populate with ~25 control IDs |
| `profiles/bastion.toml` | Modify | Populate with ~25 control IDs |
| `profiles/web.toml` | Modify | Update to include only implemented controls |
| `Cargo.toml` (workspace) | Modify | Add rayon, askama, quick-xml, indicatif, miette, sha2, clap_complete |

## Interfaces / Contracts

```rust
// vallumix-core/src/control.rs — NEW
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Filesystem, Services, Network, Logging,
    Ssh, Auth, Maintenance,
}

// Added to Control trait with default impl
pub trait Control: Send + Sync {
    // ... existing methods ...
    fn category(&self) -> Category { Category::Uncategorized } // wait — not in enum
}
// Better: default returns None, or we make all 7 categories + omit Uncategorized
// Decision: all variants explicit, existing pilot controls get category() overrides

// vallumix-backup — NEW
pub struct BackupSession {
    pub id: String,       // timestamp-based session ID
    pub backup_dir: PathBuf,
}

impl BackupManager {
    pub fn create_backup(&self, session_id: &str, control_id: &str, path: &Path) -> Result<Backup>;
    pub fn list(&self, session_id: &str) -> Result<Vec<BackupMeta>>;
    pub fn restore(&self, backup: &Backup) -> Result<()>;
    pub fn prune(&self, session_id: &str, keep: usize) -> Result<usize>;
    pub fn checksum(&self, path: &Path) -> Result<String>;  // SHA-256
}

// vallumix-reporters — Reporter trait already exists
impl Reporter for HtmlReporter { ... }
impl Reporter for JunitReporter { ... }
impl Reporter for TextReporter { ... }
```

## Testing Strategy

| Layer | What | Approach |
|-------|------|----------|
| Unit | Each control's `check()`/`apply()`/`rollback()` | `with_paths()` pattern pointing to `tempfile::tempdir()` fixtures; 2+ tests per control (compliant, non-compliant) |
| Unit | `BackupManager` create/list/restore/prune | Temp dir, verify file round-trip and checksum |
| Unit | All 4 reporters | `insta` snapshots for HTML, JUnit XML, JSON, text output |
| Unit | `Category` enum and trait default | Verify pilot controls gain category, new controls have explicit category |
| Integration | CLI `apply`, `audit`, `rollback` end-to-end | `assert_cmd` + `predicates`; test exit codes and output format |
| Integration | Profile resolution | Load real `.toml` files, resolve against registry, verify all IDs found |

## Migration / Rollout

No migration required. All changes are additive: new files, new module declarations, default trait method (non-breaking), expanded profiles. Existing 71 tests continue passing. Sub-phases 2A–2E are self-contained and revertible via `git revert`.

## Open Questions

- [ ] Should `Category` include an `Uncategorized` variant or force every control to declare a category? (Leaning: no `Uncategorized`, default returns first variant)
- [ ] Exact control ID mapping for database and bastion profiles — needs CIS Benchmark cross-reference