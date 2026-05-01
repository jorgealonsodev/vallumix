# Phase 4: Polish — Exploration Report

## PRD Reference

Section 8 — Phase 4 (2 weeks):
> Documentación bilingüe con `mdBook`, demos en video, integración Packer ejemplo, página man, autocompletado, paquetes `.deb` y `.rpm`

Section 6.5 (Usabilidad):
- Página man generada automáticamente con `clap_mangen` durante el build
- Autocompletado para Bash, Zsh, Fish y Nushell generado por `vallumix completion`

Section 3.2 (Objetivos secundarios):
- Documentar el proyecto en español e inglés, incluyendo guías de uso, contribución, mapeo CIS-control y documentación de API generada con `cargo doc`
- Publicar en `crates.io` y mantener paquetes `.deb` y `.rpm` generados con `cargo-deb` y `cargo-generate-rpm`

## 1. Current State

### Documentation
- **README.md**: Minimal (39 lines) — quick start, profiles, contributing link, license. Missing badges, features table, architecture diagram, usage examples, screenshots.
- **CONTRIBUTING.md**: Basic build/test/lint instructions. Missing code style guide, PR template details, issue templates reference.
- **docs/**: Does NOT exist. No mdBook structure, no bilingual content.
- **CHANGELOG.md**: Does NOT exist (PRD section 6.4 requires it).

### Man Page
- **clap_mangen**: NOT in dependencies. PRD section 6.5 requires it.
- No man page generation exists anywhere in the project.

### Autocompletion
- **clap_complete**: Already in workspace dependencies (`4.5`) and `vallumix-cli` Cargo.toml.
- `Shell` enum defined in `main.rs` with Bash, Zsh, Fish, Nushell variants.
- `completion.rs` exists with working implementation for Bash/Zsh/Fish.
- **Nushell**: Stub only — prints "not yet supported by clap_complete" (this is outdated; clap_complete 4.5 supports nushell).
- Integration test exists: `cli_completion_bash` in `tests/cli.rs`.

### Packaging (.deb / .rpm)
- **cargo-deb**: NOT in dependencies.
- **cargo-generate-rpm**: NOT in dependencies.
- No `[package.metadata.deb]` or `[package.metadata.generate-rpm]` sections in any Cargo.toml.

### CI/CD
- CI workflow exists with multi-distro matrix (debian:12, ubuntu:22.04, ubuntu:24.04, rockylinux:9).
- No release workflow, no packaging jobs, no docs deployment.
- No GitHub Pages / mdBook deployment.

### Packer Integration
- No Packer example exists. PRD mentions Packer integration in persona 2 (Carlos, DevOps engineer) and use case CU-05.

### Demo Videos
- No videos exist (expected — this is a deliverable, not code).

## 2. Affected Areas

| File/Area | Why Affected |
|-----------|-------------|
| `Cargo.toml` (workspace) | Add `clap_mangen` dependency |
| `crates/vallumix-cli/Cargo.toml` | Add `clap_mangen` dep, `[package.metadata.deb]`, `[package.metadata.generate-rpm]` |
| `crates/vallumix-cli/src/main.rs` | Fix Nushell completion, add `generate_man` subcommand or build script |
| `crates/vallumix-cli/src/commands/completion.rs` | Fix Nushell support (clap_complete 4.5 has `Shell::Nushell`) |
| `crates/vallumix-cli/src/commands/` | New `generate_man.rs` command OR build-time generation |
| `docs/` | NEW — mdBook structure (book.toml, es/, en/) |
| `docs/book.toml` | NEW — mdBook configuration |
| `examples/packer/` | NEW — Packer HCL example |
| `.github/workflows/release.yml` | NEW — release pipeline with .deb/.rpm/artifacts |
| `.github/workflows/docs.yml` | NEW — mdBook build + deploy to GitHub Pages |
| `README.md` | Enhancement — badges, features, architecture diagram, usage examples |
| `CHANGELOG.md` | NEW — Keep a Changelog format |

## 3. Approaches

### 3.1 Man Page Generation

| Approach | Pros | Cons | Effort |
|----------|------|------|--------|
| **A: Build script (`build.rs`)** | Generated at compile time, always in sync, ships with binary | Adds build dependency, slower builds | Low |
| **B: Runtime subcommand (`vallumix man`)** | No build overhead, user generates on demand | Extra step for users, not standard | Low |
| **C: CI-generated + release artifact** | Generated once per release, clean | Not available for source builds | Low |

**Recommendation: Approach A (build.rs)** — Standard pattern in Rust CLI ecosystem. `clap_mangen` is designed for this. Generate to `man/` directory during build.

### 3.2 mdBook Bilingual Documentation

| Approach | Pros | Cons | Effort |
|----------|------|------|--------|
| **A: Two separate mdBooks** (`docs/en/`, `docs/es/`) | Simple, each book is independent | Two `book.toml` files, two builds | Medium |
| **B: Single mdBook with language selector** | One build, unified navigation | Requires custom theme/plugin, more complex | High |
| **C: mdBook-i18n-helper** | Official i18n tooling, `mdbook-i18n-helper` | Extra dependency, POT/PO workflow | Medium |

**Recommendation: Approach A (two separate books)** — Simplest, most maintainable. Each language gets its own directory with `book.toml` and `src/`. Deploy both to GitHub Pages at `/en/` and `/es/` paths.

### 3.3 Package Generation (.deb / .rpm)

| Approach | Pros | Cons | Effort |
|----------|------|------|--------|
| **A: `cargo-deb` + `cargo-generate-rpm`** | Standard Rust tooling, metadata in Cargo.toml | Two separate tools to configure | Low |
| **B: `cargo-deb` only** | Simpler, Debian is primary target | No RPM for RHEL/Rocky users | Low |
| **C: Manual `fpm` or `dpkg-deb`** | Full control | Not idiomatic Rust, more maintenance | High |

**Recommendation: Approach A** — Both tools are standard in the Rust ecosystem. PRD explicitly mentions both.

### 3.4 Autocompletion Fix

| Approach | Pros | Cons | Effort |
|----------|------|------|--------|
| **A: Use `clap_complete::Shell::Nushell`** | clap_complete 4.5 supports it natively | Need to verify version compatibility | Trivial |
| **B: Keep stub, document limitation** | No risk | Poor UX for Nushell users | Trivial |

**Recommendation: Approach A** — `clap_complete` 4.5 added Nushell support. Just use `Shell::Nushell` variant from clap_complete instead of our custom enum.

### 3.5 README Enhancement

| Approach | Pros | Cons | Effort |
|----------|------|------|--------|
| **A: Full rewrite with all sections** | Professional, comprehensive | Takes time to write well | Medium |
| **B: Incremental additions** | Low risk, can iterate | Might feel inconsistent | Low |

**Recommendation: Approach A** — Do it once, do it right. Include: badges, elevator pitch, features table, architecture diagram (SVG), quick start, usage examples, comparison table (from PRD), contributing link, license.

## 4. Detailed Work Breakdown

### 4.1 Man Page (`clap_mangen`)
1. Add `clap_mangen = "0.2"` to workspace deps
2. Create `build.rs` in `vallumix-cli` that generates man page to `man/vallumix.1`
3. Include man page in `.deb` and `.rpm` packages
4. Add to `README.md` usage section

### 4.2 Autocompletion Fix
1. Replace custom `Shell` enum with `clap_complete::Shell` in `main.rs`
2. Update `completion.rs` to use `clap_complete::Shell::Nushell`
3. Update tests

### 4.3 .deb Package (`cargo-deb`)
1. Add `cargo-deb` to workspace deps
2. Add `[package.metadata.deb]` to `vallumix-cli/Cargo.toml`:
   - maintainer, depends, section, priority, assets (binary + man page + completions)
3. Test with `cargo deb --no-build`

### 4.4 .rpm Package (`cargo-generate-rpm`)
1. Add `cargo-generate-rpm` to workspace deps
2. Add `[package.metadata.generate-rpm]` to `vallumix-cli/Cargo.toml`:
   - assets, auto-req, license
3. Test with `cargo generate-rpm`

### 4.5 mdBook Documentation
1. Create `docs/en/book.toml` and `docs/en/src/SUMMARY.md`
2. Create `docs/es/book.toml` and `docs/es/src/SUMMARY.md`
3. Write content: Getting Started, Profiles, CLI Reference, CIS Mapping, Contributing
4. Add `.github/workflows/docs.yml` for GitHub Pages deployment

### 4.6 Packer Example
1. Create `examples/packer/vallumix-hardened.pkr.hcl`
2. Include provisioner that downloads and runs vallumix
3. Document in README and mdBook

### 4.7 README Enhancement
1. Add badges (CI, license, crates.io, MSRV)
2. Add features table
3. Add architecture SVG diagram
4. Add usage examples for all subcommands
5. Add comparison table (from PRD section 2.2)
6. Add installation section (crates.io, .deb, .rpm, from source)

### 4.8 CHANGELOG.md
1. Create with Keep a Changelog format
2. Add [Unreleased] section
3. Add v0.0.1 section (initial release)

### 4.9 Release CI Workflow
1. Create `.github/workflows/release.yml`
2. Trigger on tags (v*)
3. Build release binary (x86_64 + aarch64)
4. Generate .deb and .rpm
5. Generate man page
6. Create GitHub Release with artifacts

## 5. Risks

1. **`clap_mangen` version compatibility**: Must match clap 4.5. `clap_mangen` 0.2.x is compatible with clap 4.x.
2. **mdBook content volume**: Writing comprehensive bilingual docs is the largest effort item. Need to scope appropriately.
3. **Packer example requires cloud credentials**: Example should work with free/local providers (e.g., QEMU builder).
4. **Release workflow complexity**: Cross-compilation in CI for release artifacts adds significant complexity.
5. **Nushell completion**: Need to verify clap_complete 4.5 actually supports Nushell (it does as of 4.5.0+).
6. **Package size**: `.deb` and `.rpm` packages must include man page and completions in correct paths.

## 6. Dependencies to Add

```toml
# workspace dependencies
clap_mangen = "0.2"
# cargo-deb and cargo-generate-rpm are cargo subcommands, not library deps
# They are installed via `cargo install`, not added to Cargo.toml
```

**Correction**: `cargo-deb` and `cargo-generate-rpm` are NOT library dependencies. They are cargo subcommands installed separately. The metadata goes in Cargo.toml but the tools themselves are installed via `cargo install cargo-deb` and `cargo install cargo-generate-rpm`.

For CI, add installation steps:
```yaml
- name: Install cargo-deb
  run: cargo install cargo-deb
- name: Install cargo-generate-rpm
  run: cargo install cargo-generate-rpm
```

## 7. Recommendation

Execute all items in this phase. They are well-scoped, non-breaking, and directly align with the PRD's Phase 4 deliverables. The highest-value items are:

1. **README enhancement** — First impression for all users
2. **mdBook docs** — Required for v1.0 acceptance criteria (PRD section 12.3, item 5)
3. **Man page + autocompletion** — Core usability requirements (PRD section 6.5)
4. **Package generation** — Required for v1.0 acceptance criteria (PRD section 12.3, item 9)
5. **Release CI** — Enables all packaging deliverables
6. **Packer example** — Validates CI/CD integration use case
7. **CHANGELOG** — Required by PRD section 6.4

Demo videos are outside code scope — they should be planned as a separate deliverable (recording, editing, hosting).

## 8. Ready for Proposal

**Yes.** This exploration provides sufficient detail to create a proposal, design, spec, and tasks for Phase 4. The scope is well-defined, approaches are compared, and risks are identified.
