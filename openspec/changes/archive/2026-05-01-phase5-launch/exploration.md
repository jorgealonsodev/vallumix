## Exploration: phase5-launch

### Current State

Vallumix is a Rust workspace with 5 crates at version `0.0.1`. The project has:

- **CI Pipeline** (`.github/workflows/ci.yml`): Multi-distro testing (Debian 12, Ubuntu 22.04/24.04, Rocky 9), aarch64 cross-build, coverage (tarpaulin), dependency audit (cargo-audit, cargo-deny), insta snapshot validation.
- **Release Pipeline** (`.github/workflows/release.yml`): Triggered on `v*` tags. Builds x86_64/aarch64 musl binaries with `cross`, strips + UPX compresses, generates `.deb` (x86_64) and `.rpm` (aarch64) packages, creates GitHub Release with SHA256SUMS and extracted changelog.
- **Cargo.toml**: Workspace-level metadata includes `version`, `edition`, `authors`, `license`, `repository`, `rust-version`. **Missing**: `description`, `keywords`, `categories`, `homepage`, `documentation` — all required for a good crates.io listing. No `publish = false` on workspace root, meaning all crates would be publishable by default.
- **README.md**: Has CI, license, crates.io, MSRV, and coverage badges. References `cargo install vallumix-cli` (crates.io path already documented).
- **CHANGELOG.md**: Follows Keep a Changelog format with `[Unreleased]` and `[0.0.1]` sections.
- **No git tags exist yet** — no releases have been made.
- **GitHub repo**: `jorgealonsodev/vallumix` (Cargo.toml `repository` field says `vallumix/vallumix` — **mismatch**).
- **mdBook docs**: Bilingual (en/es) deployed to GitHub Pages via `.github/workflows/docs.yml`.

### PRD Phase 5 Requirements

Per `vallumix.prd` section 8 (Plan de Desarrollo), Phase 5 "Lanzamiento" requires:

1. **Release signed with `cosign`**
2. **Published to `crates.io`**
3. **SLSA attestations** (Level 3 per PRD section 6.2)
4. **Blog post** (technical)
5. **Announcement** on `r/rust`, `r/linux`, and Hacker News

### Affected Areas

- `.github/workflows/release.yml` — Must add cosign signing, SLSA provenance generation, crates.io publish step
- `Cargo.toml` (workspace) — Must add `description`, `keywords`, `categories`, fix `repository` URL
- `crates/vallumix-cli/Cargo.toml` — May need `description`, `keywords`, `readme` override for crates.io
- Other crate `Cargo.toml` files — Must decide which crates to publish (core + controls as libraries? or only cli?)
- `README.md` — May add SLSA/cosign badges
- `.github/workflows/` — May need new workflow for SBOM generation, or extend release.yml
- `CHANGELOG.md` — Must update for v1.0.0 release notes

### Approaches

#### 1. **Extend existing release.yml** (recommended)
Add cosign signing, SLSA provenance, and crates.io publish as additional steps/jobs in the current release workflow.

- **Pros**: Single workflow, simpler to maintain, all release artifacts generated together
- **Cons**: Workflow becomes longer/more complex; SLSA provenance requires specific build environment
- **Effort**: Medium

#### 2. **Separate workflows per concern**
Keep release.yml for binary/release, add `publish.yml` for crates.io, add `attestations.yml` for SLSA.

- **Pros**: Cleaner separation, easier to debug individual steps
- **Cons**: More workflows to maintain, potential race conditions between jobs
- **Effort**: Medium-High

#### 3. **Use slsa-framework/slsa-github-generator**
Leverage the official SLSA GitHub generator for provenance attestation.

- **Pros**: Industry-standard, generates SLSA Level 3 provenance automatically, verifiable by third parties
- **Cons**: Requires specific builder image, less control over build process
- **Effort**: Low (drop-in integration)

### Key Findings & Gaps

| Area | Current State | Gap for Phase 5 |
|------|--------------|-----------------|
| **cosign signing** | Not implemented | Need cosign keyless signing (sigstore) for binary + SBOM |
| **SLSA attestations** | Not implemented | Need `slsa-github-generator` or manual provenance generation |
| **crates.io publish** | Not implemented | Need `cargo publish` with token, metadata fixes first |
| **Cargo metadata** | Minimal | Missing `description`, `keywords`, `categories`, `homepage` |
| **Repository URL** | `vallumix/vallumix` | Actual repo is `jorgealonsodev/vallumix` — **must fix** |
| **SBOM** | Not implemented | PRD mentions `cargo-sbom` — needed for cosign attestation |
| **Publish scope** | Unclear | Which crates to publish? Only `vallumix-cli`? Or also `vallumix-core`, `vallumix-controls` as libraries? |
| **Version bump** | `0.0.1` | Phase 5 is v1.0.0 GA — needs version bump |
| **Blog post** | Not started | Content creation (not code) |
| **Announcements** | Not started | Community outreach (not code) |

### Risks

1. **Repository URL mismatch**: Cargo.toml says `vallumix/vallumix` but actual repo is `jorgealonsodev/vallumix`. This will break crates.io links and must be fixed before publishing.
2. **crates.io name conflict**: `vallumix-cli` must be available on crates.io. If the name is taken, publishing will fail.
3. **SLSA Level 3 requirements**: Requires hermetic builds, isolated builders, and verified provenance. The current `cross`-based build may not meet all SLSA L3 requirements without the official slsa-github-generator.
4. **Cosign keyless signing**: Requires OIDC identity (GitHub Actions OIDC). Works well in CI but needs proper configuration.
5. **Publish order**: If publishing multiple crates, dependency order matters (`vallumix-core` before `vallumix-cli`).
6. **v1.0.0 readiness**: PRD acceptance criteria (section 12.3) require 7 days of green CI, 0 clippy warnings, 0 audit alerts, bilingual docs, external reviews, 0 critical issues. These are preconditions, not Phase 5 tasks.

### Recommendation

**Approach 1 + 3 combined**: Extend the existing `release.yml` workflow with:
1. Fix Cargo.toml metadata (description, keywords, categories, repository URL)
2. Add `cargo publish` step with `CARGO_REGISTRY_TOKEN`
3. Add cosign keyless signing via `sigstore/cosign-installer`
4. Add SLSA provenance via `slsa-framework/slsa-github-generator`
5. Generate SBOM with `cargo-sbom` or `cyclonedx-bom`
6. Attach all attestation files to GitHub Release

Blog post and community announcements are **non-code tasks** that should be tracked separately (not as SDD tasks).

### Ready for Proposal

**Yes** — sufficient information to create a change proposal. The scope is well-defined: enhance release CI with signing/attestation/publishing, fix Cargo metadata, and prepare for v1.0.0 GA launch.
