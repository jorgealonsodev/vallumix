# Debian y Ubuntu

Este documento detalla la instalación de Vallumix en Debian 12 (Bookworm), Ubuntu 22.04 LTS (Jammy) y Ubuntu 24.04 LTS (Noble) mediante el paquete `.deb` oficial.

## Requisitos previos

- Sistema operativo: Debian 12, Ubuntu 22.04 o 24.04.
- Arquitectura: `amd64` o `arm64`.
- Acceso root o sudo.
- `wget` o `curl` instalado.

## Instalación del paquete `.deb`

### 1. Descarga el paquete

Visita la página de releases del repositorio y descarga el archivo `.deb` correspondiente a tu arquitectura:

```bash
# Para amd64
wget https://github.com/jorgealonsodev/vallumix/releases/download/v1.0.0/vallumix_1.0.0_amd64.deb

# Para arm64
wget https://github.com/jorgealonsodev/vallumix/releases/download/v1.0.0/vallumix_1.0.0_arm64.deb
```

### 2. Instala con dpkg

```bash
sudo dpkg -i vallumix_1.0.0_*.deb
```

Si `dpkg` reporta dependencias no satisfechas, corrígelas con:

```bash
sudo apt-get install -f
```

```note
Vallumix se compila como binario estático con musl. En la práctica, el paquete `.deb` no tiene dependencias de runtime. El comando `apt-get install -f` solo resolvería dependencias del paquete de metadatos si las hubiera.
```

### 3. Verifica la instalación

```bash
vallumix --version
which vallumix
```

El binario se instala en `/usr/bin/vallumix`.

## Instalación desde repositorio APT (opcional)

Si prefieres gestionar Vallumix mediante `apt` en lugar de descargar `.deb` manualmente:

```bash
# Añade la clave GPG
curl -fsSL https://vallumix.dev/apt/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/vallumix.gpg

# Añade el repositorio
echo "deb [signed-by=/usr/share/keyrings/vallumix.gpg] https://vallumix.dev/apt stable main" | \
  sudo tee /etc/apt/sources.list.d/vallumix.list

# Instala
sudo apt update
sudo apt install vallumix
```

## Desinstalación

```bash
sudo dpkg -r vallumix
```

Esto elimina el binario pero **conserva los respaldos** en `/var/backups/vallumix`. Si deseas eliminar también los respaldos:

```bash
sudo rm -rf /var/backups/vallumix
```

```danger
No elimines `/var/backups/vallumix` si podrías necesitar revertir cambios aplicados previamente. Los respaldos son tu única vía de rollback.
```
