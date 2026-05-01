# Informe JSON

El reporter JSON produce un documento JSON legible para máquinas, con formato bonito, que contiene el resultado completo de la auditoría o aplicación. Es el formato preferido para la integración con paneles de control, plataformas SIEM y scripts de automatización personalizados.

## Cuándo Usar JSON

- **Grafana / paneles personalizados** — ingiere el JSON en una base de datos de series temporales o muéstralo en un panel web.
- **Integración con SIEM** — reenvía el informe a Splunk, Elastic Security o Sentinel mediante un enviador de logs.
- **Scripts de automatización** — analiza el JSON con `jq`, Python o cualquier lenguaje con biblioteca JSON.
- **Diferencias temporales** — almacena informes JSON diarios en Git o almacenamiento de objetos y compara tendencias de cumplimiento.

## Generar un Informe JSON

```bash
vallumix audit --profile web --report json --output /var/reports/vallumix/audit
```

Resultado: `/var/reports/vallumix/audit.json`.

## Estructura JSON

```json
{
  "host": {
    "hostname": "web01",
    "distro": "debian/12"
  },
  "summary": {
    "total": 45,
    "pass": 38,
    "fail": 5,
    "skip": 2,
    "compliance_rate": 84.4
  },
  "controls": [
    {
      "id": "1.1.1.1",
      "description": "Disable cramfs",
      "severity": "Low",
      "status": "Compliant",
      "evidence": "not present",
      "message": null
    },
    {
      "id": "5.2.4",
      "description": "Disable root login",
      "severity": "High",
      "status": "NonCompliant",
      "evidence": "PermitRootLogin yes",
      "message": "should be no"
    },
    {
      "id": "3.1.1",
      "description": "Disable IP forwarding",
      "severity": "Medium",
      "status": "Skipped",
      "evidence": "dry-run",
      "message": null
    }
  ]
}
```

## Referencia de Campos

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `host.hostname` | string | Nombre del host del servidor en el momento de la ejecución. |
| `host.distro` | string | Distribución y versión detectadas (por ejemplo, `rocky/9`). |
| `summary.total` | integer | Controles totales evaluados. |
| `summary.pass` | integer | Controles con estado `Compliant`. |
| `summary.fail` | integer | Controles con estado `NonCompliant`. |
| `summary.skip` | integer | Controles con estado `Skipped`. |
| `summary.compliance_rate` | float | Porcentaje de pass / total. |
| `controls[].id` | string | Identificador de control CIS (por ejemplo, `5.2.4`). |
| `controls[].description` | string | Título legible del control. |
| `controls[].severity` | string | `Low`, `Medium` o `High`. |
| `controls[].status` | string | `Compliant`, `NonCompliant` o `Skipped`. |
| `controls[].evidence` | string | Estado actual o razón del estado. |
| `controls[].message` | string o null | Pista de remediación o detalle del error. |

## Procesamiento con jq

```bash
# Extraer solo controles de alta severidad fallidos
vallumix audit --profile web --report json | \
  jq '.controls[] | select(.status == "NonCompliant" and .severity == "High")'

# Calcular tasa de cumplimiento desde un informe guardado
jq '.summary.compliance_rate' /var/reports/vallumix/audit.json

# Contar controles por severidad
jq '[.controls[].severity] | group_by(.) | map({severity: .[0], count: length})' audit.json
```

## Ejemplo de Ingestión en SIEM

Para Splunk o Fluent Bit, envuelve el informe JSON en un sobre de una sola línea y reenvíalo:

```bash
vallumix audit --profile web --report json --output /var/log/vallumix/last-audit
jq -c '.' /var/log/vallumix/last-audit.json >> /var/log/vallumix/audit-stream.log
```

Tu enviador de logs puede entonces analizar cada línea como un objeto JSON e indexar campos como `host.hostname`, `summary.compliance_rate` y `controls[].id`.

```tip
El reporter JSON usa `serde_json::to_string_pretty`, por lo que la salida es legible para humanos por defecto. Para pipelines de producción, canalízala a través de `jq -c` para minificar y reducir el tamaño de transferencia.
```
