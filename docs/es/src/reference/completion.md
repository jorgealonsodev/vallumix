# Comando `completion`

El subcomando `completion` genera scripts de autocompletado para los shells más utilizados. Esto permite que al escribir `vallumix <TAB>` el shell sugiera subcomandos, flags y valores posibles.

## Sintaxis

```bash
vallumix completion <shell>
```

## Shells soportados

| Shell | Comando de generación | Método de instalación |
|---|---|---|
| `bash` | `vallumix completion bash` | Redirección a archivo + `source` en `.bashrc` |
| `zsh` | `vallumix completion zsh` | Redirección a directorio de completions de zsh |
| `fish` | `vallumix completion fish` | Redirección a directorio de completions de fish |
| `nushell` | `vallumix completion nushell` | Carga manual en `config.nu` |

## Bash

Genera y activa el autocompletado:

```bash
vallumix completion bash > /tmp/vallumix.bash
sudo cp /tmp/vallumix.bash /etc/bash_completion.d/vallumix
```

Para usuarios individuales sin privilegios de root:

```bash
mkdir -p ~/.local/share/bash-completion/completions
vallumix completion bash > ~/.local/share/bash-completion/completions/vallumix
```

Recarga la sesión o ejecuta `source ~/.bashrc` para activar.

## Zsh

Con Oh-My-Zsh:

```bash
mkdir -p ~/.zsh/completions
vallumix completion zsh > ~/.zsh/completions/_vallumix
```

Asegúrate de que tu `~/.zshrc` incluye el directorio de completions en `fpath`:

```zsh
fpath+=(~/.zsh/completions)
autoload -U compinit && compinit
```

Recarga la sesión o ejecuta `source ~/.zshrc`.

## Fish

```bash
mkdir -p ~/.config/fish/completions
vallumix completion fish > ~/.config/fish/completions/vallumix.fish
```

Fish recarga los completions automáticamente; no necesitas reiniciar la sesión.

## Nushell

```bash
vallumix completion nushell > ~/.config/nushell/vallumix-completions.nu
```

Añade al final de `~/.config/nushell/config.nu`:

```nu
source ~/.config/nushell/vallumix-completions.nu
```

## Verificación

Tras instalar el autocompletado, pruébalo:

```bash
vallumix <TAB><TAB>
# Debe mostrar: apply, audit, rollback, list, completion, help

vallumix apply --<TAB><TAB>
# Debe mostrar: --profile, --dry-run, --verbose, --threshold, etc.

vallumix apply --profile <TAB><TAB>
# Debe mostrar: web, database, bastion
```

```tip
Incluye la generación de completions en tu script de instalación automatizada. Un administrador que puede autocompletar flags y perfiles comete menos errores de tipeo y descubre funcionalidades que de otro modo pasarían desapercibidas.
```
