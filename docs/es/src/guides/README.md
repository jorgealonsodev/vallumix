# Guías

Esta sección contiene guías prácticas orientadas a casos de uso reales. Cada guía parte de un escenario concreto, proporciona los comandos exactos y explica el resultado esperado.

## Casos de uso cubiertos

Vallumix fue diseñado respondiendo a cinco escenarios recurrentes en la operación de servidores Linux:

| Caso de uso | Escenario | Guía |
|---|---|---|
| **CU-01: Hardening inicial** | Nuevo servidor web, necesitas endurecerlo sin romper Nginx | [Hardening de servidor web](guides/hardening-web-server.md) |
| **CU-02: Auditoría de cumplimiento** | Evaluar postura de seguridad sin modificar nada | [Auditoría de cumplimiento](guides/compliance-audit.md) |
| **CU-03: Validación previa** | Revisar cambios planeados antes de tocar producción | [Validación en dry-run](guides/dry-run-validation.md) |
| **CU-04: Rollback tras incidente** | Un servicio dejó de funcionar tras aplicar hardening | [Rollback tras incidente](guides/rollback-after-incident.md) |
| **CU-05: Integración CI/CD** | Automatizar hardening en pipelines de construcción de imágenes | [Integración CI/CD](guides/ci-cd-integration.md) |

## ¿Qué guía necesitas?

- Si eres **administrador de sistemas** y acabas de aprovisionar un servidor → CU-01.
- Si eres **consultor de seguridad** y necesitas un reporte para el cliente → CU-02.
- Si eres **ingeniero de plataformas** y no tocas producción sin validar antes → CU-03.
- Si eres **SRE** y recibiste una alerta tras un cambio de hardening → CU-04.
- Si eres **DevOps** y construyes imágenes base con Packer o Terraform → CU-05.

```tip
Todas las guías asumen que Vallumix está instalado y que tienes acceso root en el sistema objetivo. Si aún no has instalado Vallumix, comienza por la [sección de instalación](installation/README.md).
```

## Convenciones usadas en las guías

- Los comandos precedidos por `$` se ejecutan como usuario normal.
- Los comandos precedidos por `#` se ejecutan como root.
- La salida esperada se muestra en bloques de código sin prompt.
- Los bloques ```` ```admonish ```` señalan advertencias sobre operaciones destructivas o requisitos de privilegios.
