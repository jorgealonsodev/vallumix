# Proposal: Phase 5 — Launch (v1.0.0 GA)

## Intent

Prepare Vallumix for v1.0.0 GA by adding supply-chain security (cosign signing, SLSA L3 attestations, SBOM), publishing to crates.io, fixing workspace metadata, and bumping the version. Addresses 8 gaps identified in exploration.

## Scope

### In Scope
- Cosign keyless signing of release artifacts via GitHub OIDC
- SLSA Level 3 provenance attestations via `slsa-framework/slsa-github-generator`
- SBOM generation with `cargo-sbom`
- crates.io publish for `vallumix-cli` and supporting library crates
- Fix repository URL and add missing `Cargo.toml` metadata (`description`, `keywords`, `categories`, `homepage`)
- Version bump to `1.0.0`

### Out of Scope
- Blog post and community announcements (content tasks, tracked separately)
- PRD v1.0.0 readiness preconditions (7-day green CI, external reviews, 0 critical issues)

## Capabilities

### New Capabilities
- `release-attestations`: cosign signing, SLSA L3 provenance, SBOM generation for release artifacts
- `crates-io-publish`: workspace metadata fixes, publish order orchestration, crates.io token integration

### Modified Capabilities
- `ci-pipeline`: extend release workflow requirements to include signing, attestations, SBOM, and crates.io publish steps

## Approach

Extend existing `release.yml` with additional jobs: cosign keyless signing (`sigstore/cosign-installer`), SLSA provenance (`slsa-framework/slsa-github-generator`), and SBOM generation (`cargo-sbom`). Add a crates.io publish job using `CARGO_REGISTRY_TOKEN`. Fix workspace `Cargo.toml` metadata and correct the repository URL to `jorgealonsodev/vallumix`. Bump workspace version to `1.0.0`.

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `.github/workflows/release.yml` | Modified | Add cosign, SLSA, SBOM, and crates.io publish jobs |
| `Cargo.toml` (workspace) | Modified | Fix repository URL, add metadata fields |
| `crates/*/Cargo.toml` | Modified | Add per-crate metadata; decide `publish` flag |
| `README.md` | Modified | Add SLSA / cosign badges |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| crates.io name conflict | Low | Verify name availability before publishing |
| SLSA L3 build isolation | Med | Use official `slsa-github-generator` builder |
| Publish order failure | Low | Publish in dependency order: core → controls → reporters → backup → cli |

## Rollback Plan

Revert `release.yml` via git revert. Yank crates.io versions within 24 hours if critical issues are found. Re-tag and re-release if necessary.

## Dependencies

- `CARGO_REGISTRY_TOKEN` secret configured in GitHub repository settings
- `crates.io` names available (no conflicts)

## Success Criteria

- [ ] Release artifacts signed with cosign and signatures attached to GitHub Release
- [ ] SLSA provenance attestation attached to GitHub Release
- [ ] SBOM generated and attached to GitHub Release
- [ ] `vallumix-cli` (and library crates) published on crates.io
- [ ] `Cargo.toml` repository URL points to `jorgealonsodev/vallumix`
- [ ] Workspace version bumped to `1.0.0`
