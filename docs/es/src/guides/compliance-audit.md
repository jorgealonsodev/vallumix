# Auditoría de Cumplimiento (CU-02)

Esta guía describe cómo realizar una auditoría de seguridad sin modificar el sistema objetivo, generando reportes que sirven como evidencia para auditorías de cumplimiento ISO 27001, PCI-DSS o revisiones internas de seguridad.

## Escenario

- **Actor:** Consultor de seguridad con acceso temporal a un servidor de cliente.
- **Restricción:** No puede modificar configuraciones del sistema.
- **Objetivo:** Evaluar la postura de seguridad y entregar un reporte ejecutivo.
- **Distribución:** Desconocida hasta la conexión (podría ser Debian, Ubuntu, RHEL o derivada).

## Paso 1: Transferir Vallumix al servidor objetivo

Como Vallumix es un binario único estático, puedes copiarlo sin instalar dependencias:

```bash
# Desde tu máquina local
scp vallumix usuario@servidor-cliente:/tmp/
ssh usuario@servidor-cliente
sudo cp /tmp/vallumix /usr/local/bin/
```

```tip
Si no tienes acceso root en el servidor del cliente, solicita que un administrador local ejecute el binario por ti. El subcomando `audit` requiere root para leer archivos de configuración protegidos como `/etc/shadow` o `/etc/ssh/sshd_config`.
```

## Paso 2: Detectar la distribución y seleccionar perfil

```bash
sudo vallumix list
# Muestra los perfiles disponibles y los controles asociados
```

Basado en el rol del servidor, selecciona el perfil adecuado:

- Servidor web (Nginx, Apache) → `--profile web`
- Servidor de base de datos (PostgreSQL, MariaDB) → `--profile database`
- Bastión SSH → `--profile bastion`

Si no estás seguro del rol, ejecuta los tres perfiles y compara los resultados.

## Paso 3: Ejecutar la auditoría

```bash
sudo vallumix audit --profile database --report html,json --output /tmp/compliance-audit
```

El subcomando `audit` evalúa todos los controles del perfil pero **nunca ejecuta `apply`**. Solo realiza `pre_check` sobre cada control y genera el reporte.

### Salida esperada

```text
[  OK  ] 1.1.1.1  Disable cramfs support                 Compliant
[FAIL]  1.1.1.2  Disable freevxfs support               NonCompliant
[  OK  ] 2.1.1    Ensure autofs is not installed         Compliant
[FAIL]  3.1.1    Ensure IP forwarding is disabled       NonCompliant
[FAIL]  5.2.4    Ensure SSH root login is disabled      NonCompliant
[  OK  ] 5.2.5    Ensure SSH strict mode is enabled      Compliant
[FAIL]  6.1.1    Ensure permissions on /etc/passwd      NonCompliant

─────────────────────────────────────────────
Execution complete
Profile:        database
Controls run:   70
Compliant:      31
NonCompliant:   35
Skipped:        4
Compliance:     46.9%
Threshold:      not set
Status:         AUDIT ONLY (no changes applied)
Reports:        /tmp/compliance-audit.html
                /tmp/compliance-audit.json
─────────────────────────────────────────────
```

```note
El estado `AUDIT ONLY` en la salida confirma que no se modificó ningún archivo del sistema. Esto es una garantía importante que debes destacar ante el auditor o cliente.
```

## Paso 4: Generar el reporte HTML para el cliente

El reporte HTML es autocontenido: incluye CSS embebido, no requiere conexión a internet y se puede abrir directamente en cualquier navegador. Contiene:

- **Portada:** hostname, distribución, kernel, fecha y duración.
- **Resumen ejecutivo:** tasa de cumplimiento, comparación con ejecuciones previas.
- **Detalle por control:** ID CIS, descripción, severidad, estado, evidencia técnica.
- **Recomendaciones de remediación manual** para controles que no pueden automatizarse.
- **Mapeo a estándares:** referencias cruzadas a NIST 800-53, ISO 27001 Annex A y PCI-DSS.

## Paso 5: Extraer métricas del JSON para dashboards

Si el cliente tiene un dashboard de cumplimiento (Grafana, Splunk, etc.), el JSON es la fuente de datos:

```bash
# Extrae la tasa de cumplimiento global
jq '.summary.compliance_rate' /tmp/compliance-audit.json
# Salida: 46.9

# Lista todos los controles no compliant
jq '.controls[] | select(.status == "NonCompliant") | {id: .id, severity: .severity, description: .description}' /tmp/compliance-audit.json

# Cuenta controles críticos fallidos
jq '[.controls[] | select(.status == "NonCompliant" and .severity == "Critical")] | length' /tmp/compliance-audit.json
```

## Paso 6: Entregar resultados

El paquete de entrega para una auditoría típica incluye:

1. **Reporte HTML:** documento principal para revisión ejecutiva.
2. **Reporte JSON:** datos estructurados para integración con herramientas del cliente.
3. **Log de ejecución:** salida de `vallumix audit` con `--verbose` para trazabilidad técnica.
4. **Declaración de no modificación:** texto que certifica que la auditoría fue de solo lectura.

```tip
Ejecuta la auditoría con `--verbose` para capturar los comandos exactos que Vallumix usó para evaluar cada control. Esto proporciona trazabilidad técnica ante auditores que cuestionan la metodología de evaluación.
```
