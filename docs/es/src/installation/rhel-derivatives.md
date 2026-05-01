# RHEL y Derivados

Esta guía cubre la instalación de Vallumix en Red Hat Enterprise Linux 9, Rocky Linux 9 y AlmaLinux 9 mediante el paquete `.rpm` oficial.

## Requisitos previos

- Sistema operativo: RHEL 9, Rocky Linux 9 o AlmaLinux 9.
- Arquitectura: `x86_64` o `aarch64`.
- Acceso root o sudo.
- `wget` o `curl` instalado.

## Instalación del paquete `.rpm`

### 1. Descarga el paquete

```bash
# Para x86_64
wget https://github.com/tu-org/vallumix/releases/download/v1.0.0/vallumix-1.0.0-1.x86_64.rpm

# Para aarch64
wget https://github.com/tu-org/vallumix/releases/download/v1.0.0/vallumix-1.0.0-1.aarch64.rpm
```

### 2. Instala con dnf

```bash
sudo dnf install ./vallumix-1.0.0-1.*.rpm
```

`dnf` resuelve automáticamente cualquier dependencia del paquete. Como Vallumix es un binario estático compilado con musl, no requiere bibliotecas adicionales en runtime.

### 3. Verifica la instalación

```bash
vallumix --version
which vallumix
```

El binario se instala en `/usr/bin/vallumix`.

## Instalación desde repositorio DNF (opcional)

Para gestionar actualizaciones automáticas mediante `dnf`:

```bash
# Añade el repositorio
sudo tee /etc/yum.repos.d/vallumix.repo <<EOF
[vallumix]
name=Vallumix Repository
baseurl=https://vallumix.dev/rpm/\$basearch
enabled=1
gpgcheck=1
gpgkey=https://vallumix.dev/rpm/gpg.key
EOF

# Instala
sudo dnf install vallumix
```

## Desinstalación

```bash
sudo dnf remove vallumix
```

Al igual que con el paquete `.deb`, la desinstalación del `.rpm` conserva los respaldos en `/var/backups/vallumix`. Elimínalos manualmente solo si estás seguro de que no necesitarás rollback:

```bash
sudo rm -rf /var/backups/vallumix
```

```danger
Conserva los respaldos al menos hasta la próxima ventana de mantenimiento confirmada. Un rollback puede salvar horas de depuración si un control de hardening rompe un servicio crítico.
```
