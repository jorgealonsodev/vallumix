# cosign-slsa-signing Specification

## Purpose

Establish cosign keyless binary signing and SLSA Level 3 provenance attestation for all release artifacts, ensuring verifiable supply chain integrity per PRD §6.2.

## Requirements

### Requirement: Keyless Binary Signing with Cosign

The release workflow MUST sign all binary artifacts using cosign keyless signing via GitHub OIDC. Signing MUST use `sigstore/cosign-installer` and the `id-token: write` permission.

#### Scenario: Release binaries are signed

- GIVEN a version tag is pushed and release builds succeed
- WHEN the cosign signing step runs
- THEN each binary is signed with cosign using GitHub OIDC and the signature is uploaded to the GitHub Release

#### Scenario: Signing fails without OIDC permissions

- GIVEN the release workflow lacks `id-token: write` permission
- WHEN the cosign signing step executes
- THEN the step fails with a clear error about missing OIDC token

### Requirement: SLSA Level 3 Provenance Generation

The release workflow MUST generate SLSA Level 3 provenance using `slsa-framework/slsa-github-generator` and attach the provenance attestation to the GitHub Release.

#### Scenario: Provenance attestation is generated

- GIVEN a release build completes successfully
- WHEN the SLSA generator runs
- THEN a `.intoto.jsonl` provenance attestation is produced and attached to the GitHub Release

#### Scenario: Provenance includes source and builder metadata

- GIVEN a release with tag v1.0.0
- WHEN the SLSA provenance is generated
- THEN the attestation includes the source commit SHA, builder identity, and build configuration

### Requirement: SBOM Generation

The release workflow MUST generate a CycloneDX Software Bill of Materials for each binary using `cargo-cyclonedx` and attach SBOMs to the GitHub Release.

#### Scenario: SBOM generated and attached to release

- GIVEN the release build step completes
- WHEN the SBOM generation step runs
- THEN a CycloneDX XML file is produced per binary and uploaded as a release asset

### Requirement: Attestation Upload to Release

All signing artifacts — cosign signatures, SLSA provenance, and SBOMs — MUST be attached to the GitHub Release alongside binary artifacts.

#### Scenario: All attestations available on release page

- GIVEN a release is published on GitHub
- WHEN a user views the release page
- THEN cosign signatures, `.intoto.jsonl` provenance, and `.xml` SBOMs are all downloadable assets