# Controles de Hardening del Sistema de Archivos

El dominio de sistema de archivos (sección CIS 1.1.x) contiene 9 controles que reducen la superficie de ataque deshabilitando módulos del kernel de sistemas de archivos no utilizados y endureciendo las opciones de montaje temporales. Los controladores de sistema de archivos no utilizados pueden ser explotados para cargar medios maliciosos o escapar de contenedores; deshabilitarlos elimina una clase de ataques a nivel de kernel.

## `1.1.1.1` — Deshabilitar cramfs

**Qué hace apply:**
Vallumix comprueba si el módulo del kernel `cramfs` es cargable. Si está presente, crea o actualiza `/etc/modprobe.d/cramfs.conf` con `install cramfs /bin/true` y ejecuta `rmmod cramfs` si el módulo está actualmente cargado.

**Verificación manual:**

```bash
modprobe -n -v cramfs
lsmod | grep cramfs
```

Se espera: el primer comando debe mostrar `/bin/true`, y el segundo no debe devolver nada.

**Justificación de seguridad:**
cramfs es un sistema de archivos de solo lectura comprimido raramente utilizado en servidores modernos. Mantener el módulo disponible permite a un atacante con acceso local montar imágenes manipuladas que podrían contener cargas maliciosas. Deshabilitarlo sigue el principio de mínima funcionalidad.

## `1.1.1.2` — Deshabilitar freevxfs

**Qué hace apply:**
Crea `/etc/modprobe.d/freevxfs.conf` con `install freevxfs /bin/true` y descarga el módulo si está activo.

**Verificación manual:**

```bash
modprobe -n -v freevxfs
lsmod | grep freevxfs
```

**Justificación de seguridad:**
freevxfs es el controlador de compatibilidad con VxFS de FreeBSD. No tiene uso legítimo en servidores Linux y representa un punto de entrada innecesario en el kernel.

## `1.1.1.3` — Deshabilitar jffs2

**Qué hace apply:**
Instala una lista negra de modprobe para `jffs2` (Journalling Flash File System v2) y lo elimina de la memoria.

**Verificación manual:**

```bash
modprobe -n -v jffs2
lsmod | grep jffs2
```

**Justificación de seguridad:**
jffs2 está diseñado para dispositivos de memoria flash raw como sistemas embebidos. En un servidor con almacenamiento en bloque, este módulo no tiene propósito y podría ser abusado para interactuar con medios físicos.

## `1.1.1.4` — Deshabilitar hfs

**Qué hace apply:**
Añade a la lista negra el módulo `hfs` (Apple Hierarchical File System) mediante modprobe y lo descarga.

**Verificación manual:**

```bash
modprobe -n -v hfs
lsmod | grep hfs
```

**Justificación de seguridad:**
hfs permite montar volúmenes con formato legado de Apple. Los servidores no necesitan esta capacidad; eliminarlo evita que un atacante introduzca o extraiga datos mediante medios removibles con formato HFS.

## `1.1.1.5` — Deshabilitar hfsplus

**Qué hace apply:**
Añade a la lista negra el módulo `hfsplus` y lo descarga si está activo.

**Verificación manual:**

```bash
modprobe -n -v hfsplus
lsmod | grep hfsplus
```

**Justificación de seguridad:**
hfsplus es el controlador del sistema de archivos moderno de Apple. Se aplica el mismo razonamiento que para hfs: ninguna carga de trabajo de servidor requiere soporte nativo para sistemas de archivos de Apple.

## `1.1.1.6` — Deshabilitar squashfs

**Qué hace apply:**
Añade a la lista negra el módulo `squashfs`. Nota: algunos runtimes de contenedores usan squashfs internamente; verifica tu carga de trabajo antes de aplicar este control.

**Verificación manual:**

```bash
modprobe -n -v squashfs
lsmod | grep squashfs
```

**Justificación de seguridad:**
squashfs es un sistema de archivos comprimido de solo lectura usado en CDs en vivo y algunas capas de contenedores. Si tu servidor no depende de paquetes snap o tecnologías similares, deshabilitarlo elimina otro vector de montaje.

```warning
Algunas distribuciones usan squashfs para paquetes snap. Verifica que `snap list` esté vacío o no sea necesario antes de aplicar este control.
```

## `1.1.1.7` — Deshabilitar udf

**Qué hace apply:**
Añade a la lista negra el módulo `udf` (Universal Disk Format, usado por DVD-ROMs).

**Verificación manual:**

```bash
modprobe -n -v udf
lsmod | grep udf
```

**Justificación de seguridad:**
udf permite leer medios ópticos. Los servidores sin unidades ópticas no tienen uso para este módulo, y podría ser explotado para montar imágenes UDF maliciosas.

## `1.1.10` — Deshabilitar almacenamiento USB

**Qué hace apply:**
Crea `/etc/modprobe.d/usb-storage.conf` con `install usb-storage /bin/true` y elimina el módulo de la memoria.

**Verificación manual:**

```bash
modprobe -n -v usb-storage
lsmod | grep usb-storage
```

**Justificación de seguridad:**
Este es uno de los controles de sistema de archivos de mayor impacto. Deshabilitar `usb-storage` evita que el kernel reconozca dispositivos de almacenamiento USB masivo, bloqueando un vector de ataque físico común (por ejemplo, unidades USB maliciosas insertadas en una sala de servidores o un host virtualizado con passthrough USB).

## `1.1.2.1` — Endurecer opciones de montaje de tmpfs

**Qué hace apply:**
Asegura que `/tmp`, `/var/tmp` y `/dev/shm` estén montados con las opciones `noexec`, `nosuid` y `nodev`. Si aún no están montados mediante fstab o unidades de montaje de systemd, Vallumix actualiza `/etc/fstab` o crea una unidad de montaje de anulación.

**Verificación manual:**

```bash
findmnt -n -o OPTIONS /tmp
findmnt -n -o OPTIONS /var/tmp
findmnt -n -o OPTIONS /dev/shm
```

Cada una debe contener `noexec`, `nosuid` y `nodev`.

**Justificación de seguridad:**
Los directorios temporales son frecuentemente utilizados por atacantes para depositar y ejecutar cargas. El flag `noexec` previene la ejecución de binarios desde estas rutas, `nosuid` bloquea la escalada mediante setuid, y `nodev` previene el abuso de archivos de dispositivo. Juntos restringen significativamente la actividad post-explotación.
