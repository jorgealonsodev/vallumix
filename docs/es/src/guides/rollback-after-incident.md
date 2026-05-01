# Rollback tras Incidente (CU-04)

Esta guía describe cómo responder cuando un control de hardening aplica un cambio que rompe un servicio crítico. Cubre tanto el rollback de un control específico como el de una sesión completa.

## Escenario

- **Sistema:** Servidor web Ubuntu 24.04 con Nginx y PHP-FPM.
- **Acción:** Se ejecutó `vallumix apply --profile web` durante una ventana de mantenimiento.
- **Síntoma:** Tras la ejecución, los usuarios reportan errores 502 Bad Gateway. PHP-FPM no puede escribir en `/tmp` para gestionar sesiones.
- **Hipótesis:** Un control de filesystem montó `/tmp` con `noexec`, lo cual es correcto desde la perspectiva CIS pero rompe la funcionalidad de PHP-FPM que usa archivos de sesión en `/tmp`.

## Paso 1: Identificar el control responsable

El reporte de la ejecución es tu punto de partida. Localiza el archivo generado:

```bash
ls -lt /var/backups/vallumix/
# Identifica el timestamp de la sesión más reciente
# Ejemplo: 2026-05-01T09-15-33
```

Si aún tienes el reporte JSON de la ejecución:

```bash
jq '.controls[] | select(.status == "Remediated") | {id, description, severity}' /tmp/hardening-report.json
```

En este escenario, identificas que el control `1.1.2.2` (Ensure nodev option set on /tmp partition) y `1.1.2.3` (Ensure nosuid option set on /tmp partition) se aplicaron, y posiblemente `1.1.2.4` (Ensure noexec option set on /tmp partition) es el causante directo del problema.

## Paso 2: Verificar los respaldos

```bash
ls /var/backups/vallumix/2026-05-01T09-15-33/
# Debe contener: manifest.json, checksums.sha256, y subdirectorios por control

# Verifica integridad de los respaldos
cd /var/backups/vallumix/2026-05-01T09-15-33
sha256sum -c checksums.sha256
# Todos deben reportar OK
```

```danger
Si `sha256sum -c` reporta FAIL para algún archivo de respaldo, NO ejecutes rollback de ese control. La copia de seguridad puede estar corrupta y restaurarla podría dejar el sistema en un estado inconsistente. Contacta al soporte o restaura manualmente desde otro medio.
```

## Paso 3: Rollback del control específico

Si estás seguro de que solo `1.1.2.4` causó el problema, reviértelo individualmente:

```bash
sudo vallumix rollback --control-id 1.1.2.4
```

### Salida esperada

```text
[INFO] Restoring backup for control 1.1.2.4
[INFO] Source: /var/backups/vallumix/2026-05-01T09-15-33/1.1.2.4/fstab.bak
[INFO] Target: /etc/fstab
[INFO] Integrity check: SHA-256 OK
[INFO] Remounting /tmp with original options
[INFO] Post-check: /tmp is now compliant with pre-hardening state
[SUCCESS] Rollback of control 1.1.2.4 completed
```

## Paso 4: Verificar la restauración del servicio

```bash
# Verifica que /tmp tiene las opciones originales
findmnt /tmp
# Debe mostrar: /tmp tmpfs tmpfs rw,nosuid,nodev (sin noexec)

# Reinicia PHP-FPM para que recree sus archivos de sesión
sudo systemctl restart php8.3-fpm

# Verifica que Nginx responde correctamente
curl -I https://localhost/
# Debe devolver HTTP 200
```

## Paso 5: Alternativa — Rollback de sesión completa

Si no estás seguro de qué control causó el problema, o si varios controles de filesystem interactuaron mal, revierte toda la sesión:

```bash
# Rollback de la última sesión automáticamente
sudo vallumix rollback --session last

# O por timestamp específico
sudo vallumix rollback --session 2026-05-01T09-15-33
```

El rollback por sesión restaura todos los archivos en el orden inverso a como fueron modificados, minimizando dependencias circulares entre configuraciones.

```warning
El rollback de sesión completa revierte TODOS los controles aplicados, no solo el problemático. Esto puede dejar el servidor con una postura de seguridad más débil de la que tenía antes del hardening. Úsalo solo cuando el rollback individual no sea viable.
```

## Paso 6: Documentar la excepción

Tras resolver el incidente, documenta el control problemático para futuras ejecuciones:

1. Edita el perfil TOML (`/etc/vallumix/profiles/web.toml` o el perfil local) y añade el control a la lista de exclusiones.
2. Ejecuta un nuevo dry-run para confirmar que el control se omite.
3. Reporta el conflicto al proyecto Vallumix si crees que el control debería ser más inteligente (por ejemplo, detectar si `/tmp` está siendo usado por procesos activos antes de aplicar `noexec`).

```tip
Incluye un paso de validación funcional post-hardening en tus runbooks: después de `vallumix apply`, ejecuta un smoke test de tus servicios críticos (health checks, endpoints principales, conexiones de base de datos). Detecta problemas en minutos, no en horas.
```
