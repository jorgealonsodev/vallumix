#!/usr/bin/env bash
#
# End-to-end smoke test: install the built .deb into a clean container and
# drive the real binary.
#
# The unit suite passes with a binary that cannot run its own main command,
# because it never assembles the product: packaging, profile installation and
# the wiring in main.rs are all outside its reach. Every defect this script
# checks for shipped in both v1.0.0 and v1.1.0 with a fully green CI.
#
# Usage: scripts/smoke-test.sh [path/to/package.deb]
# Requires: docker, and a .deb built by `cargo deb`.

set -euo pipefail

IMAGE="${SMOKE_IMAGE:-ubuntu:24.04}"
CONTAINER="vallumix-smoke-$$"
TARGET="${SMOKE_TARGET:-x86_64-unknown-linux-musl}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

DEB="${1:-}"
if [ -z "$DEB" ]; then
    DEB="$(find "$REPO_ROOT/target/$TARGET/debian" -name '*.deb' -print -quit 2>/dev/null || true)"
fi
if [ ! -f "$DEB" ]; then
    echo "FAIL: no .deb found. Build one first:"
    echo "  cargo build --release --target $TARGET -p vallumix-cli"
    echo "  cargo deb --no-build --target $TARGET -p vallumix-cli"
    exit 1
fi

# shellcheck disable=SC2329  # invoked by the EXIT trap below
cleanup() { docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; }
trap cleanup EXIT

failures=0
pass() { echo "  PASS  $1"; }
fail() { echo "  FAIL  $1"; echo "        $2"; failures=$((failures + 1)); }

echo "Smoke test: $(basename "$DEB") on $IMAGE"
docker run -d --name "$CONTAINER" "$IMAGE" sleep 600 >/dev/null
docker cp "$DEB" "$CONTAINER:/tmp/vallumix.deb" >/dev/null

# ---------------------------------------------------------------------------
# 1. The package installs and puts the binary on PATH.
# ---------------------------------------------------------------------------
if docker exec "$CONTAINER" dpkg -i /tmp/vallumix.deb >/dev/null 2>&1; then
    pass "package installs"
else
    fail "package installs" "dpkg -i returned non-zero"
fi

if docker exec "$CONTAINER" test -x /usr/bin/vallumix; then
    pass "binary installed at /usr/bin/vallumix"
else
    fail "binary installed at /usr/bin/vallumix" "not found or not executable"
    echo "smoke test aborted: $failures failure(s)"
    exit 1
fi

# ---------------------------------------------------------------------------
# 2. The package ships the profiles the binary needs.
#    audit/apply read /etc/vallumix/profiles/<name>.toml. A package that omits
#    them installs a tool that cannot run.
# ---------------------------------------------------------------------------
for p in web database bastion; do
    if docker exec "$CONTAINER" test -f "/etc/vallumix/profiles/$p.toml"; then
        pass "profile installed: $p.toml"
    else
        fail "profile installed: $p.toml" "missing from the package"
    fi
done

# ---------------------------------------------------------------------------
# 3. `audit` runs, and says something.
#    Exit 0 means compliant, 1 means below threshold; both are real answers.
#    Exit 2 is the catch-all for a discarded error.
# ---------------------------------------------------------------------------
set +e
out="$(docker exec "$CONTAINER" /usr/bin/vallumix audit 2>&1)"
code=$?
set -e

if [ "$code" -eq 0 ] || [ "$code" -eq 1 ]; then
    pass "audit exits with a verdict (code $code)"
else
    fail "audit exits with a verdict" "exit $code — a swallowed error, not a compliance result"
fi

if [ -n "$out" ]; then
    pass "audit produces output"
else
    fail "audit produces output" "no stdout and no stderr; nothing for the user to act on"
fi

# ---------------------------------------------------------------------------
# 4. Failures are diagnosable.
#    Point the binary at a profile directory that does not exist and require it
#    to say so. Silence plus an exit code is not an error message.
# ---------------------------------------------------------------------------
set +e
err="$(docker exec -e VALLUMIX_PROFILE_DIR=/nonexistent "$CONTAINER" \
    /usr/bin/vallumix audit 2>&1)"
set -e
if [ -n "$err" ]; then
    pass "a failing run explains itself"
else
    fail "a failing run explains itself" "missing profile dir produced no message at all"
fi

# ---------------------------------------------------------------------------
# 5. The report names the distribution it actually ran on.
#    Controls are filtered by applicable_distros(), so a wrong distro silently
#    runs the wrong control set and reports a compliance rate for a system that
#    was never assessed.
# ---------------------------------------------------------------------------
actual_id="$(docker exec "$CONTAINER" sh -c '. /etc/os-release && echo "$ID"')"
report="$(docker exec "$CONTAINER" /usr/bin/vallumix audit --report text 2>&1 || true)"
header="$(printf '%s\n' "$report" | head -1)"

if printf '%s' "$header" | grep -qi "$actual_id"; then
    pass "report names the running distro ($actual_id)"
else
    fail "report names the running distro ($actual_id)" "header says: $header"
fi

echo
if [ "$failures" -eq 0 ]; then
    echo "smoke test passed"
else
    echo "smoke test: $failures failure(s)"
fi
exit $((failures > 0))
