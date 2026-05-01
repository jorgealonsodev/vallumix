# Tasks: phase5-launch — Launch

**Change**: phase5-launch
**Total Tasks**: 17
**Mode**: hybrid (Engram + file)
**Skill Resolution**: none

---

## Phase 1: Repo Fix + Cargo Metadata

- [ ] 1.1 Fix workspace `repository` in `Cargo.toml` from `vallumix/vallumix` → `https://github.com/jorgealonsodev/vallumix`
- [ ] 1.2 Add `description`, `keywords = ["cis", "linux", "hardening", "compliance"]`, `categories = ["Command Line Tools", "Security"]`, `homepage`, `documentation` to `[workspace.package]` in `Cargo.toml`
- [ ] 1.3 Add `publish = false` to `[workspace.package]` in `Cargo.toml` to prevent accidental workspace publish
- [ ] 1.4 Add `description = "Vallumix CLI — CIS Linux hardening tool"`, `keywords`, `readme = "README.md"`, `license = "MIT OR Apache-2.0"`, `categories` to `crates/vallumix-cli/Cargo.toml`

---

## Phase 2: Version Bump

- [ ] 2.1 Bump `version` in `[workspace.package]` from `0.0.1` → `1.0.0` in `Cargo.toml`
- [ ] 2.2 Update `CHANGELOG.md`: rename `[Unreleased]` section → `[1.0.0] - 2026-05-01`, add new `[Unreleased]` header for future work

---

## Phase 3: SLSA Provenance Workflow

- [ ] 3.1 Create `.github/workflows/slsa-release.yml` using `slsa-framework/slsa-github-generator@v2` with `triggers: push: tags: v*`, `depends_on: build`, and `category: download/artifacts`
- [ ] 3.2 Add `slsa-github-generator` step in `.github/workflows/release.yml` after the build job to generate SLSA Level 3 provenance for all release artifacts (binaries, .deb, .rpm)
- [ ] 3.3 Verify `slsa-release.yml` uploads provenance attestation file (`.attestation`) as a release asset

---

## Phase 4: Cosign Keyless Signing

- [ ] 4.1 Add `id-token: write` permission + `cosign` install step (`sigstore/cosign-installer@v3`) in `release.yml` build job
- [ ] 4.2 Sign all release artifacts in `release.yml` with `cosign sign --yes` using GitHub OIDC (keyless mode) — binaries, .deb, .rpm, man page, SHA256SUMS
- [ ] 4.3 Attach cosign `.sig` files to the GitHub Release in `release.yml` via `softprops/action-gh-release`
- [ ] 4.4 Add `cosign verify` step in release.yml to verify signatures before publishing (optional guard)

---

## Phase 5: Crates.io Publish

- [ ] 5.1 Add `cargo install cargo-dist` or `cargo publish` step in a new `publish` job in `release.yml`, triggered after signing, gated on tag push to main
- [ ] 5.2 Add `CARGO_REGISTRY_TOKEN` secret to repository via GitHub Actions environment secrets; add env reference in publish job
- [ ] 5.3 Publish crates in dependency order: `vallumix-core` → `vallumix-reporters` → `vallumix-backup` → `vallumix-controls` → `vallumix-cli` using individual `cargo publish -p <crate>` commands
- [ ] 5.4 Verify crates appear on crates.io after publish (check name availability, publish dry-run first)

---

## Phase 6: SBOM Generation

- [ ] 6.1 Add `cargo install cargo-sbom` to build job in `release.yml` and generate SBOM with `cargo sbom --output-format cyclonedx-json -p vallumix-cli`
- [ ] 6.2 Attach SBOM JSON file as a release asset in the `softprops/action-gh-release` step
- [ ] 6.3 Sign the SBOM with cosign and attach `.sig` file alongside the SBOM

---

## Implementation Order

1. Phase 1 (Repo fix + metadata) — Must be done first; blocks publishing
2. Phase 2 (Version bump) — Depends on Phase 1; `1.0.0` is the release tag
3. Phase 3 (SLSA) — Depends on Phase 2; provenance tied to the tagged artifact
4. Phase 4 (Cosign) — Depends on Phase 2+3; signs what was built and attested
5. Phase 5 (Crates.io) — Final publish step; depends on all signing being complete
6. Phase 6 (SBOM) — Can run in parallel with cosign signing; both attach to release

**Total: 17 tasks across 6 phases**

**Next**: Ready for sdd-apply.
