# Controles de Acceso, Autenticación y Autorización

El dominio de acceso y autenticación cubre el hardening de SSH (sección CIS 5.2.x) y la política de autenticación a nivel de sistema (PAM, umask, tiempo de espera del shell — secciones CIS 5.1.x, 5.3.x – 5.5.x). Con 21 controles en total entre SSH y autenticación, este es el dominio más crítico para prevenir accesos no autorizados. Un único parámetro SSH mal configurado puede exponer el servidor a fuerza bruta, relleno de credenciales o compromiso remoto completo.

## Controles de SSH

### `5.2.1` — Asegurar que se usa el Protocolo 2 de SSH

**Qué hace apply:**
Establece `Protocol 2` en `/etc/ssh/sshd_config`. El Protocolo 1 es obsoleto y criptográficamente roto.

**Verificación manual:**

```bash
grep -E '^Protocol' /etc/ssh/sshd_config
```

Se espera: `Protocol 2` o la línea ausente (OpenSSH moderno usa 2 por defecto).

**Justificación de seguridad:**
El Protocolo 1 de SSH carece de comprobaciones de integridad, usa CRC-32 débil para validación de paquetes y es vulnerable a ataques de inserción. El Protocolo 2 introduce integridad criptográfica robusta e intercambio de claves seguro.

### `5.2.2` — Establecer LogLevel de SSH a INFO

**Qué hace apply:**
Establece `LogLevel INFO` en `sshd_config` para asegurar que los intentos de inicio de sesión, intercambios de claves y fallos de autenticación se registren.

**Verificación manual:**

```bash
grep '^LogLevel' /etc/ssh/sshd_config
```

**Justificación de seguridad:**
`INFO` captura suficiente detalle para detección de intrusiones e investigación forense sin la verbosidad de `DEBUG`. `QUIET` o `FATAL` ocultarían patrones de fuerza bruta.

### `5.2.3` — Deshabilitar contraseñas vacías

**Qué hace apply:**
Establece `PermitEmptyPasswords no` en `sshd_config`.

**Verificación manual:**

```bash
grep '^PermitEmptyPasswords' /etc/ssh/sshd_config
```

**Justificación de seguridad:**
Las contraseñas vacías permiten el inicio de sesión sin ninguna credencial. Incluso en redes internas, esto es inaceptable porque evade toda la capa de autenticación.

### `5.2.4` — Deshabilitar login de root

**Qué hace apply:**
Establece `PermitRootLogin no` (o `prohibit-password`) en `sshd_config` y reinicia el servicio SSH.

**Verificación manual:**

```bash
grep '^PermitRootLogin' /etc/ssh/sshd_config
```

**Justificación de seguridad:**
El login de root es el objetivo de mayor valor para ataques de fuerza bruta. Deshabilitarlo obliga a los atacantes a adivinar tanto un nombre de usuario como una contraseña, duplicando el espacio de búsqueda. También asegura que todas las acciones administrativas sean atribuibles a usuarios individuales mediante logs de sudo.

```danger
Este control puede bloquearte si no tienes acceso por clave como usuario no root. Verifica que `ssh -o PasswordAuthentication=no usuario@host` funcione antes de aplicar.
```

### `5.2.4b` — Establecer MaxAuthTries en 4 o menos

**Qué hace apply:**
Establece `MaxAuthTries 4` en `sshd_config`.

**Verificación manual:**

```bash
grep '^MaxAuthTries' /etc/ssh/sshd_config
```

**Justificación de seguridad:**
Limitar los intentos de autenticación por conexión ralentiza los ataques de fuerza bruta. Después de 4 fallos, la conexión se cierra, obligando al atacante a establecer una nueva sesión TCP.

### `5.2.6` — Establecer ClientAliveInterval

**Qué hace apply:**
Establece `ClientAliveInterval 300` y `ClientAliveCountMax 0` para desconectar sesiones inactivas después de 5 minutos.

**Verificación manual:**

```bash
grep -E '^ClientAlive' /etc/ssh/sshd_config
```

**Justificación de seguridad:**
Las sesiones inactivas dejadas abiertas en estaciones de trabajo compartidas o multiplexadores de terminal olvidados proporcionan una puerta abierta para cualquiera con acceso físico. La desconexión automática limita esta ventana.

### `5.2.8` — Limitar acceso SSH (AllowUsers / AllowGroups)

**Qué hace apply:**
Si el administrador ha configurado `AllowUsers` o `AllowGroups` en los metadatos del perfil, Vallumix escribe esos valores en `sshd_config`. Por defecto, solo verifica que la directiva esté presente.

**Verificación manual:**

```bash
grep -E '^AllowUsers|^AllowGroups' /etc/ssh/sshd_config
```

**Justificación de seguridad:**
Restringir SSH a usuarios o grupos específicos evita que cuentas deshabilitadas, terminadas o comprometidas accedan al servidor. Es especialmente importante en hosts bastión.

### `5.2.9` — Establecer banner de SSH

**Qué hace apply:**
Establece `Banner /etc/ssh/banner` y crea el archivo de banner si no existe.

**Verificación manual:**

```bash
grep '^Banner' /etc/ssh/sshd_config
```

**Justificación de seguridad:**
Un banner de inicio de sesión establece aviso legal y desalienta el acceso no autorizado. En algunas jurisdicciones, un banner es requerido para el enjuiciamiento exitoso bajo estatutos de delitos informáticos.

## Controles de Autenticación (PAM)

### `5.3.1` — Asegurar que la calidad de contraseñas PAM está habilitada

**Qué hace apply:**
Verifica que `pam_pwquality.so` esté referenciado en `/etc/pam.d/system-auth` o `/etc/pam.d/common-password`.

**Verificación manual:**

```bash
grep pam_pwquality /etc/pam.d/system-auth /etc/pam.d/common-password 2>/dev/null
```

**Justificación de seguridad:**
pam_pwquality impone reglas mínimas de complejidad (longitud, clases de caracteres, comprobaciones de diccionario) en el momento de creación o cambio de contraseña. Sin él, los usuarios pueden establecer contraseñas triviales que caen ante ataques de diccionario en segundos.

### `5.3.2` — Asegurar longitud mínima de contraseña PAM

**Qué hace apply:**
Establece `minlen = 14` en `/etc/security/pwquality.conf`.

**Verificación manual:**

```bash
grep '^minlen' /etc/security/pwquality.conf
```

**Justificación de seguridad:**
Las contraseñas más largas aumentan exponencialmente el espacio de búsqueda de fuerza bruta. Una frase de contraseña de 14 caracteres con clases de caracteres mixtas es actualmente considerada el mínimo para resistir el cracking offline de hashes.

### `5.3.4` — Asegurar que faillock de PAM está configurado

**Qué hace apply:**
Añade líneas `pam_faillock.so` a la pila de autenticación con `deny=5`, `unlock_time=900` y `even_deny_root`.

**Verificación manual:**

```bash
grep pam_faillock /etc/pam.d/system-auth /etc/pam.d/password-auth 2>/dev/null
```

**Justificación de seguridad:**
faillock bloquea temporalmente cuentas tras repetidos fallos de autenticación. Esto detiene bots de fuerza bruta automatizados sin deshabilitar permanentemente la cuenta, dando tiempo a los administradores para reaccionar.

### `5.3.5` — Asegurar historial de contraseñas PAM

**Qué hace apply:**
Establece `remember = 5` en la configuración del módulo de contraseñas PAM para prevenir la reutilización de las últimas 5 contraseñas.

**Verificación manual:**

```bash
grep 'remember' /etc/pam.d/system-auth /etc/pam.d/common-password 2>/dev/null
```

**Justificación de seguridad:**
Sin aplicación de historial, los usuarios ciclan entre un pequeño conjunto de contraseñas. Si una contraseña antigua se filtra en una brecha, el atacante a menudo puede iniciar sesión porque el usuario ha vuelto a ella.

### `5.5.1` — Asegurar que la umask predeterminada es restrictiva

**Qué hace apply:**
Establece `umask 027` en `/etc/profile`, `/etc/bashrc` y `/etc/login.defs`.

**Verificación manual:**

```bash
grep -E '^umask' /etc/profile /etc/bashrc /etc/login.defs 2>/dev/null
```

**Justificación de seguridad:**
Una umask de `027` asegura que los archivos nuevos se creen con permisos `640` y los directorios con `750`, evitando que otros usuarios lean datos recién creados por defecto.

### `5.5.2` — Asegurar que el tiempo de espera del shell está configurado

**Qué hace apply:**
Añade `TMOUT=900` y `readonly TMOUT` a `/etc/profile` y `/etc/bashrc`.

**Verificación manual:**

```bash
grep 'TMOUT' /etc/profile /etc/bashrc 2>/dev/null
```

**Justificación de seguridad:**
Los tiempos de espera del shell cierran automáticamente sesiones locales e SSH inactivas después de 15 minutos. Esto complementa a `ClientAliveInterval` y protege contra acceso físico o ventanas de terminal abandonadas.
