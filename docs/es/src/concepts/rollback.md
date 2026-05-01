# Rollback

El sistema de rollback de Vallumix permite revertir cambios de hardening de forma granular, ya sea por control individual o por sesión completa. Esto transforma el hardening de una operación de riesgo en una operación reversible.

## Arquitectura de respaldos

Cada vez que Vallumix aplica cambios, crea una sesión de respaldo versionada en `/var/backups/vallumix/`:

```
/var/backups/vallumix/
├── 2026-04-30T14-22-18/
│   ├── manifest.json          # Metadatos de la sesión
│   ├── 5.2.4/
│   │   └── sshd_config.bak    # Archivo respaldado por control 5.2.4
│   ├── 1.1.1.1/
│   │   └── modprobe.cramfs.bak
│   └── checksums.sha256       # Hashes SHA-256 de todos los archivos respaldados
└── 2026-04-30T15-07-03/
    └── ...
```

Cada sesión contiene:

- **Directorio con timestamp ISO 8601:** identifica cuándo se ejecutó.
- **`manifest.json`:** lista los controles aplicados, la distribución, el perfil usado y el usuario que ejecutó Vallumix.
- **Subdirectorios por control ID:** cada control que modificó archivos tiene su propio directorio con las copias de seguridad.
- **`checksums.sha256`:** hash criptográfico de cada archivo respaldado, permitiendo detectar corrupción o manipulación.

## Rollback por control

Si identificas que un control específico causó un problema, reviértelo sin afectar los demás:

```bash
sudo vallumix rollback --control-id 5.2.4
```

Este comando:

1. Lee el `manifest.json` de la última sesión.
2. Localiza el respaldo del control `5.2.4`.
3. Verifica la integridad del archivo mediante SHA-256.
4. Restaura el archivo original en su ubicación.
5. Ejecuta `check` para confirmar que el sistema volvió al estado previo.
6. Registra la acción en el log y genera un reporte de rollback.

```tip
El rollback por control es la opción preferida cuando sabes exactamente qué cambio rompió algo. Es rápido, preciso y minimiza el riesgo de revertir controles correctos que no tienen nada que ver con el incidente.
```

## Rollback por sesión

Si no estás seguro de qué control causó el problema, o si varios controles interactuaron de forma inesperada, revierte toda una sesión:

```bash
# Última sesión automáticamente
sudo vallumix rollback --session last

# Sesión específica por timestamp
sudo vallumix rollback --session 2026-04-30T14-22-18
```

El rollback por sesión restaura todos los archivos respaldados en esa ejecución, en el orden inverso a como fueron aplicados, para manejar correctamente las dependencias entre archivos.

```danger
El rollback por sesión es una operación destructiva que revierte múltiples cambios. Aunque los respaldos están protegidos por checksums, una restauración masiva puede afectar la estabilidad del sistema si otros administradores han realizado cambios manuales en los mismos archivos entre la sesión de hardening y el rollback. Comunica siempre antes de ejecutar un rollback en producción.
```

## Verificación de integridad

Antes de restaurar cualquier archivo, Vallumix recalcula el hash SHA-256 y lo compara con el registrado en `checksums.sha256`. Si no coinciden, el rollback aborta para ese archivo y lo marca como `IntegrityCheckFailed` en el reporte.

## Limpieza de respaldos antiguos

Los respaldos no se eliminan automáticamente. Para evitar que `/var/backups/vallumix` crezca indefinidamente, configura una tarea cron que conserve solo las últimas N sesiones:

```bash
# Conservar solo las últimas 10 sesiones
0 2 * * * cd /var/backups/vallumix && ls -t | tail -n +11 | xargs rm -rf
```
