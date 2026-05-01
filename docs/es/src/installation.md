# Instalación

Vallumix puede instalarse a través de múltiples métodos dependiendo de tu plataforma y preferencias.

## Desde crates.io

La forma más sencilla de instalar Vallumix es mediante cargo:

```bash
cargo install vallumix-cli
```

Esto descargará, compilará e instalará el binario `vallumix` en tu directorio de binarios de cargo (generalmente `~/.cargo/bin`).

## Desde el Código Fuente

Para compilar desde el código fuente, necesitas Rust 1.75 o posterior:

```bash
git clone https://github.com/vallumix/vallumix.git
cd vallumix
cargo build --release -p vallumix-cli
```

El binario compilado estará disponible en `target/release/vallumix`.

## Debian / Ubuntu (.deb)

Descarga el último paquete `.deb` desde la página de [GitHub Releases](https://github.com/vallumix/vallumix/releases):

```bash
wget https://github.com/vallumix/vallumix/releases/download/v0.0.1/vallumix_0.0.1_amd64.deb
sudo dpkg -i vallumix_0.0.1_amd64.deb
```

## RHEL / Rocky / AlmaLinux (.rpm)

Descarga el último paquete `.rpm` desde la página de [GitHub Releases](https://github.com/vallumix/vallumix/releases):

```bash
wget https://github.com/vallumix/vallumix/releases/download/v0.0.1/vallumix-0.0.1-1.x86_64.rpm
sudo rpm -i vallumix-0.0.1-1.x86_64.rpm
```

## Completado de Shell

Después de la instalación, puedes generar completados de shell para bash, zsh, fish o nushell:

```bash
vallumix completion bash > /etc/bash_completion.d/vallumix
vallumix completion zsh > /usr/share/zsh/vendor-completions/_vallumix
vallumix completion fish > /usr/share/fish/vendor_completions.d/vallumix.fish
```
