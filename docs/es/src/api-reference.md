# Referencia API

Vallumix está organizado como un workspace de Cargo con múltiples crates. La API pública está documentada usando rustdoc.

## Generar Documentación API

Puedes generar la documentación API localmente:

```bash
cargo doc --no-deps --workspace
```

Luego abre `target/doc/vallumix_cli/index.html` en tu navegador.

## Visión General de Crates

### `vallumix-core`

Define los traits y tipos principales:
- `Control`: Trait para implementar controles CIS
- `Reporter`: Trait para generación de informes
- `Profile`: Trait para definiciones de perfil
- `VallumixError`: Tipo de error principal

### `vallumix-controls`

Contiene las implementaciones concretas de controles CIS organizados por categoría:
- Endurecimiento SSH
- Gestión de usuarios
- Configuración de red
- Permisos de sistema de archivos

### `vallumix-reporters`

Generadores de informes para diferentes formatos de salida:
- `HtmlReporter`
- `JsonReporter`
- `JunitReporter`
- `TextReporter`

### `vallumix-backup`

Funcionalidad de backup y rollback:
- `BackupManager`: Gestiona sesiones de backup
- `BackupSession`: Representa una sesión de endurecimiento

### `vallumix-cli`

La interfaz de línea de comandos y el binario principal.

## Documentación en Línea

La documentación API alojada está disponible en:
[https://docs.rs/vallumix-cli](https://docs.rs/vallumix-cli)
