# Introduction

<p align="center">
  <img src="/vallumix/img/valumix.png" alt="Vallumix" width="640">
</p>

Vallumix is a modular hardening engine for Linux systems, written in Rust and aligned with the Center for Internet Security (CIS) benchmarks. It automates the application of security controls on servers, eliminating the inconsistency of manual hardening and generating traceable compliance reports for audits.

## What Does Vallumix Do?

At its core, Vallumix evaluates the security posture of a Linux server and, when instructed, applies automatic remediations based on the CIS Benchmark. Each control — from disabling unnecessary filesystems to hardening SSH configuration — is implemented as an independent, testable, and reversible unit. The result is a tool that transforms hardening from hours of manual work into a reproducible, documented process that takes minutes.

### Key Capabilities

- **70 CIS controls** distributed across six categories: initial setup, services, network, logging and auditing, access and authentication, and system maintenance.
- **Three preconfigured profiles** that adapt control selection to the server's role: web, database, or bastion.
- **Five report formats**: self-contained HTML, parseable JSON, JUnit XML for pipelines, plain text with colors, and structured JSON Lines output for SIEM integration.
- **Dry-run mode** that previews all changes without modifying anything, ideal for pre-production validation.
- **Granular rollback** with versioned backups in `/var/backups/vallumix`, allowing you to revert a specific control or an entire session.
- **Single static binary** compiled with `musl`, with no runtime dependencies: copy it, run it, it works.

## Who Is It For?

Vallumix is designed for three professional profiles:

- **System administrators** at SMEs who need to harden servers quickly and demonstrate compliance during ISO 27001 or PCI-DSS audits without investing in commercial tools.
- **DevOps and DevSecOps engineers** who integrate security into CI/CD pipelines with Terraform, Packer, or Ansible, and require parseable artifacts and coherent exit codes.
- **Freelance security consultants** who audit clients with a portable, single-binary tool that generates presentable reports and enables controlled remediation.

```tip
Is this your first time with Vallumix? Go directly to the [installation guide](installation/README.md) and then run `vallumix audit --profile web --report html` to see how it works without modifying anything. Audit mode is the safest way to familiarize yourself with the tool.
```

## Why Rust

The choice of Rust is not accidental. A tool that runs as root and modifies critical system files must be intrinsically safe. Rust guarantees the absence of memory errors in safe code, enforces explicit error handling through `Result<T, E>`, and allows compiling a static binary that does not depend on Python versions, Bash interpreters, or system packages. This sets Vallumix apart from script-based solutions that fail silently or break dependencies across distributions.

## Project Status

Vallumix is an actively evolving open-source project. Version 1.0 supports Debian 12, Ubuntu 22.04/24.04 LTS, RHEL 9 and derivatives (Rocky Linux, AlmaLinux). Documentation is available in Spanish and English. If you encounter an issue or have a suggestion, consult the contribution guide in the repository.
