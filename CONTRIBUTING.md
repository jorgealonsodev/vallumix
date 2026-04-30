# Contributing to Vallumix

Thank you for your interest in contributing! This document outlines the workflow for submitting changes.

## Getting Started

1. Fork the repository.
2. Clone your fork locally.
3. Ensure you have Rust 1.75 or newer installed.

## Building

```bash
cargo build --workspace
```

## Testing

```bash
cargo test --workspace
```

## Linting

```bash
cargo clippy --workspace -- -D warnings
```

## Formatting

```bash
cargo fmt --all
```

Before submitting a PR, run:

```bash
cargo build --workspace && cargo test --workspace && cargo clippy --workspace -- -D warnings && cargo fmt --all -- --check
```

## Submitting Changes

1. Create a feature branch from `develop`.
2. Make your changes with clear, focused commits.
3. Open a pull request using the provided template.
4. Ensure CI passes on all supported distributions.

## Code of Conduct

Be respectful and constructive in all interactions.
