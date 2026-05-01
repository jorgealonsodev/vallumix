# Catálogo de Controles

Vallumix implementa **70 controles del CIS Benchmark** organizados en **7 dominios de seguridad**. Cada control es una unidad atómica de verificación y remediación que comprueba una configuración específica y, si no cumple, aplica el cambio de hardening recomendado.

Los controles siguen el esquema de numeración del CIS Benchmark (por ejemplo, `1.1.1.1` para sistema de archivos, `5.2.4` para SSH). Cada control implementa el trait `Control`: verifica el estado actual, aplica la remediación si es necesario, y verifica de nuevo para confirmar la corrección.

## Los 7 Dominios

| Dominio | Conteo | Sección CIS | Enfoque |
|---------|--------|-------------|---------|
| **Sistema de archivos** | 9 | 1.1.x | Deshabilitar módulos de sistema de archivos no utilizados, endurecer opciones de montaje |
| **Servicios** | 12 | 2.2.x | Detener y enmascarar servicios de red innecesarios |
| **Red** | 9 | 3.1.x – 3.3.x | Parámetros de red del kernel, configuración del cortafuegos |
| **Registro y auditoría** | 11 | 4.1.x | rsyslog, journald, auditd, rotación de logs, permisos de archivos |
| **SSH** | 11 | 5.2.x | Hardening del servidor SSH: protocolo, criptografía, acceso, sesión |
| **Acceso y autenticación** | 10 | 5.1.x, 5.3.x – 5.5.x | PAM, política de contraseñas, umask, tiempo de espera del shell |
| **Mantenimiento del sistema** | 8 | 6.1.x | Permisos de archivos críticos, auditoría SUID/SGID, seguridad de cron |

```tip
No todos los controles se aplican a todos los roles de servidor. Utiliza un **perfil** (`web`, `database`, `bastion`) para seleccionar solo los controles relevantes para la función de tu servidor. Los perfiles se definen en archivos TOML bajo `profiles/`.
```

## Tabla de Referencia de Controles

La siguiente tabla lista una muestra representativa de los 70 controles implementados por Vallumix. Los niveles de severidad se asignan en función de las recomendaciones del CIS y del impacto potencial del incumplimiento.

| CIS ID | Descripción | Severidad | Perfiles | Distros | NIST 800-53 | ISO 27001 |
|--------|-------------|-----------|----------|---------|-------------|-----------|
| `1.1.1.1` | Deshabilitar módulo de sistema de archivos cramfs | Baja | web, database, bastion | Todas | CM-7 | A.8.1 |
| `1.1.1.7` | Deshabilitar módulo de sistema de archivos udf | Baja | web, database, bastion | Todas | CM-7 | A.8.1 |
| `1.1.10` | Deshabilitar módulo de almacenamiento USB | Media | web, database, bastion | Todas | MP-7 | A.8.10 |
| `2.2.3` | Deshabilitar demonio Avahi mDNS/DNS-SD | Media | web, database, bastion | Todas | CM-7 | A.8.1 |
| `2.2.8` | Deshabilitar servidor DNS (named) | Media | database, bastion | Todas | CM-7 | A.8.1 |
| `2.2.14` | Deshabilitar demonio SNMP | Media | web, database, bastion | Todas | CM-7 | A.13.1 |
| `3.1.1` | Deshabilitar reenvío IP | Alta | web, database, bastion | Todas | SC-7 | A.13.1 |
| `3.2.7` | Habilitar cookies SYN TCP | Media | web, database, bastion | Todas | SC-5 | A.13.1 |
| `3.3.1` | Asegurar que firewalld esté configurado | Alta | web, database, bastion | Todas | SC-7 | A.13.1 |
| `4.1.1.1` | Asegurar que rsyslog esté instalado | Media | web, database, bastion | Todas | AU-6 | A.12.4 |
| `4.1.3.1` | Asegurar que auditd esté instalado | Media | web, database, bastion | Todas | AU-6 | A.12.4 |
| `5.2.4` | Deshabilitar login de root por SSH | Alta | web, database, bastion | Todas | IA-2 | A.9.2 |
| `5.2.4b` | Establecer MaxAuthTries de SSH en 4 o menos | Media | web, database, bastion | Todas | IA-6 | A.9.4 |
| `5.3.4` | Asegurar que PAM faillock esté configurado | Media | web, database, bastion | Todas | AC-7 | A.9.4 |
| `6.1.1` | Asegurar permisos en /etc/passwd | Media | web, database, bastion | Todas | AC-3 | A.9.1 |
| `6.1.6` | Auditar ejecutables SUID y SGID | Media | web, database, bastion | Todas | AC-3 | A.9.1 |

## Cómo Usar Este Catálogo

- **Navegar por dominio**: la barra lateral agrupa los controles en los 7 dominios listados arriba.
- **Buscar por CIS ID**: cada página lista los controles en orden CIS con su ID exacto.
- **Comprobar cobertura del perfil**: la columna `Perfiles` indica qué roles de servidor incluyen el control.
- **Verificar manualmente**: cada página de control incluye el comando de verificación manual para que puedas confirmar el trabajo de Vallumix de forma independiente.

```tip
Vallumix es idempotente: ejecutar `apply` varias veces con el mismo perfil produce el mismo estado final. Los controles que ya cumplen se omiten automáticamente.
```

Para la lista completa de los 70 controles, inspecciona el código fuente en `crates/vallumix-controls/src/lib.rs` o las definiciones de perfil en `profiles/web.toml`, `profiles/database.toml` y `profiles/bastion.toml`.
