# Informe JUnit XML

El reporter JUnit produce XML JUnit estándar que se integra de forma nativa con Jenkins, GitLab CI, GitHub Actions, Azure DevOps y cualquier otra plataforma CI/CD que entienda el formato de resultados de pruebas JUnit.

## Cuándo Usar JUnit

- **Bloqueo de CI/CD** — falla una compilación o despliegue si el cumplimiento cae por debajo del 100%.
- **Paneles de Jenkins** — visualiza tendencias de cumplimiento junto con resultados de pruebas unitarias.
- **Widgets de Merge Request de GitLab** — muestra controles fallidos directamente en el MR.
- **Anotaciones de GitHub Actions** — muestra controles fallidos como anotaciones de ejecución de verificaciones.

## Generar un Informe JUnit

```bash
vallumix audit --profile web --report junit --output /var/reports/vallumix/ci-results
```

Resultado: `/var/reports/vallumix/ci-results.xml`.

## Estructura XML

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="vallumix" tests="4" failures="1" skipped="1" errors="0">
  <testcase name="1.1.1.1" classname="Disable cramfs">
    <!-- Compliant: sin elemento hijo -->
  </testcase>
  <testcase name="5.2.4" classname="Disable root login">
    <failure message="should be no">PermitRootLogin yes</failure>
  </testcase>
  <testcase name="3.1.1" classname="Disable IP forwarding">
    <skipped message="skipped"/>
  </testcase>
  <testcase name="4" classname="D &amp; E &lt;test&gt;">
    <!-- Compliant: caracteres especiales escapados -->
  </testcase>
</testsuite>
```

## Mapeo de Campos

| Atributo JUnit | Fuente Vallumix | Notas |
|----------------|-----------------|-------|
| `testsuite.name` | `"vallumix"` | Identificador fijo. |
| `testsuite.tests` | `summary.total` | Controles totales evaluados. |
| `testsuite.failures` | `summary.fail` | Controles no cumplidos. |
| `testsuite.skipped` | `summary.skip` | Controles omitidos. |
| `testsuite.errors` | `0` | Los errores se reportan como fallos. |
| `testcase.name` | `control.id` | Identificador CIS. |
| `testcase.classname` | `control.description` | Título legible. |
| `failure.message` | `control.message` | Pista de remediación. |
| `failure` text | `control.evidence` | Estado actual. |

## Ejemplos de Integración CI/CD

### GitLab CI

```yaml
compliance:
  stage: test
  script:
    - vallumix audit --profile web --report junit --output compliance
  artifacts:
    reports:
      junit: compliance.xml
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

GitLab mostrará los controles fallidos en el widget del MR y marcará el pipeline como fallido si algún control no cumple.

### GitHub Actions

```yaml
- name: Run compliance audit
  run: vallumix audit --profile web --report junit --output compliance

- name: Publish JUnit results
  uses: mikepenz/action-junit-report@v4
  if: always()
  with:
    report_paths: 'compliance.xml'
```

### Jenkins

Usa el plugin JUnit para publicar el XML:

```groovy
post {
    always {
        junit 'compliance.xml'
    }
}
```

## Aplicación de Umbrales

Combina `--threshold` con el informe JUnit para hacer que la CLI salga con código distinto de cero cuando el cumplimiento es insuficiente. Las plataformas CI/CD tratan el código de salida `1` como un fallo de pipeline:

```bash
vallumix audit --profile web --report junit --output ci-results --threshold 100
```

```tip
Si deseas rastrear tendencias de cumplimiento a lo largo del tiempo, archiva los archivos XML JUnit en tu sistema CI. Tanto Jenkins como GitLab admiten gráficos históricos de resultados de pruebas que mostrarán tu tasa de cumplimiento a través de las compilaciones.
```
