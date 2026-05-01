# Controles de Registro y Auditoría

El dominio de registro (sección CIS 4.1.x) contiene 11 controles que aseguran que los eventos del sistema sean capturados, retenidos y protegidos contra manipulación. Sin registros confiables, detectar intrusiones, reconstruir incidentes y satisfacer requisitos de cumplimiento es imposible. Este dominio cubre rsyslog, systemd-journald, auditd, rotación de logs y permisos de archivos de registro.

## `4.1.1.1` — Asegurar que rsyslog esté instalado

**Qué hace apply:**
Verifica que el paquete `rsyslog` esté instalado. Si falta, no lo instala automáticamente (para evitar cambios inesperados en el gestor de paquetes) pero reporta el control como no cumplido.

**Verificación manual:**

```bash
rpm -q rsyslog 2>/dev/null || dpkg -l rsyslog
```

**Justificación de seguridad:**
rsyslog es el demonio syslog estándar en la mayoría de las distribuciones Linux. Es la base para la recolección centralizada de logs, integración con SIEM y análisis forense. Una instalación faltante de rsyslog significa que muchos eventos del sistema nunca se persisten en disco.

## `4.1.1.2` — Asegurar que rsyslog esté configurado

**Qué hace apply:**
Comprueba `/etc/rsyslog.conf` y `/etc/rsyslog.d/*.conf` en busca de reglas de registro estándar (auth, authpriv, cron, kern, mail, user). Si faltan reglas, Vallumix las añade a `/etc/rsyslog.d/50-default.conf`.

**Verificación manual:**

```bash
grep -r 'auth,authpriv' /etc/rsyslog.d/
grep -r 'cron\.' /etc/rsyslog.d/
```

**Justificación de seguridad:**
Sin reglas rsyslog apropiadas, los fallos de autenticación, la salida de trabajos cron y los mensajes del kernel pueden descartarse silenciosamente. Este control asegura que el conjunto mínimo de reglas recomendadas por CIS esté presente.

## `4.1.1.3` — Asegurar permisos de archivos de rsyslog

**Qué hace apply:**
Establece los permisos de los archivos de configuración de rsyslog en `640` y la propiedad en `root:adm` (o grupo apropiado para la distribución).

**Verificación manual:**

```bash
stat -c '%a %U:%G' /etc/rsyslog.conf
stat -c '%a %U:%G' /etc/rsyslog.d/
```

**Justificación de seguridad:**
Los archivos de configuración de logs pueden redirigir, suprimir o desviar flujos de registro. Si son escribibles por usuarios no root, un atacante podría deshabilitar el registro de sus actividades o redirigir logs a `/dev/null`.

## `4.1.2.1` — Asegurar que journald esté configurado

**Qué hace apply:**
Verifica que `/etc/systemd/journald.conf` contenga `Storage=persistent` y `ForwardToSyslog=yes`. Si faltan, los añade bajo la sección `[Journal]`.

**Verificación manual:**

```bash
grep -E '^Storage=|^ForwardToSyslog=' /etc/systemd/journald.conf
```

**Justificación de seguridad:**
Por defecto, journald almacena logs en memoria volátil (`/run/log/journal`). Una configuración de almacenamiento persistente asegura que los logs sobrevivan a los reinicios, lo cual es esencial para la investigación post-reinicio de incidentes.

## `4.1.2.2` — Asegurar que exista anulación de journald

**Qué hace apply:**
Crea `/etc/systemd/journald.conf.d/99-vallumix.conf` con configuraciones endurecidas (`Compress=yes`, `SystemMaxUse=500M`, `MaxFileSec=1week`).

**Verificación manual:**

```bash
cat /etc/systemd/journald.conf.d/99-vallumix.conf
```

**Justificación de seguridad:**
Un archivo de anulación evita que las actualizaciones de la distribución sobrescriban las configuraciones de hardening. Limitar el uso de disco (`SystemMaxUse`) protege contra ataques de denegación de servicio por llenado de logs.

## `4.1.3.1` — Asegurar que auditd esté instalado

**Qué hace apply:**
Verifica que `auditd` y `audispd-plugins` estén instalados. Si faltan, reporta no cumplido.

**Verificación manual:**

```bash
rpm -q audit 2>/dev/null || dpkg -l auditd
```

**Justificación de seguridad:**
auditd captura eventos relevantes para la seguridad a nivel de kernel: acceso a archivos, invocación de syscalls, inicios de sesión de usuarios y escaladas de privilegios. Es la columna vertebral de las trazas de auditoría de Linux y es requerido por la mayoría de los marcos de cumplimiento.

## `4.1.3.2` — Asegurar que auditd esté configurado

**Qué hace apply:**
Comprueba que `/etc/audit/auditd.conf` contenga `log_group = root` (o `adm`) y `max_log_file_action = ROTATE`.

**Verificación manual:**

```bash
grep '^max_log_file_action' /etc/audit/auditd.conf
grep '^log_group' /etc/audit/auditd.conf
```

**Justificación de seguridad:**
Una configuración apropiada de auditd asegura que los logs roten automáticamente y sean legibles solo por administradores autorizados. Sin rotación, la partición de auditoría puede llenarse y bloquear el demonio.

## `4.1.4.1` — Asegurar que existan reglas de auditoría de identidad

**Qué hace apply:**
Añade reglas de auditoría para monitorear cambios en archivos de identidad: `/etc/group`, `/etc/passwd`, `/etc/gshadow`, `/etc/shadow`, `/etc/security/opasswd`.

**Verificación manual:**

```bash
auditctl -l | grep -E '(/etc/passwd|/etc/shadow|/etc/group|/etc/gshadow)'
```

**Justificación de seguridad:**
Los cambios no autorizados en archivos de identidad son un indicador claro de creación de cuentas, escalada de privilegios o instalación de puertas traseras. Monitorear estos archivos permite la detección en tiempo real de manipulaciones.

## `4.1.4.2` — Asegurar auditoría de eventos de inicio de sesión

**Qué hace apply:**
Añade reglas de auditoría para `/var/log/lastlog`, `/var/run/faillock` y binarios de inicio de sesión (`/usr/bin/login`, `/usr/bin/su`).

**Verificación manual:**

```bash
auditctl -l | grep -E '(lastlog|faillock|/usr/bin/login|/usr/bin/su)'
```

**Justificación de seguridad:**
Los eventos de inicio de sesión son la fuente primaria de datos para detección de fuerza bruta y revisiones de acceso. Sin reglas de auditoría, los inicios de sesión fallidos y exitosos pueden existir solo en buffers de corta duración.

## `4.1.4.3` — Asegurar auditoría de eventos de sesión

**Qué hace apply:**
Añade reglas de auditoría para inicio de sesión (`/usr/bin/sudo`, `/usr/bin/sudoedit`, `/usr/bin/ssh`) y terminación de sesión.

**Verificación manual:**

```bash
auditctl -l | grep -E '(/usr/bin/sudo|/usr/bin/ssh)'
```

**Justificación de seguridad:**
La auditoría a nivel de sesión captura quién ejecutó comandos privilegiados y cuándo. Esto es crítico para la no repudio y para reconstruir la línea de tiempo de un incidente.

## `4.1.7` — Asegurar que logrotate esté configurado

**Qué hace apply:**
Verifica que `/etc/logrotate.conf` exista y que `/var/log` esté cubierto por una regla de logrotate con período de rotación `weekly` o menor y retención de al menos 4 rotaciones.

**Verificación manual:**

```bash
grep -E '^weekly|^rotate' /etc/logrotate.conf
ls /etc/logrotate.d/
```

**Justificación de seguridad:**
Sin rotación de logs, estos crecen indefinidamente y eventualmente agotan el espacio en disco. Los atacantes pueden explotar esto inundando logs para ocultar sus rastros o causar una denegación de servicio. La rotación también delimita la ventana de retención, lo que ayuda a cumplir con regulaciones de protección de datos.
