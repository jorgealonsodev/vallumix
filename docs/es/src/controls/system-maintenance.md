# Controles de Mantenimiento del Sistema

El dominio de mantenimiento del sistema (sección CIS 6.1.x) contiene 8 controles enfocados en permisos de archivos, propiedad y auditoría periódica de ejecutables del sistema. Estos controles no cambian servicios en ejecución ni comportamiento de red; en su lugar, aseguran que los archivos críticos no sean legibles ni escribibles por usuarios no autorizados, y que no existan binarios SUID/SGID inesperados en el sistema.

## `6.1.1` — Asegurar permisos en /etc/passwd

**Qué hace apply:**
Establece `/etc/passwd` en modo `644` y propiedad `root:root`.

**Verificación manual:**

```bash
stat -c '%a %U:%G' /etc/passwd
```

Se espera: `644 root:root`.

**Justificación de seguridad:**
`/etc/passwd` contiene nombres de usuario, UIDs, directorios home y shells predeterminados. Aunque no contiene hashes de contraseña (esos están en `/etc/shadow`), filtra datos de enumeración de usuarios e información de shell que ayuda a los atacantes a elaborar exploits dirigidos.

## `6.1.2` — Asegurar permisos en /etc/shadow

**Qué hace apply:**
Establece `/etc/shadow` en modo `000` o `640` y propiedad `root:shadow`.

**Verificación manual:**

```bash
stat -c '%a %U:%G' /etc/shadow
```

**Justificación de seguridad:**
`/etc/shadow` almacena hashes de contraseñas, envejecimiento de contraseñas y estado de bloqueo de cuentas. Si es legible por usuarios no root, un atacante puede extraer hashes para cracking offline con herramientas como Hashcat o John the Ripper.

## `6.1.3` — Asegurar permisos en /etc/group

**Qué hace apply:**
Establece `/etc/group` en modo `644` y propiedad `root:root`.

**Verificación manual:**

```bash
stat -c '%a %U:%G' /etc/group
```

**Justificación de seguridad:**
La membresía de grupo revela qué usuarios tienen acceso a recursos compartidos. Mantener este archivo legible para todos generalmente es aceptable, pero no debe ser escribible por nadie que no sea root.

## `6.1.4` — Asegurar permisos en /etc/gshadow

**Qué hace apply:**
Establece `/etc/gshadow` en modo `000` o `640` y propiedad `root:shadow`.

**Verificación manual:**

```bash
stat -c '%a %U:%G' /etc/gshadow
```

**Justificación de seguridad:**
`/etc/gshadow` almacena contraseñas de grupo y listas de administradores. Al igual que `/etc/shadow`, debe protegerse del acceso de lectura para prevenir ataques offline sobre credenciales de grupo.

## `6.1.5` — Auditar archivos escribibles por todos

**Qué hace apply:**
Ejecuta `find` para localizar archivos con modo `o+w` que no tengan el sticky bit configurado en su directorio. El control reporta la lista pero no remedia automáticamente, porque los archivos escribibles por todos pueden ser intencionales (por ejemplo, directorios de spool compartidos).

**Verificación manual:**

```bash
find / -xdev -type f -perm -002 ! -perm -1000 -exec ls -l {} \; 2>/dev/null
```

**Justificación de seguridad:**
Los archivos escribibles por todos permiten que cualquier usuario del sistema los modifique. Si dicho archivo es ejecutado por root u otro proceso privilegiado, un atacante puede inyectar contenido malicioso y escalar privilegios.

```tip
Revisa cuidadosamente la lista producida por este control. Elimina los permisos de escritura para todos donde no sean necesarios, y asegúrate de que el sticky bit esté configurado en directorios compartidos (por ejemplo, `/tmp`).
```

## `6.1.6` — Auditar ejecutables SUID y SGID

**Qué hace apply:**
Ejecuta `find` para localizar todos los archivos con el bit setuid o setgid configurado. El control los reporta pero no elimina los bits automáticamente, porque muchos son necesarios para la funcionalidad básica del sistema (por ejemplo, `/usr/bin/passwd`, `/usr/bin/sudo`).

**Verificación manual:**

```bash
find / -xdev -type f \( -perm -4000 -o -perm -2000 \) -exec ls -l {} \; 2>/dev/null
```

**Justificación de seguridad:**
Los binarios SUID/SGID se ejecutan con los privilegios de su propietario o grupo. Una vulnerabilidad en tal binario es una ruta directa de escalada de privilegios. Las auditorías regulares aseguran que ningún programa no autorizado (por ejemplo, herramientas de explotación compiladas) haya recibido permisos elevados.

## `6.1.7` — Auditar archivos sin propietario

**Qué hace apply:**
Encuentra archivos cuyo UID o GID no se resuelva a un usuario o grupo válido en `/etc/passwd` y `/etc/group`.

**Verificación manual:**

```bash
find / -xdev -nouser -o -nogroup 2>/dev/null
```

**Justificación de seguridad:**
Los archivos propiedad de usuarios eliminados suelen ser restos de cuentas comprometidas o herramientas de ataque huérfanas. También pueden indicar una instalación de paquete mal configurada. Limpiarlos reduce el desorden y elimina posibles puertas traseras.

## `6.1.8` — Auditar IDs duplicados

**Qué hace apply:**
Comprueba `/etc/passwd`, `/etc/group`, `/etc/shadow` y `/etc/gshadow` en busca de UIDs, GIDs o nombres de usuario duplicados. Reporta duplicados pero no los corrige automáticamente.

**Verificación manual:**

```bash
awk -F: '{print $3}' /etc/passwd | sort | uniq -d
awk -F: '{print $3}' /etc/group | sort | uniq -d
```

**Justificación de seguridad:**
Los UIDs duplicados pueden causar colisiones de permisos: dos usuarios diferentes pueden compartir involuntariamente la misma identidad en el sistema de archivos, permitiendo que uno acceda a los archivos del otro.

## `6.1.9` — Asegurar permisos de cron

**Qué hace apply:**
Establece `/etc/crontab`, `/etc/cron.d/`, `/etc/cron.daily/`, `/etc/cron.weekly/` y `/etc/cron.monthly/` en modo `700` o `750` y propiedad `root:root`.

**Verificación manual:**

```bash
stat -c '%a %U:%G' /etc/crontab
stat -c '%a %U:%G' /etc/cron.d
```

**Justificación de seguridad:**
Cron es un mecanismo de ejecución persistente. Si un atacante puede escribir en un directorio de cron del sistema, logra acceso root persistente en cada intervalo programado. Restringir estas rutas a root es esencial para la integridad.
