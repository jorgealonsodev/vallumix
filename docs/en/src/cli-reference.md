# CLI Reference

## Global Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--profile` | | Profile to use | `web` |
| `--dry-run` | | Preview changes without applying | `false` |
| `--verbose` | `-v` | Enable debug logging | `false` |
| `--quiet` | `-q` | Suppress non-error output | `false` |
| `--threshold` | | Compliance threshold (0-100) | `80` |
| `--no-color` | | Disable colored output | `false` |
| `--report` | | Report format(s): html, json, junit, text | |
| `--output` | | Output file path | |

## Subcommands

### `apply`

Apply hardening controls for the selected profile.

```bash
vallumix apply --profile web --dry-run
```

Requires root privileges unless `--dry-run` is specified.

### `audit`

Audit the system against the selected profile without making changes.

```bash
vallumix audit --profile web --report html --output report.html
```

### `rollback`

Revert changes from a previous session or control.

```bash
vallumix rollback --session <id>
vallumix rollback --control-id 5.2.4
```

### `list`

List all controls available for a profile.

```bash
vallumix list --profile database
```

### `completion`

Generate shell completions.

```bash
vallumix completion bash
vallumix completion zsh
vallumix completion fish
vallumix completion nushell
```

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success / compliance above threshold |
| `1` | Compliance below threshold |
| `2` | Runtime error |
| `3` | Privilege error (root required) |
