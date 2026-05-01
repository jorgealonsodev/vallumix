# Comando `rollback`

El subcomando `rollback` restaura configuraciones previas desde los respaldos versionados generados por `apply`. Permite revertir cambios de forma granular o masiva.

## Sintaxis

```bash
vallumix rollback [FLAGS] [--control-id <id> | --session <sesión>]
```

## Flags específicas de rollback

| Flag | Descripción |
|---|---|
| `--control-id <id>` | Revierte únicamente el control especificado (por ejemplo, `5.2.4`). |
| `--session <sesión>` | Revierte todos los controles de una sesión. Valores: `last` o timestamp (`2026-05-01T09-15-33`). |
| `--dry-run` | Muestra qué se restauraría sin hacerlo. |
| `--verbose` | Muestra detalle de cada archivo restaurado. |
| `--quiet` | Silencia la salida en consola. |
| `--report` | Formatos de reporte del rollback. |
| `--output` | Prefijo de ruta para reportes. |

```warning
Debes especificar exactamente uno de `--control-id` o `--session`. Si no especificas ninguno, o si especificas ambos, Vallumix devuelve un error de configuración (código `2`).
```

## Comportamiento

1. **Validación:** root, existencia del directorio de respaldos.
2. **Resolución de sesión:** si se usa `--session last`, selecciona el directorio con el timestamp más reciente.
3. **Verificación de integridad:** recalcula SHA-256 de cada archivo de respaldo y lo compara con `checksums.sha256`.
4. **Restauración:** copia los archivos de respaldo a sus ubicaciones originales.
5. **Post-check:** ejecuta `check` en los controles restaurados para confirmar que el sistema volvió al estado previo.
6. **Reporte:** documenta qué controles se restauraron, cuáles fallaron la verificación de integridad, y el estado final.

## Ejemplos

### Rollback del último control problemático

```bash
sudo vallumix rollback --control-id 5.2.4
```

Restaura únicamente el control `5.2.4` de la última sesión de respaldo.

### Rollback de la última sesión completa

```bash
sudo vallumix rollback --session last
```

Restaura todos los controles aplicados en la sesión más reciente.

### Rollback de una sesión específica

```bash
sudo vallumix rollback --session 2026-05-01T09-15-33
```

Restaura todos los controles de la sesión con timestamp exacto.

### Dry-run de rollback

```bash
sudo vallumix rollback --session last --dry-run --verbose
```

Muestra qué archivos se restaurarían, desde qué respaldos, y el resultado esperado del post-check, sin modificar nada.

## Estados de control en rollback

| Estado | Significado |
|---|---|
| `Restored` | El control fue revertido exitosamente; el post-check confirma el estado previo. |
| `RestoreFailed` | La restauración falló: archivo de respaldo corrupto, permisos insuficientes, o el post-check no coincide. |
| `IntegrityCheckFailed` | El SHA-256 del respaldo no coincide con el registrado; la restauración se aborta para este control. |
| `NotFound` | No existe respaldo para el control solicitado en la sesión indicada. |

```danger
Si un control muestra `IntegrityCheckFailed`, no intentes restaurar manualmente el archivo de respaldo sin investigar primero por qué el hash cambió. Podría indicar corrupción de disco, manipulación del archivo, o un bug en el proceso de respaldo. Contacta al administrador del sistema o al equipo de seguridad.
```
