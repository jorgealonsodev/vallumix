# From Source

Compiling Vallumix from source gives you full control over optimizations, patches, and target architectures. This method is recommended for developers, distribution maintainers, or environments with cross-compilation requirements.

## Prerequisites

- Rust toolchain ≥ 1.75 (MSRV). Recommended: latest stable via `rustup`.
- Working `cargo` and `rustc`.
- For cross-compilation: `cross` or `docker`.
- Linux operating system with root access (to run, not to compile).

```note
Vallumix requires no system dependencies at runtime, but needs a complete Rust toolchain to compile. Do not attempt to compile it with the distribution's rustc if it is outdated; use rustup.
```

## Native Compilation

### 1. Clone the Repository

```bash
git clone https://github.com/tu-org/vallumix.git
cd vallumix
```

### 2. Compile in Release Mode

```bash
cargo build --release
```

The resulting binary is located at `target/release/vallumix`.

### 3. Install on the System

```bash
sudo cp target/release/vallumix /usr/local/bin/
sudo chmod +x /usr/local/bin/vallumix
```

## Static Compilation with musl

To produce a fully static binary with no glibc dependencies:

```bash
# Add the musl target
rustup target add x86_64-unknown-linux-musl

# Compile
cargo build --release --target x86_64-unknown-linux-musl
```

The static binary is generated at `target/x86_64-unknown-linux-musl/release/vallumix`.

```tip
The musl binary is the one distributed in official releases. It is slightly larger than the glibc binary but guarantees universal compatibility across distributions without worrying about libc versions.
```

## Cross Compilation

To compile from your x86_64 development machine to ARM64 (for example, for AWS Graviton or Raspberry Pi):

```bash
# Install cross
cargo install cross --git https://github.com/cross-rs/cross

# Compile for ARM64
cross build --release --target aarch64-unknown-linux-musl
```

`cross` uses Docker containers with the necessary toolchains, avoiding the need to install cross-compilation chains on your host system.

## Verification

After compiling, run the tests and verify the binary:

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
./target/release/vallumix --version
```

If `cargo test` passes and `clippy` reports no warnings, your compilation is valid.
