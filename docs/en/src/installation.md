# Installation

Vallumix can be installed through multiple methods depending on your platform and preferences.

## From crates.io

The simplest way to install Vallumix is via cargo:

```bash
cargo install vallumix-cli
```

This will download, compile, and install the `vallumix` binary into your cargo bin directory (usually `~/.cargo/bin`).

## From Source

To build from source, you need Rust 1.75 or later:

```bash
git clone https://github.com/vallumix/vallumix.git
cd vallumix
cargo build --release -p vallumix-cli
```

The compiled binary will be available at `target/release/vallumix`.

## Debian / Ubuntu (.deb)

Download the latest `.deb` package from the [GitHub Releases](https://github.com/vallumix/vallumix/releases) page:

```bash
wget https://github.com/vallumix/vallumix/releases/download/v0.0.1/vallumix_0.0.1_amd64.deb
sudo dpkg -i vallumix_0.0.1_amd64.deb
```

## RHEL / Rocky / AlmaLinux (.rpm)

Download the latest `.rpm` package from the [GitHub Releases](https://github.com/vallumix/vallumix/releases) page:

```bash
wget https://github.com/vallumix/vallumix/releases/download/v0.0.1/vallumix-0.0.1-1.x86_64.rpm
sudo rpm -i vallumix-0.0.1-1.x86_64.rpm
```

## Shell Completions

After installation, you can generate shell completions for bash, zsh, fish, or nushell:

```bash
vallumix completion bash > /etc/bash_completion.d/vallumix
vallumix completion zsh > /usr/share/zsh/vendor-completions/_vallumix
vallumix completion fish > /usr/share/fish/vendor_completions.d/vallumix.fish
```
