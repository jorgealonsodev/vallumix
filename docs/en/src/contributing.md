# Contributing

Thank you for your interest in contributing to Vallumix! This document provides guidelines for getting started.

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/vallumix/vallumix.git
   cd vallumix
   ```

2. Build the workspace:
   ```bash
   cargo build --workspace
   ```

3. Run tests:
   ```bash
   cargo test --workspace
   ```

4. Run clippy:
   ```bash
   cargo clippy --workspace -- -D warnings
   ```

## Code Style

- Follow the Rust API Guidelines.
- Run `cargo fmt` before committing.
- All public items must have documentation comments.
- Use meaningful variable and function names.

## Adding New Controls

1. Implement the `Control` trait in `crates/vallumix-controls/src/`.
2. Add the control to the appropriate profile TOML file.
3. Write unit tests for the control logic.
4. Update the CIS control mapping documentation.

## Testing

- Write unit tests for pure functions and control logic.
- Use integration tests in `crates/vallumix-cli/tests/cli.rs` for CLI behavior.
- Snapshot tests using `insta` are encouraged for reporter output.

## Submitting Changes

1. Create a feature branch from `develop`.
2. Make your changes with clear commit messages.
3. Ensure all tests pass.
4. Open a pull request against the `develop` branch.

## Reporting Issues

Please use the GitHub issue tracker to report bugs or request features. Include:
- Your operating system and version
- The Vallumix version
- Steps to reproduce
- Expected and actual behavior
