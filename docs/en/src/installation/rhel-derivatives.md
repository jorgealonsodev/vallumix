# RHEL and Derivatives

This guide covers the installation of Vallumix on Red Hat Enterprise Linux 9, Rocky Linux 9, and AlmaLinux 9 using the official `.rpm` package.

## Prerequisites

- Operating system: RHEL 9, Rocky Linux 9, or AlmaLinux 9.
- Architecture: `x86_64` or `aarch64`.
- Root or sudo access.
- `wget` or `curl` installed.

## Installing the `.rpm` Package

### 1. Download the Package

```bash
# For x86_64
wget https://github.com/jorgealonsodev/vallumix/releases/download/v1.0.0/vallumix-1.0.0-1.x86_64.rpm

# For aarch64
wget https://github.com/jorgealonsodev/vallumix/releases/download/v1.0.0/vallumix-1.0.0-1.aarch64.rpm
```

### 2. Install with dnf

```bash
sudo dnf install ./vallumix-1.0.0-1.*.rpm
```

`dnf` automatically resolves any package dependencies. Since Vallumix is a static binary compiled with musl, it requires no additional libraries at runtime.

### 3. Verify the Installation

```bash
vallumix --version
which vallumix
```

The binary is installed in `/usr/bin/vallumix`.

## Installing from DNF Repository (Optional)

To manage automatic updates via `dnf`:

```bash
# Add the repository
sudo tee /etc/yum.repos.d/vallumix.repo <<EOF
[vallumix]
name=Vallumix Repository
baseurl=https://vallumix.dev/rpm/\$basearch
enabled=1
gpgcheck=1
gpgkey=https://vallumix.dev/rpm/gpg.key
EOF

# Install
sudo dnf install vallumix
```

## Uninstallation

```bash
sudo dnf remove vallumix
```

As with the `.deb` package, uninstalling the `.rpm` preserves backups in `/var/backups/vallumix`. Only remove them manually if you are certain you will not need rollback:

```bash
sudo rm -rf /var/backups/vallumix
```

```danger
Keep backups at least until the next confirmed maintenance window. A rollback can save hours of debugging if a hardening control breaks a critical service.
```
