#!/bin/bash
set -euo pipefail

PROFILE="${VALLUMIX_PROFILE:-web}"
DRY_RUN="${VALLUMIX_DRY_RUN:-0}"
DISTRO="debian12"

# Idempotent: install dependencies
export DEBIAN_FRONTEND=noninteractive
apt-get update || true
apt-get install -y curl ca-certificates build-essential git pkg-config libssl-dev || true

# Idempotent: install rustup if not present
if ! command -v rustc &>/dev/null; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.75
fi
source "$HOME/.cargo/env"

# Verify Rust
rustc --version
cargo --version

# Build vallumix
cd /vagrant
cargo build --release

# Run baseline audit
cd /vagrant
if [ "$DRY_RUN" = "1" ]; then
  ./target/release/vallumix audit --profile "$PROFILE" --report json --dry-run > "/vagrant/baseline-${DISTRO}.json" || true
else
  ./target/release/vallumix audit --profile "$PROFILE" --report json > "/vagrant/baseline-${DISTRO}.json" || true
fi

echo "Provision complete for ${DISTRO}"
