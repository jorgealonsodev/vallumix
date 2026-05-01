# Comando `list`

El subcomando `list` muestra el catálogo completo de controles disponibles en Vallumix, filtrados por perfil. Es útil para explorar qué controles se ejecutarían antes de lanzar un `apply` o `audit`.

## Sintaxis

```bash
vallumix list [FLAGS]
```

## Flags específicas de list

| Flag | Descripción |
|---|---|
| `--profile` | **Obligatorio.** Filtra los controles por perfil (`web`, `database`, `bastion`). |
| `--no-color` | Deshabilita colores en la salida. |

## Formato de salida

La salida por defecto es una tabla en terminal con las siguientes columnas:

```text
ID      | SEVERITY | PROFILE    | DESCRIPTION
--------|----------|------------|------------------------------------------
1.1.1.1 | Low      | web,db,ba  | Disable cramfs support
1.1.1.2 | Low      | web,db,ba  | Disable freevxfs support
5.2.4   | Critical | web,db,ba  | Ensure SSH root login is disabled
3.4.1   | Medium   | web,db     | Ensure firewalld is installed
```

### Columnas

- **ID:** identificador CIS del control (por ejemplo, `5.2.4`).
- **SEVERITY:** nivel de criticidad (`Critical`, `High`, `Medium`, `Low`).
- **PROFILE:** perfiles que incluyen este control (`web`, `database`, `bastion`, abreviados como `w`, `db`, `ba`).
- **DESCRIPTION:** descripción legible de la recomendación.

## Ejemplos

### Listar controles del perfil web

```bash
vallumix list --profile web
```

### Listar controles del perfil bastión sin colores

```bash
vallumix list --profile bastion --no-color
```

### Filtrar controles críticos con grep

```bash
vallumix list --profile web --no-color | grep Critical
```

### Contar controles por perfil

```bash
vallumix list --profile web --no-color | wc -l
vallumix list --profile database --no-color | wc -l
vallumix list --profile bastion --no-color | wc -l
```

```tip
Usa `vallumix list` como paso de descubrimiento en scripts de aprovisionamiento. Antes de aplicar un perfil, lista sus controles y notifica al operador qué cambios esperar. Esto reduce la ansiedad en equipos que nunca han usado Vallumix.
```
