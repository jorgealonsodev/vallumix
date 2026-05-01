# Packer Example — Vallumix Hardened VM

This directory contains a Packer template that builds a Debian 12 VM with Vallumix hardening applied automatically.

## Prerequisites

- [Packer](https://www.packer.io/) >= 1.9
- [QEMU](https://www.qemu.org/) / KVM
- Approximately 10 GB of free disk space
- Internet access to download the Debian ISO and Vallumix package

## Files

- `vallumix-hardened.pkr.hcl` — Packer template with QEMU builder
- `http/preseed.cfg` — Debian preseed configuration (optional, create if needed)

## Build

```bash
cd examples/packer
packer init vallumix-hardened.pkr.hcl
packer build vallumix-hardened.pkr.hcl
```

## Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `vallumix_version` | `0.0.1` | Vallumix version to install |
| `vallumix_deb_url` | GitHub Releases URL | URL to the Vallumix .deb package |

Override variables at build time:

```bash
packer build -var 'vallumix_version=0.1.0' vallumix-hardened.pkr.hcl
```

## Expected Output

After a successful build:
- A QCOW2 image in `output-debian-12/`
- An HTML compliance report generated during provisioning

## Links

- [Vallumix Documentation](../../docs/en/src/README.md)
- [Vallumix GitHub Releases](https://github.com/jorgealonsodev/vallumix/releases)
