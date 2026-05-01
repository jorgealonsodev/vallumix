# API Reference

Vallumix is organized as a Cargo workspace with multiple crates. The public API is documented using rustdoc.

## Generating API Documentation

You can generate the API documentation locally:

```bash
cargo doc --no-deps --workspace
```

Then open `target/doc/vallumix_cli/index.html` in your browser.

## Crate Overview

### `vallumix-core`

Defines the core traits and types:
- `Control`: Trait for implementing CIS controls
- `Reporter`: Trait for report generation
- `Profile`: Trait for profile definitions
- `VallumixError`: Main error type

### `vallumix-controls`

Contains the concrete implementations of CIS controls organized by category:
- SSH hardening
- User management
- Network configuration
- Filesystem permissions

### `vallumix-reporters`

Report generators for different output formats:
- `HtmlReporter`
- `JsonReporter`
- `JunitReporter`
- `TextReporter`

### `vallumix-backup`

Backup and rollback functionality:
- `BackupManager`: Manages backup sessions
- `BackupSession`: Represents a single hardening session

### `vallumix-cli`

The command-line interface and main binary.

## Online Documentation

Hosted API documentation is available at:
[https://docs.rs/vallumix-cli](https://docs.rs/vallumix-cli)
