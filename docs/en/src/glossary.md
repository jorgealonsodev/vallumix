# Glossary

This glossary defines the technical terms used throughout the Vallumix documentation, based on the PRD and standard industry cybersecurity terminology.

## Terms

### Hardening
Process of reducing a system's attack surface by disabling unnecessary functionality and securely configuring the remaining features. In Vallumix, hardening is automated by applying CIS controls.

### CIS Benchmark
Secure configuration guides maintained by the Center for Internet Security (CIS). They are widely recognized industry references that define secure configurations for operating systems, applications, and network devices.

### Idempotency
Property by which executing an operation once or multiple times produces the same final result. Vallumix guarantees idempotency through pre-checks: if a control already complies, nothing is modified.

### Bastion (jump host)
Server exposed to the internet whose sole purpose is to serve as an authenticated entry point to internal infrastructure. Vallumix's `bastion` profile applies the most aggressive hardening, oriented exclusively to SSH.

### Dry-run
Execution mode that simulates the effects of an operation without actually performing them. In Vallumix, `--dry-run` executes all checks and reports what changes would be made, but does not create backups or modify files.

### MSRV (Minimum Supported Rust Version)
Minimum version of the Rust compiler that the project commits to support. For Vallumix, the MSRV is 1.75.

### Crate
Compilation and distribution unit in Rust, conceptually equivalent to a package. Vallumix is organized as a Cargo workspace with multiple crates: `vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup`, and `vallumix-cli`.

### Workspace
Grouping of multiple related crates jointly managed by Cargo. The Vallumix workspace allows compiling and testing all crates from the project root.

### Trait
Rust mechanism for defining shared behavior, similar to interfaces in other languages. The `Control` trait abstracts the lifecycle of each CIS verification in Vallumix.

### SLSA (Supply-chain Levels for Software Artifacts)
Google's framework for securing the software supply chain. Vallumix publishes SLSA Level 3 attestations in its releases to guarantee binary provenance and integrity.

### Control
Atomic unit of verification and remediation in Vallumix. Each control implements the `Control` trait and represents a specific CIS Benchmark recommendation (for example, "disable root login via SSH").

### Profile
Preconfigured set of controls adapted to a server's role. Vallumix includes three profiles: `web`, `database`, and `bastion`.

### Rollback
Reversal operation that restores previous configurations from versioned backups. Vallumix allows rollback by individual control or by complete session.

### Versioned Backups
Security copies organized by execution session, identified by timestamp. Each session includes the modified file, a metadata manifest, and SHA-256 checksums.

### Threshold
Configurable minimum compliance percentage. If a execution's compliance rate is below the threshold, Vallumix returns exit code `1`.

### Reporter
Trait that abstracts report generation in different formats. Vallumix implements reporters for HTML, JSON, JUnit XML, and plain text.

### Attack Surface
Set of entry points that an attacker could exploit in a system. The goal of hardening is to reduce this surface to the minimum necessary for operational functionality.

```tip
If you find a term in the documentation that is not in this glossary, open an issue to request its inclusion. Documentation should be accessible both for experienced administrators and for those starting out in system hardening.
```
