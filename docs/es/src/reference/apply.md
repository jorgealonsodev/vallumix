# Comando `apply`

El subcomando `apply` es la operación principal de Vallumix. Evalúa cada control del perfil seleccionado y, si el sistema no cumple, aplica la remediación correspondiente tras crear un respaldo.

## Sintaxis

```bash
vallumix apply [FLAGS] --profile <perfil>
```

## Flags específicos de apply

Además de las flags globales, `apply` soporta:

| Flag | Descripción |
|---|---|
| `--profile` | **Obligatorio.** Perfil de controles a aplicar. |
| `--dry-run` | Simula la ejecución sin modificar nada. |
| `--threshold` | Umbral mínimo de cumplimiento para éxito. |
| `--report` | Formatos de reporte a generar. |
| `--output` | Prefijo de ruta para reportes. |
| `--verbose` | Muestra detalle de cada operación de control. |
| `--quiet` | Silencia la salida en consola. |

## Comportamiento

1. **Validación:** comprueba root, distro soportada, espacio en disco.
2. **Carga de perfil:** lee el TOML del perfil y resuelve la lista de controles.
3. **Creación de sesión de respaldo:** directorio versionado en `/var/backups/vallumix/<timestamp>/`.
4. **Iteración por controles:** para cada control:
   - `check`: ¿cumple? → `Compliant`, salta al siguiente.
   - `backup`: copia archivos a la sesión de respaldo.
   - `apply`: ejecuta la remediación.
   - `post_check`: verifica que el cambio surtió efecto.
5. **Generación de reportes:** según los formatos solicitados.
6. **Resumen:** tasa de cumplimiento, estados, ruta al reporte.
7. **Código de salida:** `0` si cumple umbral, `1` si no, `2` o `3` si hay errores.

## Ejemplos

### Aplicación básica

```bash
sudo vallumix apply --profile web
```

Aplica el perfil web con configuración por defecto. Genera un reporte HTML en `/tmp`.

### Aplicación con umbral y reporte JSON

```bash
sudo vallumix apply --profile database --threshold 90 --report json --output /tmp/db-hardening
```

Si la tasa de cumplimiento es inferior al 90%, el comando devuelve `1`.

### Dry-run previo a aplicación

```bash
sudo vallumix apply --profile bastion --dry-run --verbose
```

Muestra todos los cambios que se harían sin ejecutarlos.

### Aplicación silenciosa en pipeline

```bash
sudo vallumix apply --profile web --quiet --threshold 95 --report junit --output /tmp/results
```

Útil en scripts donde solo importa el código de salida y el artefacto JUnit.

## Estados de control en apply

En el reporte de una ejecución `apply`, cada control aparece con uno de estos estados:

| Estado | Significado |
|---|---|
| `Compliant` | El sistema ya cumplía antes de la ejecución. |
| `Remediated` | No cumplía; se aplicó la remediación y el post-check pasó. |
| `Failed` | No cumplía; se intentó remediar pero el post-check falló. |
| `Skipped` | El control no es aplicable a esta distribución o fue excluido del perfil. |
| `SkippedAlreadyCompliant` | Variante explícita de `Compliant` por idempotencia. |

```danger
Un control en estado `Failed` indica que el sistema sigue en una configuración insegura para ese control. Revisa el reporte detallado para entender por qué la remediación no funcionó: puede deberse a un archivo de configuración inesperado, a un servicio que bloquea la modificación, o a un bug en la implementación del control.
```
