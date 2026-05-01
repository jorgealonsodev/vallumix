# Controles de Red y Kernel

El dominio de red (sección CIS 3.1.x – 3.3.x) contiene 9 controles que endurecen la pila de red del kernel de Linux. Estos parámetros se aplican mediante `sysctl` y el subsistema de cortafuegos. Las configuraciones de red mal configuradas pueden convertir un servidor en un relé de tráfico, exponerlo a suplantación o hacerlo vulnerable a ataques de inundación de conexiones.

## `3.1.1` — Deshabilitar reenvío IP

**Qué hace apply:**
Establece `net.ipv4.ip_forward = 0` y `net.ipv6.conf.all.forwarding = 0` en `/etc/sysctl.conf` (o un drop-in bajo `/etc/sysctl.d/`) y aplica el cambio con `sysctl -p`.

**Verificación manual:**

```bash
sysctl net.ipv4.ip_forward
sysctl net.ipv6.conf.all.forwarding
```

Ambos deberían devolver `0`.

**Justificación de seguridad:**
El reenvío IP permite al kernel enrutar tráfico entre interfaces. A menos que el servidor sea explícitamente un enrutador o puerta de enlace VPN, habilitar el reenvío puede convertirlo en un relé de tráfico no deseado para atacantes. Este es uno de los controles de red de mayor severidad.

## `3.1.2` — Deshabilitar redirecciones ICMP de envío

**Qué hace apply:**
Establece `net.ipv4.conf.all.send_redirects = 0` y `net.ipv4.conf.default.send_redirects = 0`.

**Verificación manual:**

```bash
sysctl net.ipv4.conf.all.send_redirects
sysctl net.ipv4.conf.default.send_redirects
```

**Justificación de seguridad:**
Los mensajes de redirección ICMP son usados por enrutadores para informar a los hosts sobre mejores rutas. Un actor malicioso en la red local puede falsificar redirecciones para secuestrar tráfico (hombre en el medio). Deshabilitarlos previene este vector de ataque.

## `3.2.1` — Deshabilitar enrutamiento por origen

**Qué hace apply:**
Establece `net.ipv4.conf.all.accept_source_route = 0` y `net.ipv4.conf.default.accept_source_route = 0`.

**Verificación manual:**

```bash
sysctl net.ipv4.conf.all.accept_source_route
sysctl net.ipv4.conf.default.accept_source_route
```

**Justificación de seguridad:**
El enrutamiento por origen permite al remitente especificar la ruta de red exacta que los paquetes deben seguir. Los atacantes pueden usar esto para eludir reglas de cortafuegos y políticas de enrutamiento dictando su propio camino a través de la red.

## `3.2.2` — Deshabilitar aceptación de redirecciones ICMP

**Qué hace apply:**
Establece `net.ipv4.conf.all.accept_redirects = 0` y `net.ipv4.conf.default.accept_redirects = 0`.

**Verificación manual:**

```bash
sysctl net.ipv4.conf.all.accept_redirects
sysctl net.ipv4.conf.default.accept_redirects
```

**Justificación de seguridad:**
Aceptar redirecciones ICMP permite a un atacante en el mismo segmento de red redirigir el tráfico del host a través de una máquina comprometida. Este control complementa `3.1.2` al deshabilitar redirecciones en ambas direcciones.

## `3.2.3` — Deshabilitar envío de redirecciones ICMP

**Qué hace apply:**
Asegura que `net.ipv4.conf.all.send_redirects` sea `0` (se solapa con 3.1.2 pero se valida independientemente).

**Verificación manual:**

```bash
sysctl net.ipv4.conf.all.send_redirects
```

**Justificación de seguridad:**
Evita que el servidor mismo envíe redirecciones ICMP, que podrían ser falsificadas o mal utilizadas para influir en las tablas de enrutamiento de hosts pares.

## `3.2.6` — Habilitar filtrado de ruta inversa (rp_filter)

**Qué hace apply:**
Establece `net.ipv4.conf.all.rp_filter = 1` y `net.ipv4.conf.default.rp_filter = 1`.

**Verificación manual:**

```bash
sysctl net.ipv4.conf.all.rp_filter
sysctl net.ipv4.conf.default.rp_filter
```

**Justificación de seguridad:**
El filtrado de ruta inversa descarta paquetes que llegan por una interfaz diferente a la que el kernel usaría para alcanzar la dirección de origen. Esta es una defensa principal contra ataques de suplantación de IP y falsificación de direcciones de origen.

## `3.2.7` — Habilitar cookies SYN TCP

**Qué hace apply:**
Establece `net.ipv4.tcp_syncookies = 1`.

**Verificación manual:**

```bash
sysctl net.ipv4.tcp_syncookies
```

**Justificación de seguridad:**
Las cookies SYN permiten al kernel mantener conexiones TCP durante un ataque de inundación SYN sin asignar recursos para conexiones semiabiertas. Cuando el backlog se llena, el kernel envía cookies criptográficas en lugar de mantener estado, permitiendo a clientes legítimos completar el handshake mientras descarta a los atacantes.

## `3.3.1` — Asegurar que firewalld esté configurado

**Qué hace apply:**
Verifica que `firewalld` esté instalado, en ejecución, y tenga una política de denegación predeterminada. Si el servicio no está activo, Vallumix lo habilita e inicia. No define reglas específicas; el administrador debe configurar los puertos según las necesidades de la aplicación.

**Verificación manual:**

```bash
systemctl is-active firewalld
firewall-cmd --state
firewall-cmd --get-default-zone
```

**Justificación de seguridad:**
Un cortafuegos a nivel de host es la última línea de defensa contra ataques de red. Incluso si el servidor está detrás de un cortafuegos perimetral, el filtrado local protege contra movimiento lateral y reglas ascendentes mal configuradas. El modelo basado en zonas de firewalld se integra bien con entornos cloud dinámicos.

```tip
Vallumix no abre ni cierra puertos específicos porque eso depende de la aplicación. Después de aplicar este control, configura los servicios requeridos con `firewall-cmd --permanent --add-service=...`.
```
