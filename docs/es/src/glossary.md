# Glosario

Este glosario define los términos técnicos utilizados a lo largo de la documentación de Vallumix, basados en el PRD y en la terminología estándar de la industria de ciberseguridad.

## Términos

### Hardening
Proceso de reducción de la superficie de ataque de un sistema mediante la desactivación de funcionalidades innecesarias y la configuración segura de las restantes. En Vallumix, el hardening se automatiza aplicando controles CIS.

### CIS Benchmark
Guías de configuración segura mantenidas por el Center for Internet Security (CIS). Son referencias industriales ampliamente reconocidas que definen configuraciones seguras para sistemas operativos, aplicaciones y dispositivos de red.

### Idempotencia
Propiedad por la cual ejecutar una operación una o múltiples veces produce el mismo resultado final. Vallumix garantiza idempotencia mediante pre-checks: si un control ya cumple, no se modifica nada.

### Bastión (jump host)
Servidor expuesto a internet cuyo único propósito es servir como punto de entrada autenticado hacia infraestructura interna. El perfil `bastion` de Vallumix aplica el hardening más agresivo, orientado exclusivamente a SSH.

### Dry-run
Modo de ejecución que simula los efectos de una operación sin realizarlos efectivamente. En Vallumix, `--dry-run` ejecuta todos los checks y reporta qué cambios se harían, pero no crea respaldos ni modifica archivos.

### MSRV (Minimum Supported Rust Version)
Versión mínima del compilador de Rust que el proyecto se compromete a soportar. Para Vallumix, la MSRV es 1.75.

### Crate
Unidad de compilación y distribución en Rust, equivalente conceptual a un paquete. Vallumix se organiza como un workspace de Cargo con múltiples crates: `vallumix-core`, `vallumix-controls`, `vallumix-reporters`, `vallumix-backup` y `vallumix-cli`.

### Workspace
Agrupación de múltiples crates relacionados gestionados conjuntamente por Cargo. El workspace de Vallumix permite compilar y testear todos los crates desde la raíz del proyecto.

### Trait
Mecanismo de Rust para definir comportamiento compartido, similar a las interfaces en otros lenguajes. El trait `Control` abstrae el ciclo de vida de cada verificación CIS en Vallumix.

### SLSA (Supply-chain Levels for Software Artifacts)
Marco de Google para asegurar la cadena de suministro de software. Vallumix publica attestations SLSA Level 3 en sus releases para garantizar la proveniencia y integridad del binario.

### Control
Unidad atómica de verificación y remediación en Vallumix. Cada control implementa el trait `Control` y representa una recomendación específica del CIS Benchmark (por ejemplo, "deshabilitar login de root por SSH").

### Perfil
Conjunto preconfigurado de controles adaptado al rol de un servidor. Vallumix incluye tres perfiles: `web`, `database` y `bastion`.

### Rollback
Operación de reversión que restaura configuraciones previas desde respaldos versionados. Vallumix permite rollback por control individual o por sesión completa.

### Respaldos versionados
Copias de seguridad organizadas por sesión de ejecución, identificadas por timestamp. Cada sesión incluye el archivo modificado, un manifesto de metadatos y checksums SHA-256.

### Threshold (umbral)
Porcentaje mínimo de cumplimiento configurable. Si la tasa de cumplimiento de una ejecución está por debajo del umbral, Vallumix devuelve código de salida `1`.

### Reporter
Trait que abstrae la generación de reportes en diferentes formatos. Vallumix implementa reporters para HTML, JSON, JUnit XML y texto plano.

### Superficie de ataque
Conjunto de puntos de entrada que un atacante podría explotar en un sistema. El objetivo del hardening es reducir esta superficie al mínimo necesario para la funcionalidad operativa.

```tip
Si encuentras un término en la documentación que no está en este glosario, abre un issue para solicitar su inclusión. La documentación debe ser accesible tanto para administradores experimentados como para quienes se inician en hardening de sistemas.
```
