# Visión General del Sistema de Informes

Vallumix genera informes estructurados después de cada ejecución de `audit` o `apply`. Los informes resumen la información del host, las estadísticas de cumplimiento y el estado detallado de cada control evaluado. El sistema de informes se implementa en el crate `vallumix-reporters` y admite cuatro formatos de salida.

## Formatos de Informe de un Vistazo

| Formato | Caso de Uso | Salida | Extensión de Archivo |
|---------|-------------|--------|----------------------|
| **HTML** | Auditores, presentaciones de gestión, evidencia de cumplimiento | Archivo autónomo con CSS incrustado | `.html` |
| **JSON** | Paneles de control, ingestión en SIEM, análisis programático | Objeto JSON con formato legible | `.json` |
| **JUnit** | Pipelines CI/CD, Jenkins, GitLab CI, GitHub Actions | XML JUnit estándar | `.xml` |
| **Texto** | Retroalimentación rápida en CLI, revisión en terminal, triaje | Texto plano coloreado con iconos | `.txt` |

## Generar un Informe

Usa la bandera `--report` para seleccionar el formato y `--output` para especificar la ruta de destino (sin extensión):

```bash
# Informe HTML para un auditor
vallumix audit --profile web --report html --output /var/reports/vallumix/audit-2024-06-01

# Informe JSON para un panel de control
vallumix audit --profile web --report json --output /var/reports/vallumix/audit-2024-06-01

# Informe JUnit para CI/CD
vallumix audit --profile web --report junit --output /var/reports/vallumix/ci-results

# Informe de texto para revisión en terminal
vallumix audit --profile web --report text --output /var/reports/vallumix/audit-2024-06-01
```

Si se omite `--output`, Vallumix escribe el informe en un archivo con marca temporal en `/tmp`.

```tip
Puedes generar múltiples informes de la misma ejecución ejecutando `audit` una vez y convirtiendo la salida JSON, o ejecutando `audit` con diferentes valores de `--report` en invocaciones separadas. Dado que Vallumix es idempotente, reejecutar `audit` no cambia el estado del sistema.
```

## Modelo de Datos del Informe

Cada informe contiene los mismos datos subyacentes, independientemente del formato:

- **`host.hostname`** — el nombre del host del servidor.
- **`host.distro`** — la distribución detectada (por ejemplo, `debian/12`, `rocky/9`).
- **`summary.total`** — número total de controles evaluados.
- **`summary.pass`** — controles marcados como `Compliant`.
- **`summary.fail`** — controles marcados como `NonCompliant`.
- **`summary.skip`** — controles marcados como `Skipped` (por ejemplo, dry-run o no aplicable).
- **`summary.compliance_rate`** — porcentaje de controles cumplidos (`pass / total * 100`).
- **`controls[]`** — array de resultados individuales de controles, cada uno con `id`, `description`, `severity`, `status`, `evidence` y `message` opcional.

## Aplicación de Umbrales

Vallumix admite un umbral de cumplimiento. Si la tasa de cumplimiento cae por debajo del umbral, la CLI sale con código `1`, lo que señala fallo a los pipelines CI/CD y herramientas de orquestación:

```bash
vallumix audit --profile web --threshold 95
```

Esto es especialmente útil con el reporter **JUnit**: una etapa de pipeline puede bloquear el despliegue ante un resultado de cumplimiento del 100%.

## Notas Específicas por Formato

- Los informes **HTML** son completamente autónomos; no requieren archivos CSS o JavaScript externos. Puedes enviar un único archivo `.html` a un auditor o abrirlo sin conexión.
- Los informes **JSON** usan nombres de campo en snake_case y formato legible para humanos. Se pueden canalizar directamente a `jq` para filtrar.
- Los informes **JUnit** mapean `Compliant` a pruebas aprobadas, `NonCompliant` a `<failure>`, y `Skipped` a `<skipped>`. El atributo `classname` es la descripción del control, y `name` es el ID del CIS.
- Los informes de **texto** detectan automáticamente `NO_COLOR` y eliminan los códigos de escape ANSI. En un pipeline, recaen en etiquetas de texto plano (`OK`, `FAIL`, `SKIP`).

Para ejemplos detallados de cada formato, consulta las páginas dedicadas en esta sección.
