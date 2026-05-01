# Controles

Un control es la unidad atómica de trabajo en Vallumix. Representa una verificación de seguridad individual alineada con el CIS Benchmark, junto con la lógica necesaria para evaluar el estado actual del sistema y, si es necesario, aplicar una remediación.

## ¿Qué es un control CIS?

El Center for Internet Security (CIS) publica benchmarks que desglosan la configuración segura de sistemas operativos en cientos de recomendaciones numeradas. Cada recomendación se denomina "control" y tiene un identificador como `5.2.4` (deshabilitar login directo de root por SSH) o `1.1.1.1` (deshabilitar el soporte para cramfs).

Vallumix implementa una selección de estos controles —70 en la versión 1.0— distribuidos en seis categorías funcionales:

1. **Configuración inicial:** filesystems, actualizaciones automáticas, integridad de paquetes.
2. **Servicios:** identificación y desactivación de servicios innecesarios.
3. **Red:** parámetros del kernel para TCP/IP, IPv6, ICMP, firewall.
4. **Logging y auditoría:** `rsyslog`, `journald`, `auditd`.
5. **Acceso, autenticación y autorización:** PAM, contraseñas, SSH, sudo, cron.
6. **Mantenimiento del sistema:** permisos, integridad de `/etc/passwd`, `umask`.

## El trait `Control`

En Vallumix, cada control se implementa como una estructura Rust que implementa el trait `Control`. Esta abstracción permite que el motor principal itere sobre `Box<dyn Control>` sin acoplarse a la lógica específica de cada verificación.

```rust
pub trait Control: Send + Sync {
    fn id(&self) -> &str;                          // p. ej. "5.2.4"
    fn description(&self) -> &str;
    fn severity(&self) -> Severity;
    fn applicable_distros(&self) -> &[Distro];

    fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError>;
    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError>;
    fn rollback(&self, ctx: &Context, backup: &Backup) -> Result<(), ControlError>;
}
```

Cada método tiene una responsabilidad clara:

- **`id`**: devuelve el identificador CIS del control (por ejemplo, `5.2.4`).
- **`description`**: descripción legible de lo que verifica y remedia.
- **`severity`**: nivel de criticidad del control.
- **`applicable_distros`**: lista de distribuciones donde el control tiene sentido.
- **`check`**: evalúa el estado actual sin modificar nada. Devuelve `Compliant` o `NonCompliant`.
- **`apply`**: aplica la remediación. Solo se invoca en modo `apply`, nunca en `audit`.
- **`rollback`**: revierte los cambios aplicados por este control usando el respaldo proporcionado.

## Niveles de severidad

Cada control se clasifica según el impacto potencial de su incumplimiento:

| Severidad | Significado | Ejemplo |
|---|---|---|
| **Crítica** | Brecha que permite compromiso directo del sistema | Permitir login SSH como root |
| **Alta** | Configuración insegura con impacto significativo | No deshabilitar IPv6 si no se usa |
| **Media** | Desviación que aumenta la superficie de ataque | Servicios innecesarios activos |
| **Baja** | Recomendación de buena práctica | Configuración de banner MOTD |

## Organización en el código

Los controles se organizan en `vallumix-controls` por dominio funcional:

```
vallumix-controls/src/
├── filesystem/
│   ├── mod.rs
│   └── disable_cramfs.rs      # CIS 1.1.1.1
├── ssh/
│   ├── mod.rs
│   └── disable_root_login.rs  # CIS 5.2.4
├── network/
│   └── ...
└── ...
```

Cada archivo implementa una única estructura de control y no debe exceder las 200 líneas de código efectivo. Si un control requiere lógica compleja, se descompone en funciones auxiliares privadas.

```tip
Puedes listar todos los controles disponibles con `vallumix list --profile web`. Esto muestra el ID CIS, la descripción, la severidad y el estado de cumplimiento actual sin aplicar cambios.
```
