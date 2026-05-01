# Hardening de Servidor Web (CU-01)

Esta guía describe el endurecimiento completo de un servidor Ubuntu 24.04 LTS recién aprovisionado que alojará una aplicación web servida por Nginx. El objetivo es aplicar los controles del perfil `web` sin interrumpir el servicio HTTP.

## Escenario

- **Servidor:** Ubuntu 24.04 LTS (Noble), 2 vCPU, 4 GB RAM.
- **Servicio:** Nginx 1.24 sirviendo una aplicación PHP-FPM en puerto 443.
- **Estado inicial:** Instalación mínima del servidor, sin hardening previo.
- **Objetivo:** Aplicar controles CIS del perfil web, generar reporte HTML para auditoría.

## Paso 1: Verificar el entorno

```bash
# Verifica la distribución y versión
cat /etc/os-release | grep PRETTY_NAME
# Salida esperada: PRETTY_NAME="Ubuntu 24.04 LTS"

# Confirma que Nginx está activo
systemctl is-active nginx
# Salida esperada: active

# Verifica que el puerto 443 está escuchando
ss -tlnp | grep :443
# Salida esperada: LISTEN 0 4096 *:443
```

## Paso 2: Instalar Vallumix

```bash
wget https://github.com/jorgealonsodev/vallumix/releases/download/v1.0.0/vallumix_1.0.0_amd64.deb
sudo dpkg -i vallumix_1.0.0_amd64.deb
vallumix --version
# Salida esperada: vallumix 1.0.0
```

## Paso 3: Auditoría inicial (opcional pero recomendada)

Antes de modificar nada, evalúa la postura actual para tener una línea base:

```bash
sudo vallumix audit --profile web --report html,json --output /tmp/pre-audit
```

Esto genera `/tmp/pre-audit.html` y `/tmp/pre-audit.json`. En un servidor nuevo, la tasa de cumplimiento suele estar entre 30% y 50%.

## Paso 4: Ejecutar hardening

```bash
sudo vallumix apply --profile web --report html --output /tmp/hardening-report
```

### Salida esperada (resumen)

```text
[  OK  ] 1.1.1.1  Disable cramfs support                 Compliant
[ FIX  ] 1.1.1.2  Disable freevxfs support               Remediated
[ FIX  ] 1.1.1.3  Disable jffs2 support                  Remediated
[  OK  ] 2.1.1    Ensure autofs is not installed         Compliant
[ FIX  ] 3.1.1    Ensure IP forwarding is disabled       Remediated
[  OK  ] 3.2.1    Ensure packet redirect sending is dis… Compliant
[ FIX  ] 3.4.1    Ensure firewalld is installed          Remediated
[ FIX  ] 5.2.4    Ensure SSH root login is disabled      Remediated
[  OK  ] 5.2.5    Ensure SSH strict mode is enabled      Compliant
[ FIX  ] 6.1.1    Ensure permissions on /etc/passwd      Remediated

─────────────────────────────────────────────
Execution complete
Profile:        web
Controls run:   70
Compliant:      28
Remediated:     38
Failed:         2
Skipped:        2
Compliance:     94.3%
Threshold:      90.0%
Status:         PASS
Report:         /tmp/hardening-report.html
─────────────────────────────────────────────
```

```warning
El perfil web no deshabilita Nginx ni cierra los puertos 80/443. Sin embargo, algunos controles de firewall (`3.4.x`) configuran reglas por defecto. Si tu aplicación requiere puertos adicionales (por ejemplo, 8080 para un API interna), revísalas después de la ejecución.
```

## Paso 5: Verificar que Nginx sigue funcionando

```bash
curl -I https://localhost/
# Debe devolver HTTP/2 200 con los headers de tu aplicación

systemctl status nginx
# Debe mostrar active (running)
```

## Paso 6: Re-ejecución idempotente

Para confirmar que Vallumix es idempotente, ejecútalo de nuevo:

```bash
sudo vallumix apply --profile web --report json --output /tmp/post-audit
```

En esta segunda ejecución, la mayoría de controles deberían aparecer como `Compliant` o `SkippedAlreadyCompliant`, con muy pocas o ninguna entrada `Remediated`.

## Paso 7: Documentar para auditoría

Conserva estos tres artefactos:

1. `/tmp/pre-audit.html` — línea base inicial.
2. `/tmp/hardening-report.html` — evidencia de hardening aplicado.
3. `/tmp/post-audit.json` — confirmación de idempotencia.

```tip
Programa una re-ejecución mensual de `vallumix audit --profile web --report html` como tarea cron. Esto detecta desviaciones de configuración causadas por actualizaciones de paquetes o cambios manuales de otros administradores.
```
