# Exploration: Phase 0 Bootstrap — Vallumix

## Current State

**The repository is a completely greenfield project.** Only the following exist:

| Path | Description |
|------|-------------|
| `vallumix.prd` | Product Requirements Document (578 lines, comprehensive) |
| `.git/` | Git repo with 1 commit on `main` + `develop` branch (HEAD on `develop`) |
| `.atl/skill-registry.md` | Agent skill registry (25 user skills, no Rust-specific skills) |
| `openspec/config.yaml` | SDD configuration with stack context and rules |
| `openspec/specs/` | Empty — no specs written yet |
| `openspec/changes/archive/` | Empty — no prior changes |

**No Rust code, no Cargo workspace, no CI, no configuration files whatsoever.** No `.gitignore`, no `rust-toolchain.toml`, no `deny.toml`, no LICENSE files, no README, no GitHub templates.

The Rust toolchain (rustc/cargo) is **NOT installed** on this machine.

## PRD Phase 0 Requirements (from vallumix.prd §8)

Phase 0 "Bootstrap" is defined as:
- **Duration:** 1 week
- **Deliverables:** Workspace initialized, CI with `cargo build`/`test`/`clippy`/`fmt`, issue and PR templates, initial README
- **Milestone:** v0.0.1

Additional Phase 0 details scattered throughout the PRD:

### Workspace Structure (§7.2)
- 5 crates: `vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`, `vallumix-cli`
- Rust edition 2021, MSRV 1.75
- `rust-toolchain.toml` pinning toolchain
- `deny.toml` for cargo-deny

### CI (§7.1, §6.2)
- GitHub Actions with `cargo build`/`test`/`clippy`/`fmt`
- Multi-distro matrix (Debian 12, Ubuntu 22.04/24.04, RHEL 9)
- Multi-arch (x86_64 + aarch64) via `cross`
- `cargo-audit` and `cargo-deny` in CI
- `clippy -- -D warnings` (pedantic mode)

### Governance & Templates (§9, §12.3)
- LICENSE: MIT OR Apache-2.0 (dual)
- Issue and PR templates
- CODEOWNERS
- dependabot configuration
- CONTRIBUTING.md
- CHANGELOG.md (Keep a Changelog format)

### Tooling
- `cargo-audit`, `cargo-deny`, `cargo-tarpaulin`, `cross`, `cargo-deb`, `cargo-generate-rpm`

## Affected Areas (To-Be Created)

All files need to be created from scratch:

| Area | Files |
|------|-------|
| **Workspace root** | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, `.gitignore` |
| **Crates** | `crates/vallumix-core/`, `crates/vallumix-controls/`, `crates/vallumix-reporters/`, `crates/vallumix-backup/`, `crates/vallumix-cli/` |
| **CI** | `.github/workflows/ci.yml`, `.github/dependabot.yml`, `.github/CODEOWNERS` |
| **Templates** | `.github/ISSUE_TEMPLATE/bug_report.yml`, `.github/ISSUE_TEMPLATE/feature_request.yml`, `.github/PULL_REQUEST_TEMPLATE.md` |
| **Docs** | `README.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, `LICENSE-MIT`, `LICENSE-APACHE` |
| **Profiles** | `profiles/web.toml`, `profiles/database.toml`, `profiles/bastion.toml` (placeholder) |

## Approaches

### Approach 1: Monolithic Bootstrap (All-in-One Change)
Create everything in a single change: workspace, all 5 crates (with minimal `lib.rs`/`main.rs` stubs), CI, templates, docs, licenses.

- **Pros:** Single PR, everything compiles together, no inter-crate dependency ordering issues
- **Cons:** Large PR to review, harder to isolate failures, more complex to test
- **Effort:** Medium-High (many files, but all scaffolding)

### Approach 2: Phased Bootstrap (Infrastructure First, Then Crates)
Phase 0a: Workspace root, CI, templates, docs, licenses. Phase 0b: Create 5 crates with stubs.

- **Pros:** Smaller, more focused changes; CI validates before crates exist
- **Cons:** Two changes instead of one, CI will fail until crates exist
- **Effort:** Medium (split across two changes)

### Approach 3: Minimal Viable Bootstrap (Workspace + 1 Crate + CI)
Create workspace root + `vallumix-core` (the foundational crate) + CI pipeline. Other crates added in Phase 1.

- **Pros:** Smallest possible bootstrap, CI validates immediately, follows YAGNI
- **Cons:** PRD explicitly lists all 5 crates as Phase 0 deliverables; workspace members must all exist
- **Effort:** Low

## Recommendation

**Approach 1 (Monolithic Bootstrap)** — The PRD is explicit that Phase 0 includes the full workspace with all 5 crates. Since this is greenfield with no existing code, there's nothing to break. All crates will be stubs (empty `lib.rs` or minimal `main.rs`) that compile cleanly. The CI pipeline can validate the entire workspace in one pass.

The key insight: these are **stubs**, not implementations. Each crate needs only:
- A `Cargo.toml` with metadata and dependencies
- A `src/lib.rs` (or `src/main.rs` for `vallumix-cli`) with a module declaration
- No actual logic yet — that comes in Phase 1

## Risks

1. **No Rust toolchain installed** — Cannot locally verify `cargo build`/`test`/`clippy`/`fmt`. CI will be the only validation. Mitigation: carefully construct `Cargo.toml` files with correct workspace references.
2. **Cross-compilation complexity** — `cross` requires Docker and QEMU for aarch64. CI matrix may be slow. Mitigation: start with x86_64 only in initial CI, add aarch64 as a separate job or Phase 3 as PRD indicates.
3. **cargo-deny configuration** — `deny.toml` requires understanding of dependency policies. Mitigation: start with conservative defaults, tighten in later phases.
4. **MSRV 1.75 constraint** — Some newer crate versions may require Rust 1.76+. Mitigation: pin dependency versions compatible with 1.75, or bump MSRV if justified.
5. **Workspace dependency management** — 5 crates with inter-dependencies need careful `Cargo.toml` setup. Mitigation: use workspace-level `[workspace.dependencies]` for shared deps.

## Open Questions

1. **Should aarch64 CI be included in Phase 0 or deferred to Phase 3?** The PRD mentions multi-arch in CI but Phase 3 specifically lists "CI multi-arch (x86_64 + aarch64)" as a deliverable. Recommend: x86_64 only in Phase 0, aarch64 in Phase 3.
2. **Should `profiles/*.toml` be created in Phase 0?** They're part of the workspace structure but contain no logic yet. Recommend: create empty placeholder files.
3. **Should `docker/` files be created in Phase 0?** PRD shows them in structure but Phase 4 mentions Docker examples. Recommend: defer to Phase 4.

## Ready for Proposal

**Yes.** The scope is well-defined by the PRD. The recommendation is to create a single change that establishes the full workspace skeleton with all 5 crates as compilable stubs, CI pipeline (x86_64 only, multi-distro), governance files, and documentation scaffolding.

---

*skill_resolution: "none" (no Rust-specific skills in registry yet)*
