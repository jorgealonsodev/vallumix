# Solución de Problemas

Esta página recoge los problemas más frecuentes reportados por usuarios de Vallumix, junto con sus causas probables y soluciones.

## Permiso denegado (Permission denied)

**Síntoma:**

```text
Error: privilege check failed — effective UID is 1000, root required
```

**Causa:** Vallumix requiere privilegios de root para leer archivos de configuración protegidos (`/etc/shadow`, `/etc/ssh/sshd_config`) y para modificar configuraciones del sistema.

**Solución:**

```bash
sudo vallumix apply --profile web
```

Si ejecutas Vallumix dentro de un contenedor o máquina virtual sin acceso root, eleva los privilegios antes de ejecutarlo. No uses `chmod 777` como workaround en archivos de sistema.

## Distribución no soportada

**Síntoma:**

```text
Error: distribution "Fedora 40" is not supported
Supported: debian-12, ubuntu-22.04, ubuntu-24.04, rhel-9, rocky-9, almalinux-9
```

**Causa:** Vallumix detecta la distribución leyendo `/etc/os-release`. Si la distro no está en la lista de soportadas, aborta para evitar aplicar controles que podrían no ser compatibles.

**Solución:**

- Si es una derivada de Debian/RHEL que debería funcionar, fuerza la detección con la variable de entorno `VALLUMIX_DISTRO_OVERRIDE` (solo para pruebas, sin soporte garantizado).
- Si necesitas soporte oficial para una nueva distribución, abre un issue en el repositorio.
- Para contenedores o entornos no convencionales, evalúa si Vallumix es la herramienta adecuada (véase "Fuera de alcance" en el PRD).

## Bloqueo de acceso SSH tras hardening

**Síntoma:** Tras aplicar el perfil `bastion` o controles de SSH agresivos, no puedes conectarte por SSH.

**Causa:** El perfil bastión deshabilita `PasswordAuthentication` y `PermitRootLogin`. Si te conectabas por contraseña o como root, perdiste el acceso.

**Prevención:**

```bash
# ANTES de aplicar bastión, verifica acceso por clave
ssh -o PasswordAuthentication=no admin@servidor
```

**Recuperación:**

- Accede a la consola física o al panel de administración de la nube (VNC, serial console, EC2 Instance Connect).
- Restaura manualmente `/etc/ssh/sshd_config` desde el respaldo en `/var/backups/vallumix/<timestamp>/5.2.4/`.
- Reinicia SSH: `sudo systemctl restart sshd`.

```danger
El bloqueo SSH es el incidente más grave que puede causar una herramienta de hardening remota. Siempre verifica el acceso por clave antes de aplicar controles de SSH. En servidores cloud, confirma que tienes acceso a la consola serial o VNC como vía de recuperación.
```

## Espacio insuficiente en disco para respaldos

**Síntoma:**

```text
Error: insufficient disk space for backups — 45 MB available, 100 MB required
```

**Causa:** Vallumix verifica el espacio libre en disco antes de crear la sesión de respaldo. Si no hay suficiente espacio, aborta para evitar quedarse a mitad de camino sin respaldos.

**Solución:**

```bash
# Libera espacio en /var
df -h /var
sudo apt autoremove   # Debian/Ubuntu
sudo dnf autoremove   # RHEL/Rocky/Alma
sudo journalctl --vacuum-time=7d
```

Si `/var` está en una partición separada y pequeña, considera montar `/var/backups/vallumix` en un volumen mayor o configurar una ruta alternativa mediante variable de entorno (si el proyecto la soporta).

## Controles fallidos tras aplicación

**Síntoma:** Un control aparece como `Failed` en el reporte tras `apply`.

**Causas comunes:**

1. **Servicio bloquea el archivo:** un demonio mantiene abierto el archivo de configuración con bloqueo de escritura.
2. **Configuración inesperada:** el archivo tiene una sintaxis que el control no anticipa.
3. **Dependencia de otro control:** un control previo modificó algo que este control asumía constante.

**Solución:**

- Revisa el log con `--verbose` para ver el error exacto.
- Verifica si un servicio bloquea el archivo: `lsof /etc/ssh/sshd_config`.
- Aplica el control manualmente siguiendo la recomendación CIS, luego re-ejecuta Vallumix para confirmar.

## Reporte no generado

**Síntoma:** La ejecución termina pero no encuentras el archivo de reporte.

**Causa:** `--output` no especificó un directorio existente o los permisos impiden escribir.

**Solución:**

```bash
# Asegúrate de que el directorio existe y es escribible
mkdir -p /var/reports/vallumix
sudo vallumix audit --profile web --report html --output /var/reports/vallumix/audit
```

Si no usas `--output`, Vallumix usa `/tmp` por defecto. Algunos sistemas limpian `/tmp` en cada reinicio.
