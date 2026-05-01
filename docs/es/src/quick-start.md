# Inicio Rápido

Esta guía te ayudará a ejecutar tu primera auditoría y aplicar tu primer perfil de endurecimiento.

## Ejecutar una Auditoría

Una auditoría verifica tu sistema contra un perfil sin realizar cambios:

```bash
vallumix audit --profile web
```

Esto generará un informe de cumplimiento en stdout. Para guardarlo en un archivo:

```bash
vallumix audit --profile web --report html --output report.html
```

## Previsualizar Cambios con Dry-Run

Antes de aplicar cualquier endurecimiento, previsualiza lo que cambiaría:

```bash
sudo vallumix apply --profile web --dry-run
```

## Aplicar Endurecimiento

Una vez satisfecho con la previsualización, aplica el perfil:

```bash
sudo vallumix apply --profile web
```

Vallumix creará automáticamente una sesión de backup, permitiéndote hacer rollback más tarde si es necesario.

## Listar Controles Disponibles

Para ver todos los controles incluidos en un perfil:

```bash
vallumix list --profile database
```

## Revertir una Sesión

Si necesitas deshacer cambios:

```bash
vallumix rollback --session <session-id>
```

Puedes encontrar los IDs de sesión en la salida del comando apply o verificando el directorio de backup.
