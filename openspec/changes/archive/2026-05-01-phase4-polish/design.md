# Design: Phase 4 — Polish

## Technical Approach

Phase 4 delvers the polish layer for v0.9 RC: man page generation via `build.rs` + `clap_mangen`, Nushell completion fix using native `clap_complete::Shell::Nushell`, two independent mdBook sites (English + Spanish), `cargo-deb` / `cargo-generate-rpm` metadata in `Cargo.toml`, a Packer example, README overhaul, CHANGELOG, and a release CI pipeline with cross-compilation. All changes are additive and non-breaking.

## Architecture Decisions

| # | Decision | Choice | Rejected | Rationale |
|---|----------|-------|----------|-----------|
| D1 | Man page generation | `build.rs` with `clap_mangen` | Runtime subcommand; CI-only artifact | Standard Rust CLI pattern. Always in sync with CLI args. Ships with binary and packages. |
| D2 | Bilingual docs | Two separate mdBook trees (`docs/en/`, `docs/es/`) | Single book with i18n plugin; `mdbook-i18n-helper` | Simplest, each language evolves independently. No extra deps or POT/PO workflow. Deploy to `/en/` and `/es/` on GitHub Pages. |
| D3 | Package metadata | `[package.metadata.deb]` + `[package.metadata.generate-rpm]` in `vallumix-cli/Cargo.toml` | Separate packaging configs; `fpm` | Idiomatic Rust tooling. Tools (`cargo-deb`, `cargo-generate-rpm`) are cargo subcommands installed in CI — not library deps. |
| D4 | Nushell completion | Use `clap_complete::Shell::Nushell` directly | Keep custom `Shell` enum with stub | `clap_complete` 4.5 ships `Shell::Nushell`. The custom enum causes drift and the stub is outdated. Delete our enum; re-export `clap_complete::Shell`. |
| D5 | Release CI | GitHub Actions workflow with `cross` for x86_64 + aarch64 musl, `cargo-deb`, `cargo-generate-rpm`, `clap_mangen` | Manual release; single-arch | Automates v1.0 acceptance criteria: multi-arch binaries + `.deb` + `.rpm` on every tag. |
| D6 | CHANGELOG | Keep a Changelog format, seeded with v0.0.1 | No changelog; git log | PRD §6.4 mandates it. Required for release notes. |

## Data Flow

```
┌─ build.rs ──────────────────────────┐
│  clap_mangen → man/vallumix.1       │
└─────────────────────────────────────┘
          │
          ▼
┌─ Release CI (.github/workflows/release.yml) ─────────────────────┐
│  tag v* ──► cross build (x86_64-musl, aarch64-musl)             │
│           ├─► cargo deb (per arch)                                │
│           ├─► cargo generate-rpm (per arch)                      │
│           └─► GitHub Release (binaries + packages + man page)    │
└──────────────────────────────────────────────────────────────────┘

┌─ Docs CI (.github/workflows/docs.yml) ──┐
│  push to develop ──► mdbook build (en)   │
│                     mdbook build (es)     │
│                     ──► GitHub Pages      │
└──────────────────────────────────────────┘
```

## File Changes

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` (workspace) | Modify | Add `clap_mangen = "0.2"` to workspace deps |
| `crates/vallumix-cli/Cargo.toml` | Modify | Add `clap_mangen` dep, `[package.metadata.deb]`, `[package.metadata.generate-rpm]` sections |
| `crates/vallumix-cli/build.rs` | Create | Generate man page via `clap_mangen` at compile time |
| `crates/vallumix-cli/src/main.rs` | Modify | Remove custom `Shell` enum; re-export `clap_complete::Shell` |
| `crates/vallumix-cli/src/commands/completion.rs` | Modify | Use `clap_complete::Shell` directly; handle `Nushell` variant natively |
| `docs/en/book.toml` | Create | English mdBook config |
| `docs/en/src/SUMMARY.md` | Create | English book structure (Getting Started, Profiles, CLI Ref, CIS Mapping, Contributing) |
| `docs/en/src/*.md` | Create | English content chapters |
| `docs/es/book.toml` | Create | Spanish mdBook config |
| `docs/es/src/SUMMARY.md` | Create | Spanish book structure (mirror of English) |
| `docs/es/src/*.md` | Create | Spanish content chapters |
| `examples/packer/vallumix-hardened.pkr.hcl` | Create | Packer QEMU builder example with vallumix provisioner |
| `.github/workflows/release.yml` | Create | Cross-compile + package + GitHub Release on tag |
| `.github/workflows/docs.yml` | Create | mdBook build + deploy to GitHub Pages |
| `README.md` | Modify | Badges, features table, architecture SVG, usage examples, comparison table, installation |
| `CHANGELOG.md` | Modify | Seed with v0.0.1 entry under Keep a Changelog format |

## Interfaces / Contracts

### `clap_complete::Shell` re-export (replaces custom enum)

```rust
// In main.rs — replace custom Shell enum with:
pub use clap_complete::Shell;

// completion.rs — direct match without custom enum:
use clap_complete::Shell;
use clap_complete::generate;

pub fn run(shell: Shell) -> Result<i32> {
    let mut cmd = crate::Cli::command();
    generate(shell, &mut cmd, "vallumix", &mut std::io::stdout());
    Ok(0)
}
```

### `[package.metadata.deb]` (Cargo.toml)

```toml
[package.metadata.deb]
maintainer = "Vallumix Contributors <vallumix@example.com>"
section = "utils"
priority = "optional"
assets = [
    ["target/release/vallumix", "usr/bin/", "755"],
    ["man/vallumix.1", "usr/share/man/man1/", "644"],
]
```

### `[package.metadata.generate-rpm]` (Cargo.toml)

```toml
[package.metadata.generate-rpm]
assets = [
    { source = "target/release/vallumix", dest = "/usr/bin/vallumix", mode = "755" },
    { source = "man/vallumix.1", dest = "/usr/share/man/man1/vallumix.1", mode = "644" },
]
```

## Testing Strategy

| Layer | What to Test | Approach |
|-------|-------------|----------|
| Unit | `completion.rs` handles all `Shell` variants including Nushell | `assert_cmd` integration test that generates completions for each shell |
| Unit | `build.rs` generates man page file | CI asserts `man/vallumix.1` exists after build |
| Integration | `.deb` package structure | `cargo deb --no-build` + verify deb contents with `dpkg-deb` |
| Integration | `.rpm` package structure | `cargo generate-rpm` + verify with `rpm -qlp` |
| E2E | Release workflow produces all artifacts | Manual tag push triggers CI; verify all assets in GitHub Release |
| E2E | mdBook builds both languages | `mdbook build docs/en && mdbook build docs/es` |

## Migration / Rollout

No migration required. All changes are additive:
- Man page generation is build-time only (no runtime impact)
- Shell enum change is internal — `vallumix completion` CLI surface stays identical
- mdBook content is new (no existing docs to migrate)
- Packages are new CI artifacts (no existing packaging to replace)

## Open Questions

- [ ] Should the Packer example use QEMU builder or VirtualBox? (Exploration recommends QEMU — free, no cloud creds needed)
- [ ] Should `CHANGELOG.md` include the existing v0.0.1 entry from the workspace bootstrap, or start fresh?
- [ ] Should the release workflow use `cosign` signing now (Phase 4) or defer to Phase 5 (Launch)? PRD §6.2 and §12.3 mention it but Phase 5 is explicit about it.