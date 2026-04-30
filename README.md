# Vallumix

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

**Modular Linux hardening engine with CIS-aligned profiles, written in Rust.**

Vallumix automates server hardening by applying controls aligned with CIS Benchmarks. Select a profile based on your server's role — web, database, or bastion — and generate detailed compliance reports.

## Quick Start

```bash
# Build the workspace
cargo build --workspace

# Run tests
cargo test --workspace

# Run the CLI
cargo run -p vallumix-cli
```

## Profiles

- **web** — Hardening for HTTP/HTTPS servers (Nginx, Apache).
- **database** — Hardening for database servers (PostgreSQL, MariaDB, MongoDB).
- **bastion** — Hardening for jump hosts / SSH bastions.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for build, test, and submission guidelines.

## License

This project is licensed under either of:

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
