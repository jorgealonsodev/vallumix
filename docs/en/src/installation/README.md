# Installation

Vallumix is distributed as a single binary with no runtime dependencies. You can install it using three methods, depending on your environment and needs.

## Installation Methods

| Method | Ideal For | Requires Privileges | Estimated Time |
|---|---|---|---|
| **`.deb` package** | Debian 12, Ubuntu 22.04/24.04 | Root | < 1 min |
| **`.rpm` package** | RHEL 9, Rocky, AlmaLinux | Root | < 1 min |
| **From source** | Developers, unsupported architectures, patches | Root (only to install) | 5-10 min |

## Quick Decision

```
Using Debian or Ubuntu?     →  .deb  (see next page)
Using RHEL, Rocky or Alma?  →  .rpm  (see next page)
Need to modify code or compile for ARM64?  →  From source
```

## Common Prerequisites

Regardless of the method:

- Root access or ability to run `sudo`.
- Supported operating system (consult the compatibility section).
- Free disk space: at least 50 MB for the binary and initial backups.

```warning
Vallumix must run as root to apply hardening controls. However, installing the binary itself can be done as a normal user if you place it in `~/.local/bin`. Only the `apply`, `audit`, and `rollback` operations require privilege elevation.
```

## Post-Installation Verification

After installing Vallumix, verify that it works correctly:

```bash
vallumix --version
vallumix list --profile web
```

If both commands return output without errors, the installation is correct and you can continue with the [getting started guide](getting-started.md).
