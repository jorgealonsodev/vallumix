# Terminal Text Report

The text reporter produces a colored, terminal-friendly report designed for operators who want immediate feedback during an `audit` or `apply` run. It uses Unicode icons and ANSI color codes by default, with automatic fallback to plain text when output is piped or when `NO_COLOR` is set.

## When to Use Text

- **Operator triage** — quickly spot failed controls after running `apply`.
- **Terminal dashboards** — display compliance status on a console or wall monitor.
- **Log streaming** — stream output to a terminal multiplexer (tmux, screen) during automation.
- **Piped filtering** — pipe into `grep` or `less` to isolate specific controls.

## Generating a Text Report

```bash
vallumix audit --profile web --report text --output /var/reports/vallumix/audit
```

When `--report text` is used without `--output`, the report is printed to stdout.

## Output Format

```text
Vallumix Compliance Report — srv01 (rocky/9)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total:  45   Pass:  38   Fail:   5   Skip:   2
  Compliance Rate: 84.4%

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
[LOW]  ✓ Compliant 1.1.1.1 — Disable cramfs
      → not present
[HIGH] ✗ Non-Compliant 5.2.4 — Disable root login
      → PermitRootLogin yes
[MED]  ⚠ Skipped 3.1.1 — Disable IP forwarding
      → dry-run
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Visual Elements

| Element | Colored Mode | Plain Mode (`NO_COLOR`) |
|---------|--------------|--------------------------|
| Compliant | `✓ Compliant` (green) | `OK` |
| NonCompliant | `✗ Non-Compliant` (red) | `FAIL` |
| Skipped | `⚠ Skipped` (yellow) | `SKIP` |
| High severity | `[HIGH]` (red, bold) | `[HIGH]` |
| Medium severity | `[MED]` (yellow) | `[MED]` |
| Low severity | `[LOW]` (green) | `[LOW]` |

## NO_COLOR Support

Vallumix respects the [NO_COLOR](https://no-color.org/) standard. If the environment variable `NO_COLOR` is set to any non-empty value, the text reporter strips all ANSI escape sequences:

```bash
NO_COLOR=1 vallumix audit --profile web --report text
```

This is useful when:

- Redirecting output to a file that will be read by humans.
- Running inside CI systems that do not render ANSI codes.
- Piping into tools that choke on escape sequences.

## Piping and Filtering

Because the text format is line-oriented, it works well with standard Unix tools:

```bash
# Show only failed controls
vallumix audit --profile web --report text | grep 'FAIL'

# Count high-severity failures
vallumix audit --profile web --report text | grep -c '\[HIGH\].*FAIL'

# Stream to a log file with timestamps
vallumix audit --profile web --report text | ts '[%Y-%m-%d %H:%M:%S]' > /var/log/vallumix/audit.log
```

```tip
In a terminal with a dark background, the default color scheme (green pass, red fail, yellow skip) provides high contrast. If your terminal theme uses unusual colors, the plain-text mode (`NO_COLOR=1`) may be more readable.
```
