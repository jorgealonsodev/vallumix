# Design: Phase 5 — Launch (v1.0.0 GA)

## Technical Approach

Extend the existing `release.yml` workflow with three new jobs (cosign-sign, slsa-provenance, publish-crates) plus SBOM generation in the build job. Fix all Cargo.toml metadata to meet crates.io standards. Bump version from `0.0.1` → `1.0.0`. The release pipeline becomes: build → sign/SBOM → provenance → crates.io publish.

## Architecture Decisions

| Decision | Choice | Rejected | Rationale |
|----------|--------|----------|-----------|
| SLSA provenance | `slsa-framework/slsa-github-generator@v2` | Manual provenance, habusr | Industry-standard, drop-in, achieves L3 without custom builders |
| Signing method | Cosign keyless (GitHub OIDC) | Cosign with KMS-stored keys | No key management overhead; OIDC identity bound to repo+workflow |
| SBOM tool | `cargo-sbom` (CycloneDX JSON) | `cyclonedx-bom` CLI, spdx-rs | Direct integration with Cargo workspace; CycloneDX is SLSA-recommended format |
| crates.io token | `CARGO_REGISTRY_TOKEN` in GH Secrets | `CARGO_REGISTRY_TOKEN` in env file | Standard GH Actions secret pattern; scoped to repo, rotatable |
| Publish scope | All 5 crates (core→reporters→backup→controls→cli) | Only `vallumix-cli` | Libraries have standalone value; publishing all enables `cargo install` + library reuse |
| SBOM attach | Same release job as binaries | Separate SBOM workflow | Fewer workflows; SBOM is small and co-located with artifacts |
| SLSA workflow | Separate `slsa-release.yml` for generator | Inline in `release.yml` | SLSA generator requires its own reusable workflow reference; cleaner separation |

## Data Flow

```
v* tag push
    │
    ▼
┌──────────┐    ┌──────────┐
│  build    │    │  build    │
│  x86_64   │    │  aarch64  │
│  (musl)   │    │  (musl)   │
└────┬──────┘    └─────┬─────┘
     │                 │
     │  + cargo-sbom   │
     │  → sbom.json    │
     ▼                 ▼
┌──────────────────────────┐
│  release (prepare assets) │
│  SHA256SUMS, .deb, .rpm  │
└──────────┬───────────────┘
           │
     ┌─────┴──────┐
     ▼            ▼
┌─────────┐  ┌──────────────┐
│ cosign   │  │ slsa-release  │
│ sign     │  │ provenance    │
│ (keyless)│  │ (generator)  │
└────┬─────┘  └──────┬───────┘
     │               │
     │  .sig files   │  .intoto.jsonl
     ▼               ▼
┌────────────────────────────┐
│  gh-release (attach all)   │
│  binaries, .deb, .rpm,     │
│  SHA256SUMS, sbom.json,   │
│  *.sig, *.intoto.jsonl     │
└──────────┬─────────────────┘
           │
           ▼
┌────────────────────────────┐
│  publish-crates            │
│  core→reporters→backup→   │
│  controls→cli              │
│  (CARGO_REGISTRY_TOKEN)    │
└────────────────────────────┘
```

## File Changes

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` | Modify | Fix `repository` URL, add `description`, `keywords`, `categories`, `homepage`, `documentation`, version → `1.0.0` |
| `crates/vallumix-cli/Cargo.toml` | Modify | Add `description`, `keywords`, `categories`, `readme` override for crates.io listing |
| `crates/vallumix-core/Cargo.toml` | Modify | Add `description`, `keywords` per crate |
| `crates/vallumix-controls/Cargo.toml` | Modify | Add `description`, `keywords` per crate |
| `crates/vallumix-reporters/Cargo.toml` | Modify | Add `description`, `keywords` per crate |
| `crates/vallumix-backup/Cargo.toml` | Modify | Add `description`, `keywords` per crate |
| `.github/workflows/release.yml` | Modify | Add `id-token: write` perm, cosign install+sign job, cargo-sbom step, attach .sig/.sbom to release |
| `.github/workflows/slsa-release.yml` | Create | SLSA L3 reusable workflow calling `slsa-framework/slsa-github-generator@v2` |
| `CHANGELOG.md` | Modify | `[Unreleased]` → `[1.0.0] - 2026-05-01`, add new `[Unreleased]` |
| `README.md` | Modify | Add SLSA/cosign verification badges |

## Interfaces / Contracts

### Workspace Cargo.toml additions

```toml
[workspace.package]
version = "1.0.0"
description = "Modular Linux hardening engine with CIS-aligned profiles"
keywords = ["cis", "linux", "hardening", "compliance", "security"]
categories = ["command-line-utilities", "os"]
homepage = "https://vallumix.dev"
documentation = "https://docs.vallumix.dev"
repository = "https://github.com/jorgealonsodev/vallumix"
```

### crates.io publish order

```bash
cargo publish -p vallumix-core
cargo publish -p vallumix-reporters
cargo publish -p vallumix-backup
cargo publish -p vallumix-controls
cargo publish -p vallumix-cli
```

### Cosign verification command (for README)

```bash
cosign verify-blob vallumix-x86_64-unknown-linux-musl \
  --certificate vallumix-x86_64-unknown-linux-musl.sig \
  --certificate-identity https://github.com/jorgealonsodev/vallumix/.github/workflows/release.yml \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
```

## Testing Strategy

| Layer | What to Test | Approach |
|-------|-------------|----------|
| Unit | Cargo.toml metadata completeness | `cargo publish --dry-run` for each crate validates metadata |
| Integration | Release workflow YAML validity | `actionlint` on workflow files; workflow syntax check |
| Integration | crates.io publish dry-run | `cargo publish -p <crate> --dry-run` before real publish |
| E2E | Full release pipeline | Tag push → verify GitHub Release has binaries + .sig + SBOM + provenance |
| E2E | cosign verify-blob | Download release binary + signature, run `cosign verify-blob` |
| E2E | crates.io listing | `curl https://crates.io/api/v1/crates/vallumix-cli` confirms 1.0.0 |

## Migration / Rollout

No data migration required. Release order: tag `v1.0.0` → pipeline runs → crates.io publish is last step. If crates.io publish fails, yank within 24h with `cargo yank`. If signing fails, re-tag after fix. The `repository` URL fix is a breaking metadata change but has zero runtime impact.

## Open Questions

- [ ] Verify `vallumix-cli` name is available on crates.io (no conflict)
- [ ] Decide if `homepage` / `documentation` URLs should be GitHub Pages (already deployed via docs.yml) or custom domain
- [ ] Confirm `CARGO_REGISTRY_TOKEN` is already configured in repo secrets (dependency for publish job)