# Perfiles

Los perfiles son el mecanismo mediante el cual Vallumix adapta la selección y severidad de los controles al rol operativo del servidor. En lugar de aplicar un hardening genérico que puede romper servicios legítimos, cada perfil prioriza los controles relevantes y omite o suaviza aquellos que no tienen sentido para el contexto.

## Perfil Web

El perfil `web` está optimizado para servidores que alojan aplicaciones HTTP/HTTPS mediante Nginx, Apache u otros servidores web.

### Qué incluye

- Controles de filtrado de red enfocados en puertos 80 y 443.
- Límites de recursos (`ulimit`, `systemd` limits) para procesos del servidor web.
- Endurecimiento de configuración TLS: versiones mínimas de protocolo, ciphersuites seguras, deshabilitación de certificados autofirmados.
- Permisos restrictivos en directorios de despliegue (`/var/www`, `/usr/share/nginx/html`).
- Políticas de logging estructurado para análisis forense de accesos y errores.
- Deshabilitación de servicios innecesarios que no afectan la funcionalidad web (avahi, cups, etc.).

### Cuándo usarlo

- Servidores frontales que exponen aplicaciones web.
- Balanceadores de carga inversos (reverse proxies).
- Servidores de archivos estáticos.

```note
El perfil web no deshabilita el servicio HTTP ni restringe el acceso a los puertos web. Su objetivo es endurecer el host subyacente sin romper la funcionalidad del servidor.
```

## Perfil Database

El perfil `database` está diseñado para hosts que ejecutan motores de base de datos como PostgreSQL, MariaDB o MongoDB.

### Qué incluye

- Restricción de acceso de red a interfaces internas o localhost.
- Controles estrictos sobre montajes de filesystem, especialmente `/tmp` y `/var`, con opciones `noexec`, `nodev`, `nosuid`.
- Endurecimiento del kernel para cargas de trabajo de E/S intensivas (`vm.swappiness`, `dirty_ratio`).
- Desactivación de binarios SUID/SGID no esenciales.
- Configuración de PAM y límites de recursos para el usuario del motor de base de datos.
- Logging de auditoría para conexiones y consultas administrativas.

### Cuándo usarlo

- Servidores de base de datos dedicados.
- Nodos de réplica o clúster de bases de datos.
- Instancias donde el motor de BD es el servicio principal.

## Perfil Bastión

El perfil `bastion` es el más agresivo de los tres. Está pensado para hosts saltadores (jump hosts) cuyo único propósito es servir como punto de entrada SSH autenticado hacia infraestructura interna.

### Qué incluye

- Solo puerto 22 (SSH) expuesto; todo lo demás filtrado.
- Autenticación por clave pública obligatoria; deshabilitación de contraseñas.
- Soporte opcional para autenticación multifactor (MFA) con Google Authenticator.
- Registro exhaustivo de sesiones con `auditd` y `script`.
- Restricciones extensas de comandos y entorno mediante `ForceCommand`, `ChrootDirectory` y `Match` blocks.
- Endurecimiento máximo de PAM, sudo y cron.

### Cuándo usarlo

- Bastiones de acceso remoto a infraestructura interna.
- Servidores de administración centralizada.
- Cualquier host cuya única función sea SSH.

```danger
El perfil bastión puede romper servicios que no sean SSH. No lo apliques en servidores web, bases de datos o cualquier host que necesite ejecutar otros servicios. Siempre ejecuta `--dry-run` antes de aplicar este perfil en un entorno nuevo.
```

## Comparativa rápida

| Aspecto | Web | Database | Bastión |
|---|---|---|---|
| Puertos abiertos por defecto | 80, 443 | Puerto del motor BD | 22 |
| Agresividad | Media | Media-Alta | Máxima |
| Riesgo de ruptura de servicio | Bajo | Bajo-Medio | Alto si no es SSH-only |
| Controles de red | Firewall para web | Interfaces restringidas | Solo SSH |
| Logging | Accesos web | Auditoría de queries | Sesiones SSH completas |

## Selección de perfil

Usa la flag `--profile` en cualquier subcomando:

```bash
vallumix apply --profile web
vallumix audit --profile database --report html
vallumix list --profile bastion
```
