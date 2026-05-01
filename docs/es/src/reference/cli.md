# CLI Global

Esta página documenta las flags globales disponibles en todos los subcomandos de Vallumix, con ejemplos de uso y combinaciones comunes.

## `--profile <web|database|bastion>`

Selecciona el perfil de controles a ejecutar. Es obligatorio en `apply`, `audit` y `list`.

```bash
vallumix apply --profile web
vallumix audit --profile database
vallumix list --profile bastion
```

## `--dry-run`

Simula la ejecución completa sin modificar el sistema. Funciona con `apply` y `rollback`.

```bash
vallumix apply --profile web --dry-run --verbose
```

En modo dry-run:
- Se ejecutan `check` en todos los controles.
- Se informa qué cambios se harían.
- No se crean respaldos ni se modifican archivos.
- El código de salida es `0` si la validación de argumentos es correcta, independientemente del cumplimiento.

## `--verbose`

Muestra información detallada de cada control: comandos ejecutados, salida cruda, archivos modificados.

```bash
vallumix apply --profile web --verbose
```

Útil para depuración y para capturar trazabilidad técnica en auditorías.

## `--quiet`

Suprime toda la salida en consola excepto errores fatales. Los reportes se generan normalmente.

```bash
vallumix apply --profile web --quiet --report json --output /tmp/report
```

Ideal para pipelines donde solo interesa el código de salida y el artefacto de reporte.

## `--threshold <0-100>`

Define el porcentaje mínimo de cumplimiento para que la ejecución devuelva código de salida `0`.

```bash
vallumix apply --profile web --threshold 95
vallumix audit --profile database --threshold 85
```

Si la tasa de cumplimiento es inferior al umbral, Vallumix devuelve `1`.

```warning
El umbral aplica al resultado final de la ejecución. Un control `Failed` reduce la tasa de cumplimiento igual que uno `NonCompliant`. Configura el umbral de forma realista según la madurez de seguridad de tu entorno.
```

## `--no-color`

Deshabilita el coloreado de la salida terminal, respetando también la variable de entorno `NO_COLOR`.

```bash
vallumix apply --profile web --no-color
NO_COLOR=1 vallumix audit --profile web
```

## `--report <formatos>`

Especifica uno o más formatos de reporte separados por comas.

```bash
vallumix apply --profile web --report html
vallumix audit --profile database --report html,json,junit
vallumix apply --profile bastion --report text
```

Formatos soportados:

| Formato | Extensión | Uso típico |
|---|---|---|
| `html` | `.html` | Reporte ejecutivo, auditorías, clientes |
| `json` | `.json` | Integración con SIEMs, dashboards, scripts |
| `junit` | `.xml` | Jenkins, GitLab CI, GitHub Actions |
| `text` | `.txt` | Revisión rápida en terminal, logs |

## `--output <prefijo>`

Prefijo de ruta para los archivos de reporte. Si no se especifica, Vallumix usa un nombre temporal en `/tmp`.

```bash
vallumix apply --profile web --report html,json --output /var/reports/web-$(date +%Y%m%d)
# Genera: /var/reports/web-20260501.html y /var/reports/web-20260501.json
```

## Combinaciones comunes

```bash
# Auditoría silenciosa con reporte JSON para dashboard
vallumix audit --profile web --quiet --report json --output /var/lib/metrics/vallumix

# Aplicación con validación de umbral y reporte JUnit para CI
vallumix apply --profile database --threshold 95 --report junit --output /tmp/results

# Dry-run verbose para revisión manual antes de producción
vallumix apply --profile bastion --dry-run --verbose --report html --output /tmp/dryrun

# Listado completo sin colores para procesamiento con awk/grep
vallumix list --profile web --no-color
```
