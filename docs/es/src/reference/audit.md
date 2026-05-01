# Comando `audit`

El subcomando `audit` evalúa la postura de seguridad del sistema sin aplicar ningún cambio. Es la herramienta principal para auditorías de cumplimiento, evaluaciones de riesgo y líneas base de seguridad.

## Sintaxis

```bash
vallumix audit [FLAGS] --profile <perfil>
```

## Flags específicas de audit

| Flag | Descripción |
|---|---|
| `--profile` | **Obligatorio.** Perfil de controles a auditar. |
| `--threshold` | Umbral mínimo de cumplimiento para éxito. |
| `--report` | Formatos de reporte a generar. |
| `--output` | Prefijo de ruta para reportes. |
| `--verbose` | Muestra detalle de cada verificación. |
| `--quiet` | Silencia la salida en consola. |

## Comportamiento

1. **Validación:** root, distro soportada.
2. **Carga de perfil:** lee el TOML y resuelve controles.
3. **Iteración por controles:** para cada control, ejecuta únicamente `check`.
   - No se crean respaldos.
   - No se ejecuta `apply`.
   - No se ejecuta `post_check` (no hay cambio que verificar).
4. **Generación de reportes:** con los estados `Compliant` o `NonCompliant`.
5. **Resumen:** tasa de cumplimiento.
6. **Código de salida:** `0` si cumple umbral, `1` si no.

## Paralelismo con rayon

A diferencia de `apply`, el modo `audit` paraleliza la ejecución de `check` mediante `rayon`. Los controles que solo leen estado del sistema son independientes entre sí, por lo que pueden evaluarse concurrentemente de forma segura.

```rust
// Simplificación del motor de audit
use rayon::prelude::*;

let results: Vec<ControlResult> = controls
    .par_iter()   // Iterador paralelo de rayon
    .map(|control| control.check(&ctx))
    .collect();
```

Esto reduce significativamente el tiempo de auditoría en sistemas con muchos controles. El paralelismo es seguro porque `check` es de solo lectura y cada control implementa `Send + Sync`.

## Ejemplos

### Auditoría básica

```bash
sudo vallumix audit --profile web
```

Evalúa todos los controles del perfil web y muestra el resumen en consola.

### Auditoría con reporte HTML y JSON

```bash
sudo vallumix audit --profile database --report html,json --output /tmp/compliance-audit
```

Genera `/tmp/compliance-audit.html` y `/tmp/compliance-audit.json`.

### Auditoría con umbral para pipeline

```bash
sudo vallumix audit --profile web --threshold 85 --report junit --output /tmp/audit.xml
```

Devuelve `1` si la tasa de cumplimiento es menor al 85%. Útil para gates de calidad en CI/CD.

### Auditoría verbose para diagnóstico

```bash
sudo vallumix audit --profile bastion --verbose
```

Muestra los comandos exactos que se ejecutaron para evaluar cada control, útil para entender falsos positivos o negativos.

## Estados de control en audit

| Estado | Significado |
|---|---|
| `Compliant` | El sistema cumple con la recomendación CIS. |
| `NonCompliant` | El sistema no cumple; se documenta la evidencia actual. |
| `Skipped` | El control no es aplicable a esta distribución. |

```tip
Ejecuta `audit` periódicamente (semanal o mensual) como tarea cron para detectar desviaciones de configuración causadas por actualizaciones de paquetes, cambios manuales de administradores o despliegues de aplicaciones. Un `audit` que baja de 95% a 70% en una semana es una señal de alerta clara.
```
