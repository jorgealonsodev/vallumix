# Contributing to Vallumix

Thank you for your interest in contributing to Vallumix! This document outlines everything you need to know to set up your environment, write code, and submit changes.

## Development Environment Setup

### Prerequisites

- **Rust toolchain** 1.75 or newer (install via [rustup](https://rustup.rs/))
- `cargo`, `rustfmt`, and `clippy` (included with rustup)
- Optional but recommended: `cargo-tarpaulin`, `cargo-release`

### Clone and Build

```bash
git clone https://github.com/jorgealonsodev/vallumix.git
cd vallumix
cargo build --workspace
```

Verify everything works:

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## Adding a New CIS Control

Controls live in `crates/vallumix-controls/src/`. Each control is a module that implements the `Control` trait from `vallumix-core`.

### Module Template

Create a new file at `crates/vallumix-controls/src/cis_5_2_4.rs` (adapt the name to your control ID):

```rust
use vallumix_core::{Control, Context, CheckResult, ApplyResult, ControlError, Severity};

pub struct SshProtocolVersion;

impl Control for SshProtocolVersion {
    fn id(&self) -> &str {
        "5.2.4"
    }

    fn description(&self) -> &str {
        "Ensure SSH Protocol is set to 2"
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

    fn applicable_distros(&self) -> &[vallumix_core::Distro] {
        use vallumix_core::Distro::*;
        &[Debian12, Ubuntu2204, Ubuntu2404, Rhel9, Rocky9, AlmaLinux9]
    }

    fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError> {
        // Read current state and return CheckResult::Pass or CheckResult::Fail
        todo!()
    }

    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
        // Apply the hardening change and return ApplyResult
        todo!()
    }

    fn rollback(&self, ctx: &Context, backup: &vallumix_core::Backup) -> Result<(), ControlError> {
        // Restore state from backup
        todo!()
    }
}
```

### Registration

Add your module to `crates/vallumix-controls/src/lib.rs` and include it in the registry function so the engine can discover it. Update `controls/README.md` with the mapping entry.

### Tests

Write integration tests in `crates/vallumix-controls/tests/` using temporary directories and mock contexts. Ensure your control can be checked, applied, and rolled back in isolation.

## Running Tests

```bash
# Unit and integration tests
cargo test --workspace

# Coverage report (requires cargo-tarpaulin)
cargo tarpaulin --workspace --out Html --out Lcov

# CLI integration tests (requires assert_cmd and predicates)
cargo test -p vallumix-cli --test integration

# Snapshot tests (requires insta)
cargo test --workspace --features snapshot
```

## Code Style

- **Formatting**: `rustfmt` (enforced in CI)
- **Linting**: `clippy` in pedantic mode with `-D warnings`
- **Error handling**: Use `Result<T, E>` with typed errors. **No `.unwrap()` or `.expect()` in production code.** Use `?` propagation or explicit `match` blocks.
- **Unsafe code**: Avoid `unsafe`. If absolutely necessary, document the invariants and seek explicit approval in the PR.
- **Documentation**: All public items must have rustdoc comments. Run `cargo doc --workspace --no-deps` to verify.

Before submitting a PR, run the full check suite:

```bash
cargo build --workspace \
  && cargo test --workspace \
  && cargo clippy --workspace -- -D warnings \
  && cargo fmt --all -- --check
```

## Pull Request Process

1. **Fork and branch**: Create a feature branch from `main`. Use a descriptive name: `feat/ssh-control`, `fix/rollback-cleanup`, `docs/api-examples`.
2. **Conventional Commits**: Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:
   - `feat:` new feature
   - `fix:` bug fix
   - `docs:` documentation only
   - `test:` adding or correcting tests
   - `refactor:` code change that neither fixes a bug nor adds a feature
   - `chore:` build process or auxiliary tool changes
3. **Fill the PR template**: Check every item in the checklist.
4. **Ensure CI passes**: All workflows (build, test, clippy, fmt, audit, deny) must be green.
5. **Code review**: Address review feedback promptly and respectfully.

## Release Workflow

Releases are managed with [cargo-release](https://github.com/crate-ci/cargo-release). To cut a new release:

```bash
cargo release <major|minor|patch> --execute
```

This will:

1. Bump versions across all workspace crates.
2. Generate the changelog entry.
3. Create a signed git tag.
4. Push the tag, triggering the release CI pipeline.

> **Do not** manually edit version numbers in `Cargo.toml` files. Always use `cargo-release` to ensure consistency.

## Getting Help

- Open a [discussion](https://github.com/jorgealonsodev/vallumix/discussions) for questions.
- Open an [issue](https://github.com/jorgealonsodev/vallumix/issues) for bugs or feature requests.
- Read the [docs](https://jorgealonsodev.github.io/vallumix/) for detailed guides.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.
