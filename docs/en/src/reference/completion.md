# `completion` Command

The `completion` subcommand generates autocompletion scripts for the most commonly used shells. This allows the shell to suggest subcommands, flags, and possible values when you type `vallumix <TAB>`.

## Syntax

```bash
vallumix completion <shell>
```

## Supported Shells

| Shell | Generation Command | Installation Method |
|---|---|---|
| `bash` | `vallumix completion bash` | Redirect to file + `source` in `.bashrc` |
| `zsh` | `vallumix completion zsh` | Redirect to zsh completions directory |
| `fish` | `vallumix completion fish` | Redirect to fish completions directory |
| `nushell` | `vallumix completion nushell` | Manual load in `config.nu` |

## Bash

Generate and activate autocompletion:

```bash
vallumix completion bash > /tmp/vallumix.bash
sudo cp /tmp/vallumix.bash /etc/bash_completion.d/vallumix
```

For individual users without root privileges:

```bash
mkdir -p ~/.local/share/bash-completion/completions
vallumix completion bash > ~/.local/share/bash-completion/completions/vallumix
```

Reload the session or run `source ~/.bashrc` to activate.

## Zsh

With Oh-My-Zsh:

```bash
mkdir -p ~/.zsh/completions
vallumix completion zsh > ~/.zsh/completions/_vallumix
```

Make sure your `~/.zshrc` includes the completions directory in `fpath`:

```zsh
fpath+=(~/.zsh/completions)
autoload -U compinit && compinit
```

Reload the session or run `source ~/.zshrc`.

## Fish

```bash
mkdir -p ~/.config/fish/completions
vallumix completion fish > ~/.config/fish/completions/vallumix.fish
```

Fish reloads completions automatically; you do not need to restart the session.

## Nushell

```bash
vallumix completion nushell > ~/.config/nushell/vallumix-completions.nu
```

Add to the end of `~/.config/nushell/config.nu`:

```nu
source ~/.config/nushell/vallumix-completions.nu
```

## Verification

After installing autocompletion, test it:

```bash
vallumix <TAB><TAB>
# Should show: apply, audit, rollback, list, completion, help

vallumix apply --<TAB><TAB>
# Should show: --profile, --dry-run, --verbose, --threshold, etc.

vallumix apply --profile <TAB><TAB>
# Should show: web, database, bastion
```

```tip
Include completion generation in your automated installation script. An administrator who can autocomplete flags and profiles makes fewer typing errors and discovers functionalities that would otherwise go unnoticed.
```
