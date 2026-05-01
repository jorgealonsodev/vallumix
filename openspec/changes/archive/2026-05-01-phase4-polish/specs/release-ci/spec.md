# Delta for ci-pipeline — Release CI

## ADDED Requirements

### Requirement: Release Workflow on Version Tags

A `.github/workflows/release.yml` workflow MUST trigger on pushes matching `v*` tags. It MUST build release binaries for `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`, generate `.deb` and `.rpm` packages, produce SHA256 checksums, and create a GitHub Release with all artifacts attached.

#### Scenario: Tag push triggers release build

- GIVEN a tag `v0.9.0` is pushed
- WHEN the release workflow triggers
- THEN two musl binaries are built: `vallumix-x86_64-musl` and `vallumix-aarch64-musl`

#### Scenario: Release artifacts include packages

- GIVEN the release workflow completes successfully
- WHEN the GitHub Release page is viewed
- THEN it contains tarballs for each architecture, `.deb`, `.rpm`, and `SHA256SUMS` file

#### Scenario: Release workflow runs independently of CI

- GIVEN the release workflow is triggered by a tag
- WHEN the main CI workflow is also running on `develop`
- THEN both workflows complete independently without conflicts

### Requirement: Release Binary Strip and Optimize

Release binaries MUST be stripped (`strip`) and compressed with UPX to meet the PRD §6.3 size target of <8 MB.

#### Scenario: Stripped binary meets size target

- GIVEN release binaries are built and stripped
- WHEN `ls -lh` is checked
- THEN each binary is <8 MB

### Requirement: Cross-Compilation via Cross

The release workflow MUST use the `cross` tool for aarch64 builds, matching the CI cross-build pattern already established in the existing CI workflow.

#### Scenario: aarch64 binary built via cross

- GIVEN the release workflow runs the aarch64 build step
- WHEN `cross build --target aarch64-unknown-linux-musl --release` executes
- THEN an aarch64 static binary is produced

### Requirement: Changelog Included in Release

The release workflow MUST include the contents of `CHANGELOG.md` in the GitHub Release body, extracting the relevant version section.

#### Scenario: Release notes include changelog

- GIVEN a `v0.9.0` tag is pushed and `CHANGELOG.md` has a `[0.9.0]` section
- WHEN the release is created on GitHub
- THEN the release body contains the changelog entry for that version