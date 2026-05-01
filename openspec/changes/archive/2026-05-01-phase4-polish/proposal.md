# Proposal: phase4-polish

## Intent

Execute PRD Section 8 Phase 4: 2-week polish sprint to make Vallumix distributable and documented before v0.9 RC. Covers v1.0 acceptance criteria for bilingual docs, man page, packages, and release automation.

## Scope

### In Scope
- README enhancement (badges, install, architecture).
- Bilingual mdBook docs (`docs/en/` + `docs/es/`) on GitHub Pages.
- Man page via `clap_mangen` in `build.rs`.
- `.deb` (cargo-deb) and `.rpm` (cargo-generate-rpm) metadata.
- Fix Nushell completion stub (clap_complete 4.5+).
- Packer HCL example under `examples/packer/`.
- Release CI workflow.

### Out of Scope
- Demo videos (non-code; tracked separately).
- crates.io publish and SLSA attestation (Phase 5).

## Capabilities

### New
- `docs-mdbook`: Bilingual user guide, contribution guide, CIS mapping.
- `packaging-deb-rpm`: Distribution packages.
- `man-page`: Auto-generated man(1) from clap.
- `release-ci`: Multi-arch release pipeline.

### Modified
- `cli-completion`: Enable native Nushell generation.

## Approach

- **README**: shields, quick install, architecture diagram, profile matrix.
- **mdBook**: two books (`docs/en/`, `docs/es/`) with shared snippets; deploy to `gh-pages`.
- **Man page**: `build.rs` generates `target/man/vallumix.1`; included in packages.
- **Packages**: add `[package.metadata.deb]` and `[package.metadata.generate-rpm]` to `vallumix-cli/Cargo.toml`.
- **Autocomplete**: replace Nushell stub with `ClapShell::Nushell`.
- **Packer example**: `examples/packer/vallumix.pkr.hcl` with non-interactive apply.
- **Release CI**: `.github/workflows/release.yml` on tags: build x86_64/aarch64 musl, test, package, create Release.

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `README.md` | Modified | Badges, install, architecture |
| `docs/` | New | mdBook en/es |
| `crates/vallumix-cli/build.rs` | New | Man page generation |
| `crates/vallumix-cli/Cargo.toml` | Modified | clap_mangen, deb/rpm metadata |
| `crates/vallumix-cli/src/commands/completion.rs` | Modified | Nushell native |
| `examples/packer/` | New | HCL example |
| `.github/workflows/release.yml` | New | Release pipeline |
| `.github/workflows/docs.yml` | New | mdBook deploy |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| cargo-deb / cargo-generate-rpm fail on musl cross-compile | Med | Test in CI; fall back to native |
| mdBook bilingual drift | Med | Shared snippets; PR checklist |
| Release CI secrets issue | Low | Use `GITHUB_TOKEN` only |

## Rollback Plan

Revert commits or delete `docs/`, `examples/`, workflows; remove `clap_mangen` and package metadata; restore prior README from git.

## Dependencies

- `mdbook`, `cargo-deb`, `cargo-generate-rpm` in CI.

## Success Criteria

- [ ] README renders with badges and install steps.
- [ ] `mdbook build` succeeds for both languages.
- [ ] `man ./target/man/vallumix.1` displays correctly.
- [ ] `cargo deb` and `cargo generate-rpm` produce packages.
- [ ] `vallumix completion nushell` emits valid script.
- [ ] Packer example validates with `packer validate`.
- [ ] Release CI produces artifacts on a test tag.
