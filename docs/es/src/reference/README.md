# Referencia

Esta sección documenta exhaustivamente la interfaz de línea de comandos de Vallumix: subcomandos, flags globales, códigos de salida y comportamiento esperado de cada operación.

## Subcomandos

Vallumix sigue el patrón de subcomandos popularizado por `git` y `cargo`:

| Subcomando | Descripción | Modifica el sistema |
|---|---|---|
| `apply` | Aplica los controles del perfil seleccionado, genera respaldos y reportes | **Sí** |
| `audit` | Evalúa el estado sin aplicar cambios, genera reporte de cumplimiento | No |
| `rollback` | Restaura configuraciones desde respaldos versionados | **Sí** (revierte) |
| `list` | Muestra el catálogo de controles disponibles | No |
| `completion` | Genera scripts de autocompletado para shells | No |

## Flags globales

Estas flags están disponibles en todos los subcomandos:

| Flag | Valor | Descripción |
|---|---|---|
| `--profile` | `web`, `database`, `bastion` | Perfil de controles a utilizar |
| `--dry-run` | — | Simula la ejecución sin modificar el sistema |
| `--verbose` | — | Muestra salida detallada de cada control |
| `--quiet` | — | Suprime la salida en consola (solo reportes) |
| `--threshold` | `0`–`100` | Porcentaje mínimo de cumplimiento para código de salida 0 |
| `--no-color` | — | Deshabilita colores en la salida terminal |
| `--report` | `html`, `json`, `junit`, `text` | Formatos de reporte a generar (separados por coma) |
| `--output` | ruta | Prefijo de ruta para los archivos de reporte |
| `--help` | — | Muestra ayuda del subcomando |
| `--version` | — | Muestra versión de Vallumix |

## Códigos de salida

Vallumix devuelve códigos de salida explícitos que facilitan la integración con scripts y pipelines:

| Código | Significado | Cuándo ocurre |
|---|---|---|
| `0` | Éxito | Cumplimiento ≥ umbral (o sin umbral configurado); operación completada sin errores |
| `1` | Umbral no alcanzado | La tasa de cumplimiento está por debajo de `--threshold` |
| `2` | Error de configuración | Perfil inválido, distro no soportada, argumentos incorrectos |
| `3` | Error de privilegios | Operación que requiere root ejecutada sin privilegios efectivos |

```tip
Diseña tus scripts de wrapper para que capturen el código `1` como "política de seguridad violada" y el código `2` como "revisar configuración". No trates ambos como genéricos "errores".
```

## Convenciones de salida

- En modo interactivo con TTY, la salida usa colores, iconos (✓ ✗ ⚠ ℹ) y barras de progreso.
- En modo no-TTY (pipes, redirecciones, CI), la salida se simplifica a texto plano sin códigos de escape ANSI, respetando la variable `NO_COLOR`.
- El log estructurado se controla mediante `RUST_LOG` y la flag `--log-level`.
