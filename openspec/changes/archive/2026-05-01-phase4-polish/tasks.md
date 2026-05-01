# Tasks: phase4-polish — Polish

**Change**: phase4-polish
**Total Tasks**: 28
**Mode**: hybrid (Engram + file)
**Skill Resolution**: none

---

## Phase 1: README Enhancement

### [x] T-001 — readme-enhancement
Add CI status badge, license badge (MIT OR Apache-2.0), crates.io version badge, MSRV badge (1.75), and coverage badge to README.md header using shield.io URLs.

### [x] T-002 — readme-enhancement
Add features table listing: idempotent execution, dry-run mode, rollback, 3 profile presets (web/database/bastion), 4 report formats (HTML/JSON/JUnit/text), 70+ CIS controls, multi-distro support (Debian 12, Ubuntu 22.04/24.04, RHEL 9, Rocky 9, AlmaLinux 9), static musl binary.

### [x] T-003 — readme-enhancement
Add usage examples section with copy-paste-ready code blocks for all 5 subcommands:
- `vallumix apply --profile web --dry-run`
- `vallumix audit --profile web --report html`
- `vallumix rollback --session <id>`
- `vallumix list --profile database`
- `vallumix completion bash`

### [x] T-004 — readme-enhancement
Add comparison table from PRD §2.2: Vallumix vs OpenSCAP vs Lynis vs Ansible Lockdown, comparing Language, Distribution, Applies Changes, Profiles, Rollback, Report Types, and Memory Safety.

### [x] T-005 — readme-enhancement
Add architecture SVG diagram showing crate graph: `vallumix-cli` → `vallumix-core` ← `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`. Include directional arrows.

### [x] T-006 — readme-enhancement
Add installation section listing all methods: from source (`cargo install`), from crates.io, .deb package (GitHub Releases), .rpm package (GitHub Releases). Include `sudo dpkg -i vallumix_*.deb` and `sudo rpm -i vallumix-*.rpm` commands.

---

## Phase 2: Autocomplete Fix

### [x] T-007 — autocomplete-fix
Replace custom `Shell` enum in `crates/vallumix-cli/src/main.rs` with `pub use clap_complete::Shell`. Remove the `#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)] pub enum Shell` block entirely.

### [x] T-008 — autocomplete-fix
Update `crates/vallumix-cli/src/commands/completion.rs`: remove stub for Nushell, call `generate()` directly for all shells using `clap_complete::Shell`. Import `Shell` from `clap_complete` (aliased or direct).

### [x] T-009 — autocomplete-fix
Update `Commands::Completion { shell: Shell }` in main.rs to use `clap_complete::Shell`. Update `commands::completion::run(*shell)` call to pass the re-exported shell type.

### [x] T-010 — autocomplete-fix
Add integration test `cli_completion_nushell` in `tests/cli.rs` that runs `vallumix completion nushell` and asserts output does not contain "not yet supported" or error messages.

---

## Phase 3: Man Page Generation

### [x] T-011 — man-page
Add `clap_mangen = "0.2"` to `[workspace.dependencies]` in `Cargo.toml`.

### [x] T-012 — man-page
Add `clap_mangen` as a dev-dependency in `crates/vallumix-cli/Cargo.toml` with version from workspace.

### [x] T-013 — man-page
Create `crates/vallumix-cli/build.rs` that generates `man/vallumix.1` using `clap_mangen::Man` from `Cli::command()`. Write output to `OUT_DIR` and copy to `man/vallumix.1` in project root. Include sections: NAME, SYNOPSIS, DESCRIPTION, SUBCOMMANDS, OPTIONS, EXIT CODES.

### [x] T-014 — man-page
Update `.gitignore` to include `man/vallumix.1` and `man/*.1`. Add `man/` directory to `.gitignore` if not already present.

### [x] T-015 — man-page
Verify `cargo build -p vallumix-cli` produces `man/vallumix.1`. Run `man ./man/vallumix.1` to confirm sections for apply, audit, rollback, list, completion are present and exit codes 0-3 are documented.

---

## Phase 4: mdBook Documentation

### [x] T-016 — mdbook-docs
Create `docs/en/book.toml` with title "Vallumix Documentation", src-dir pointing to `src`, and output-dir pointing to `book`. Set language to English.

### [x] T-017 — mdbook-docs
Create `docs/en/src/SUMMARY.md` with entries for: Getting Started, Installation, Quick Start, CLI Reference, Profiles Guide, CIS Control Mapping, Contributing, API Reference.

### [x] T-018 — mdbook-docs
Write English chapter files under `docs/en/src/` with ≥100 words each: `getting-started.md`, `installation.md`, `cli-reference.md`, `profiles.md`, `cis-mapping.md`, `contributing.md`, `api-reference.md`.

### [x] T-019 — mdbook-docs
Create `docs/es/book.toml` mirroring English config with Spanish title and language set to Spanish.

### [x] T-020 — mdbook-docs
Create `docs/es/src/SUMMARY.md` with same structure as English but chapter titles in Spanish.

### [x] T-021 — mdbook-docs
Write Spanish chapter files under `docs/es/src/` with same structure as English chapters.

### [x] T-022 — mdbook-docs
Create `.github/workflows/docs.yml` that triggers on push to `develop`, runs `mdbook build docs/en` and `mdbook build docs/es`, and deploys to GitHub Pages at `/en/` and `/es/` paths.

### [x] T-023 — mdbook-docs
Add link to `cargo doc --no-deps` generated API docs in API Reference chapter. Verify with `mdbook build docs/en` that book builds without errors.

---

## Phase 5: Debian and RPM Packages

### [x] T-024 — deb-rpm-packages
Add `[package.metadata.deb]` section to `crates/vallumix-cli/Cargo.toml` with: maintainer ("Vallumix Contributors <vallumix@example.com>"), section ("admin"), priority ("optional"), depends ("libc6"), and assets mapping binary to `/usr/bin/`, man page to `/usr/share/man/man1/`, bash completions to `/usr/share/bash-completion/`, zsh completions to `/usr/share/zsh/vendor-completions/`.

### [x] T-025 — deb-rpm-packages
Add `[package.metadata.generate-rpm]` section with: assets (binary at /usr/bin/vallumix mode 755, man page at /usr/share/man/man1/ mode 644), auto-req = "no", and license matching MIT OR Apache-2.0.

### [x] T-026 — deb-rpm-packages
Create `CHANGELOG.md` at project root in Keep a Changelog format. Include `[Unreleased]` section and `[0.0.1] - 2026-04-30` section with Added subsection listing "Initial workspace bootstrap", "CLI structure with clap", "Core traits and error types".

### [x] T-027 — deb-rpm-packages
Install `cargo-deb` and run `cargo deb --no-build` to verify `.deb` package is produced. Inspect with `dpkg-deb -I` to confirm maintainer, section, priority, and assets are correct.

### [x] T-028 — deb-rpm-packages
Install `cargo-generate-rpm` and run `cargo generate-rpm` to verify `.rpm` package is produced. Inspect with `rpm -qip` to confirm summary, license, and vendor fields.

---

## Phase 6: Packer Example

### [x] T-029 — packer-example
Create `examples/packer/vallumix-hardened.pkr.hcl` with QEMU builder targeting Debian 12 (source `debian-12-image` or equivalent). Configure 4GB RAM, 2 CPUs, and 20GB disk.

### [x] T-030 — packer-example
Add shell provisioner that downloads Vallumix from GitHub release (use variable for URL) and installs via `dpkg -i`. Run `sudo vallumix apply --profile web --report html` as provisioner step.

### [x] T-031 — packer-example
Create `examples/packer/README.md` documenting prerequisites (Packer, QEMU, 10GB disk space), build command (`packer build vallumix-hardened.pkr.hcl`), expected output, and links to main documentation.

---

## Phase 7: Release CI Pipeline

### [x] T-032 — release-ci
Create `.github/workflows/release.yml` triggered on push tags matching `v*`. Use `actions/checkout@v4` with fetch-depth 0 for changelog access.

### [x] T-033 — release-ci
Add step to `cargo install cargo-deb cargo-generate-rpm upx`. Install `cross` for aarch64 cross-compilation.

### [x] T-034 — release-ci
Build x86_64-musl release: `cross build --target x86_64-unknown-linux-musl --release`. Strip binary with `strip` and compress with `upx --best --strip 1`. Repeat for aarch64 target.

### [x] T-035 — release-ci
Generate packages: `cargo deb --release --target x86_64-unknown-linux-musl` and `cargo generate-rpm --release --target aarch64-unknown-linux-musl`. Produce SHA256 checksums for all artifacts.

### [x] T-036 — release-ci
Create GitHub Release using `softprops/action-gh-release@v1` with changelog body extracted from `CHANGELOG.md` for the matching version section. Attach all binaries, packages, man page, and SHA256SUMS as release assets.

### [x] T-037 — release-ci
Verify release artifacts: two musl binaries (< 8MB each), `.deb` for x86_64, `.rpm` for aarch64, `SHA256SUMS`, and man page. Verify CI run produces all assets on a test tag push.

---

**Total: 37 tasks across 7 phases**

**Implementation Order**:
1. README (T-001 to T-006) — First impression for all users, no deps
2. Autocomplete fix (T-007 to T-010) — Small internal refactor before build.rs work
3. Man page (T-011 to T-015) — build.rs pattern, depends on autocomplete Shell enum cleanup
4. mdBook docs (T-016 to T-023) — Content-heavy, independent of code changes
5. Deb/RPM packages (T-024 to T-028) — Metadata in Cargo.toml, after man page (needs man page for assets)
6. Packer example (T-029 to T-031) — Standalone HCL, independent
7. Release CI (T-032 to T-037) — Depends on all packaging being configured

**Next**: Ready for sdd-apply.