# Controles de Deshabilitación de Servicios

El dominio de servicios (sección CIS 2.2.x) contiene 12 controles que detienen, deshabilitan y enmascaran servicios orientados a la red que no son necesarios para el rol del servidor. Cada demonio en ejecución es una potencial fuente de vulnerabilidades; eliminar servicios innecesarios es una de las formas más efectivas de reducir la superficie de ataque.

## `2.2.2` — Deshabilitar CUPS

**Qué hace apply:**
Detiene el servicio `cups`, lo deshabilita del inicio en el arranque, y enmascara la unidad para que no pueda iniciarse manualmente o por otro servicio.

**Verificación manual:**

```bash
systemctl is-enabled cups
systemctl status cups
```

Se espera: `masked` o `disabled`, y el servicio debe estar inactivo.

**Justificación de seguridad:**
CUPS (Common UNIX Printing System) expone puertos de red para descubrimiento de impresoras y envío de trabajos. Los servidores raramente necesitan capacidad de impresión, y CUPS tiene un historial de vulnerabilidades explotables remotamente.

## `2.2.3` — Deshabilitar Avahi

**Qué hace apply:**
Detiene, deshabilita y enmascara el servicio `avahi-daemon`.

**Verificación manual:**

```bash
systemctl is-enabled avahi-daemon
systemctl status avahi-daemon
```

**Justificación de seguridad:**
Avahi implementa mDNS/DNS-SD (Bonjour) para descubrimiento de servicios en la red local. Multidifunde información del host y de servicios, lo que filtra la superficie de ataque a cualquiera en el segmento de red local, y ha sido objetivo de ataques de amplificación.

## `2.2.4` — Deshabilitar servidor DHCP

**Qué hace apply:**
Detiene, deshabilita y enmascara el servicio de servidor DHCP (por ejemplo, `isc-dhcp-server`, `dhcpd`).

**Verificación manual:**

```bash
systemctl is-enabled dhcpd 2>/dev/null || systemctl is-enabled isc-dhcp-server
```

**Justificación de seguridad:**
Un servidor DHCP solo debe ejecutarse en infraestructura de red dedicada. Dejarlo habilitado accidentalmente en un servidor puede llevar a ataques de DHCP rogue, secuestro de red y incorporación no autorizada de dispositivos.

## `2.2.5` — Deshabilitar servidor LDAP

**Qué hace apply:**
Detiene, deshabilita y enmascara el servicio de servidor LDAP (por ejemplo, `slapd`).

**Verificación manual:**

```bash
systemctl is-enabled slapd
```

**Justificación de seguridad:**
A menos que el servidor sea explícitamente un proveedor de identidad, un servidor LDAP expone información de directorio y tráfico de autenticación que debería aislarse en servicios de directorio dedicados.

## `2.2.6` — Deshabilitar servidor NFS

**Qué hace apply:**
Detiene, deshabilita y enmascara los servicios de servidor NFS (`nfs-server`, `rpc-nfsd`).

**Verificación manual:**

```bash
systemctl is-enabled nfs-server
```

**Justificación de seguridad:**
Las exportaciones NFS pueden filtrar datos sensibles si están mal configuradas. El protocolo NFS históricamente ha sufrido problemas de suplantación de UID y autenticación débil. Solo los servidores de archivos dedicados deberían ejecutar NFS.

## `2.2.7` — Deshabilitar rpcbind

**Qué hace apply:**
Detiene, deshabilita y enmascara `rpcbind`, el asignador de puertos RPC requerido por NFS y otros servicios RPC legados.

**Verificación manual:**

```bash
systemctl is-enabled rpcbind
```

**Justificación de seguridad:**
`rpcbind` mapea números de programa RPC a puertos de red. Es esencial para NFSv3 pero innecesario en servidores modernos. Ha sido usado en ataques DDoS de amplificación y proporciona información sobre servicios RPC en ejecución a escáneres remotos.

## `2.2.8` — Deshabilitar servidor DNS (BIND)

**Qué hace apply:**
Detiene, deshabilita y enmascara `named` (BIND) o el paquete de servidor DNS específico de la distribución.

**Verificación manual:**

```bash
systemctl is-enabled named
```

**Justificación de seguridad:**
Ejecutar un servidor DNS recursivo o autoritativo en un servidor de propósito general aumenta la superficie de ataque y el riesgo de envenenamiento de caché DNS o abuso de amplificación. El DNS debería delegarse a resolvedores dedicados.

## `2.2.9` — Deshabilitar vsftpd

**Qué hace apply:**
Detiene, deshabilita y enmascara el servidor FTP `vsftpd`.

**Verificación manual:**

```bash
systemctl is-enabled vsftpd
```

**Justificación de seguridad:**
FTP transmite credenciales y datos en texto plano. Los flujos de trabajo modernos deberían usar SFTP (sobre SSH) o HTTPS. Un servidor FTP habilitado accidentalmente es una fuente frecuente de fugas de datos.

## `2.2.10` — Deshabilitar servidor HTTP

**Qué hace apply:**
Detiene, deshabilita y enmascara el servicio `httpd` o `apache2`.

**Verificación manual:**

```bash
systemctl is-enabled httpd 2>/dev/null || systemctl is-enabled apache2
```

**Justificación de seguridad:**
Este control se incluye en los perfiles `database` y `bastion` porque esos roles no deberían servir contenido web. Solo el perfil `web` omite este control, asumiendo que el administrador quiere explícitamente un servidor web.

## `2.2.11` — Deshabilitar Dovecot

**Qué hace apply:**
Detiene, deshabilita y enmascara el servidor de correo IMAP/POP3 `dovecot`.

**Verificación manual:**

```bash
systemctl is-enabled dovecot
```

**Justificación de seguridad:**
Los protocolos de acceso a correo deberían centralizarse en servidores de correo dedicados. Ejecutar Dovecot en un servidor web o de bases de datos expone innecesariamente credenciales de correo y almacenamiento.

## `2.2.14` — Deshabilitar demonio SNMP

**Qué hace apply:**
Detiene, deshabilita y enmascara `snmpd`.

**Verificación manual:**

```bash
systemctl is-enabled snmpd
```

**Justificación de seguridad:**
SNMP (especialmente v1 y v2c) usa cadenas de comunidad que a menudo se dejan en valores predeterminados como `public`. Incluso SNMPv3 requiere una gestión cuidadosa de claves. Si se necesita monitoreo, use métricas sin agente (Prometheus node_exporter, Telegraf) en su lugar.

## `2.2.15` — Deshabilitar demonio rsync

**Qué hace apply:**
Detiene, deshabilita y enmascara el servicio `rsyncd`.

**Verificación manual:**

```bash
systemctl is-enabled rsync
```

**Justificación de seguridad:**
El demonio rsync expone un puerto TCP (873) sin cifrado por defecto. Aunque rsync sobre SSH es seguro y ampliamente utilizado, el demonio independiente no debería ejecutarse a menos que se requiera explícitamente para un espejo público.
