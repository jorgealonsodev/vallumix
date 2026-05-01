# Debian and Ubuntu

This document details the installation of Vallumix on Debian 12 (Bookworm), Ubuntu 22.04 LTS (Jammy), and Ubuntu 24.04 LTS (Noble) using the official `.deb` package.

## Prerequisites

- Operating system: Debian 12, Ubuntu 22.04, or 24.04.
- Architecture: `amd64` or `arm64`.
- Root or sudo access.
- `wget` or `curl` installed.

## Installing the `.deb` Package

### 1. Download the Package

Visit the releases page of the repository and download the `.deb` file corresponding to your architecture:

```bash
# For amd64
wget https://github.com/tu-org/vallumix/releases/download/v1.0.0/vallumix_1.0.0_amd64.deb

# For arm64
wget https://github.com/tu-org/vallumix/releases/download/v1.0.0/vallumix_1.0.0_arm64.deb
```

### 2. Install with dpkg

```bash
sudo dpkg -i vallumix_1.0.0_*.deb
```

If `dpkg` reports unsatisfied dependencies, fix them with:

```bash
sudo apt-get install -f
```

```note
Vallumix is compiled as a static binary with musl. In practice, the `.deb` package has no runtime dependencies. The `apt-get install -f` command would only resolve metadata package dependencies if any existed.
```

### 3. Verify the Installation

```bash
vallumix --version
which vallumix
```

The binary is installed in `/usr/bin/vallumix`.

## Installing from APT Repository (Optional)

If you prefer to manage Vallumix via `apt` instead of manually downloading `.deb` files:

```bash
# Add the GPG key
curl -fsSL https://vallumix.dev/apt/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/vallumix.gpg

# Add the repository
echo "deb [signed-by=/usr/share/keyrings/vallumix.gpg] https://vallumix.dev/apt stable main" | \
  sudo tee /etc/apt/sources.list.d/vallumix.list

# Install
sudo apt update
sudo apt install vallumix
```

## Uninstallation

```bash
sudo dpkg -r vallumix
```

This removes the binary but **preserves backups** in `/var/backups/vallumix`. If you also want to remove the backups:

```bash
sudo rm -rf /var/backups/vallumix
```

```danger
Do not delete `/var/backups/vallumix` if you might need to revert previously applied changes. The backups are your only rollback path.
```
