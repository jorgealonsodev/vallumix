#!/usr/bin/env bash
# TDD Verification Script for phase5-launch
# Run this to verify all 17 tasks are correctly implemented

set -euo pipefail

ERRORS=0
REPO_URL="https://github.com/jorgealonsodev/vallumix"
VERSION="1.0.0"

echo "=== Phase 5 Launch Verification ==="
echo ""

# --- Phase 1: Repo Fix + Cargo Metadata ---

echo "--- Phase 1 ---"

# 1.1 Fix workspace repository URL
if grep -q "repository = \"$REPO_URL\"" Cargo.toml; then
    echo "[PASS] 1.1 repository URL correct"
else
    echo "[FAIL] 1.1 repository URL incorrect or missing"
    ERRORS=$((ERRORS + 1))
fi

# 1.2 Add workspace metadata
if grep -q '^description = ' Cargo.toml; then
    echo "[PASS] 1.2a workspace description present"
else
    echo "[FAIL] 1.2a workspace description missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'keywords = \["cis"' Cargo.toml; then
    echo "[PASS] 1.2b workspace keywords present"
else
    echo "[FAIL] 1.2b workspace keywords missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'categories = \["command-line-utilities"' Cargo.toml || grep -q 'categories = \["Command Line Tools"' Cargo.toml; then
    echo "[PASS] 1.2c workspace categories present"
else
    echo "[FAIL] 1.2c workspace categories missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '^homepage = ' Cargo.toml; then
    echo "[PASS] 1.2d workspace homepage present"
else
    echo "[FAIL] 1.2d workspace homepage missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '^documentation = ' Cargo.toml; then
    echo "[PASS] 1.2e workspace documentation present"
else
    echo "[FAIL] 1.2e workspace documentation missing"
    ERRORS=$((ERRORS + 1))
fi

# 1.3 Add publish = false to workspace
if grep -q '^publish = false' Cargo.toml; then
    echo "[PASS] 1.3 workspace publish = false present"
else
    echo "[FAIL] 1.3 workspace publish = false missing"
    ERRORS=$((ERRORS + 1))
fi

# 1.4 Add metadata to vallumix-cli Cargo.toml
CLI_TOML="crates/vallumix-cli/Cargo.toml"
if grep -q '^description = ' "$CLI_TOML"; then
    echo "[PASS] 1.4a cli description present"
else
    echo "[FAIL] 1.4a cli description missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '^keywords = ' "$CLI_TOML"; then
    echo "[PASS] 1.4b cli keywords present"
else
    echo "[FAIL] 1.4b cli keywords missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '^readme = "README.md"' "$CLI_TOML"; then
    echo "[PASS] 1.4c cli readme present"
else
    echo "[FAIL] 1.4c cli readme missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '^license = ' "$CLI_TOML"; then
    echo "[PASS] 1.4d cli license present"
else
    echo "[FAIL] 1.4d cli license missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '^categories = ' "$CLI_TOML"; then
    echo "[PASS] 1.4e cli categories present"
else
    echo "[FAIL] 1.4e cli categories missing"
    ERRORS=$((ERRORS + 1))
fi

# --- Phase 2: Version Bump ---

echo ""
echo "--- Phase 2 ---"

# 2.1 Bump version
if grep -q "version = \"$VERSION\"" Cargo.toml; then
    echo "[PASS] 2.1 workspace version bumped to $VERSION"
else
    echo "[FAIL] 2.1 workspace version not $VERSION"
    ERRORS=$((ERRORS + 1))
fi

# 2.2 CHANGELOG updated
if grep -q "^## \[$VERSION\]" CHANGELOG.md; then
    echo "[PASS] 2.2 CHANGELOG has [$VERSION] section"
else
    echo "[FAIL] 2.2 CHANGELOG missing [$VERSION] section"
    ERRORS=$((ERRORS + 1))
fi

if grep -q "^## \[Unreleased\]" CHANGELOG.md; then
    echo "[PASS] 2.2 CHANGELOG has [Unreleased] header"
else
    echo "[FAIL] 2.2 CHANGELOG missing [Unreleased] header"
    ERRORS=$((ERRORS + 1))
fi

# --- Phase 3: SLSA Provenance Workflow ---

echo ""
echo "--- Phase 3 ---"

SLSA_YML=".github/workflows/slsa-release.yml"
if [ -f "$SLSA_YML" ]; then
    echo "[PASS] 3.1 slsa-release.yml exists"
else
    echo "[FAIL] 3.1 slsa-release.yml missing"
    ERRORS=$((ERRORS + 1))
fi

if [ -f "$SLSA_YML" ] && grep -q 'slsa-github-generator' "$SLSA_YML"; then
    echo "[PASS] 3.2 slsa-github-generator referenced"
else
    echo "[FAIL] 3.2 slsa-github-generator not referenced"
    ERRORS=$((ERRORS + 1))
fi

if [ -f "$SLSA_YML" ] && grep -q 'tags:' "$SLSA_YML" && grep -q "v\*" "$SLSA_YML"; then
    echo "[PASS] 3.1b slsa-release.yml triggers on v* tags"
else
    echo "[FAIL] 3.1b slsa-release.yml tag trigger missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'slsa-github-generator' .github/workflows/release.yml; then
    echo "[PASS] 3.2 release.yml references slsa-github-generator"
else
    echo "[FAIL] 3.2 release.yml does not reference slsa-github-generator"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '.attestation' .github/workflows/release.yml || grep -q 'provenance' .github/workflows/release.yml; then
    echo "[PASS] 3.3 release.yml uploads provenance attestation"
else
    echo "[FAIL] 3.3 release.yml provenance attestation upload missing"
    ERRORS=$((ERRORS + 1))
fi

# --- Phase 4: Cosign Keyless Signing ---

echo ""
echo "--- Phase 4 ---"

if grep -q 'id-token: write' .github/workflows/release.yml; then
    echo "[PASS] 4.1 id-token: write permission present"
else
    echo "[FAIL] 4.1 id-token: write permission missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'cosign-installer' .github/workflows/release.yml; then
    echo "[PASS] 4.1 cosign-installer step present"
else
    echo "[FAIL] 4.1 cosign-installer step missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'cosign sign' .github/workflows/release.yml; then
    echo "[PASS] 4.2 cosign sign step present"
else
    echo "[FAIL] 4.2 cosign sign step missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q '.sig' .github/workflows/release.yml; then
    echo "[PASS] 4.3 .sig files attached to release"
else
    echo "[FAIL] 4.3 .sig files not attached to release"
    ERRORS=$((ERRORS + 1))
fi

# --- Phase 5: Crates.io Publish ---

echo ""
echo "--- Phase 5 ---"

if grep -q 'cargo publish' .github/workflows/release.yml; then
    echo "[PASS] 5.1 cargo publish step present"
else
    echo "[FAIL] 5.1 cargo publish step missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'CARGO_REGISTRY_TOKEN' .github/workflows/release.yml; then
    echo "[PASS] 5.2 CARGO_REGISTRY_TOKEN env present"
else
    echo "[FAIL] 5.2 CARGO_REGISTRY_TOKEN env missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'vallumix-cli' .github/workflows/release.yml && grep -q 'publish = false' crates/vallumix-core/Cargo.toml && grep -q 'publish = false' crates/vallumix-controls/Cargo.toml; then
    echo "[PASS] 5.3 only vallumix-cli published (lib crates have publish=false)"
else
    echo "[FAIL] 5.3 publish scope incorrect"
    ERRORS=$((ERRORS + 1))
fi

# --- Phase 6: SBOM Generation ---

echo ""
echo "--- Phase 6 ---"

if grep -q 'cargo-sbom' .github/workflows/release.yml || grep -q 'cargo sbom' .github/workflows/release.yml; then
    echo "[PASS] 6.1 cargo-sbom installation/generation present"
else
    echo "[FAIL] 6.1 cargo-sbom missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'cyclonedx' .github/workflows/release.yml || grep -iq 'sbom' .github/workflows/release.yml; then
    echo "[PASS] 6.2 SBOM generation present"
else
    echo "[FAIL] 6.2 SBOM generation missing"
    ERRORS=$((ERRORS + 1))
fi

if grep -q 'sbom' .github/workflows/release.yml && grep -q 'files:' .github/workflows/release.yml; then
    echo "[PASS] 6.2b SBOM attached as release asset"
else
    echo "[FAIL] 6.2b SBOM not attached as release asset"
    ERRORS=$((ERRORS + 1))
fi

echo ""
echo "=== Verification Complete ==="
echo "Errors: $ERRORS"
exit $ERRORS
