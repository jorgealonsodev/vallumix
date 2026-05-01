# Desde Código Fuente

Compilar Vallumix desde el código fuente te da control total sobre las optimizaciones, parches y arquitecturas objetivo. Este método es el recomendado para desarrolladores, mantenedores de distribuciones o entornos con requisitos de compilación cruzada.

## Requisitos previos

- Rust toolchain ≥ 1.75 (MSRV). Recomendado: latest stable via `rustup`.
- `cargo` y `rustc` funcionando.
- Para compilación cruzada: `cross` o `docker`.
- Sistema operativo Linux con acceso root (para ejecutar, no para compilar).

```note
Vallumix no requiere dependencias de sistema en runtime, pero necesita un toolchain Rust completo para compilar. No intentes compilarlo con el rustc de los repositorios de la distribución si está desactualizado; usa rustup.
```

## Compilación nativa

### 1. Clona el repositorio

```bash
git clone https://github.com/jorgealonsodev/vallumix.git
cd vallumix
```

### 2. Compila en modo release

```bash
cargo build --release
```

El binario resultante se encuentra en `target/release/vallumix`.

### 3. Instala en el sistema

```bash
sudo cp target/release/vallumix /usr/local/bin/
sudo chmod +x /usr/local/bin/vallumix
```

## Compilación estática con musl

Para producir un binario completamente estático sin dependencias de glibc:

```bash
# Añade el target musl
rustup target add x86_64-unknown-linux-musl

# Compila
cargo build --release --target x86_64-unknown-linux-musl
```

El binario estático se genera en `target/x86_64-unknown-linux-musl/release/vallumix`.

```tip
El binario musl es el que se distribuye en los releases oficiales. Es ligeramente mayor que el binario glibc pero garantiza compatibilidad universal entre distribuciones sin preocuparse por versiones de libc.
```

## Compilación cruzada

Para compilar desde tu máquina de desarrollo x86_64 hacia ARM64 (por ejemplo, para AWS Graviton o Raspberry Pi):

```bash
# Instala cross
cargo install cross --git https://github.com/cross-rs/cross

# Compila para ARM64
cross build --release --target aarch64-unknown-linux-musl
```

`cross` utiliza contenedores Docker con los toolchains necesarios, evitando que instales cadenas de compilación cruzada en tu sistema anfitrión.

## Verificación

Tras compilar, ejecuta los tests y verifica el binario:

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
./target/release/vallumix --version
```

Si `cargo test` pasa y `clippy` no reporta warnings, tu compilación es válida.
