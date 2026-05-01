# Referencia CLI

## Opciones Globales

| Opción | Corto | Descripción | Por Defecto |
|--------|-------|-------------|-------------|
| `--profile` | | Perfil a utilizar | `web` |
| `--dry-run` | | Previsualizar cambios sin aplicar | `false` |
| `--verbose` | `-v` | Habilitar logging de depuración | `false` |
| `--quiet` | `-q` | Suprimir salida no esencial | `false` |
| `--threshold` | | Umbral de cumplimiento (0-100) | `80` |
| `--no-color` | | Deshabilitar salida coloreada | `false` |
| `--report` | | Formato(s) de informe: html, json, junit, text | |
| `--output` | | Ruta del archivo de salida | |

## Subcomandos

### `apply`

Aplica controles de endurecimiento para el perfil seleccionado.

```bash
vallumix apply --profile web --dry-run
```

Requiere privilegios root a menos que se especifique `--dry-run`.

### `audit`

Audita el sistema contra el perfil seleccionado sin realizar cambios.

```bash
vallumix audit --profile web --report html --output report.html
```

### `rollback`

Revierte cambios de una sesión o control anterior.

```bash
vallumix rollback --session <id>
vallumix rollback --control-id 5.2.4
```

### `list`

Lista todos los controles disponibles para un perfil.

```bash
vallumix list --profile database
```

### `completion`

Genera completados de shell.

```bash
vallumix completion bash
vallumix completion zsh
vallumix completion fish
vallumix completion nushell
```

## Códigos de Salida

| Código | Significado |
|--------|-------------|
| `0` | Éxito / cumplimiento por encima del umbral |
| `1` | Cumplimiento por debajo del umbral |
| `2` | Error de ejecución |
| `3` | Error de privilegios (se requiere root) |
