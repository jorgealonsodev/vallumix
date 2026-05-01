# `list` Command

The `list` subcommand shows the complete catalog of controls available in Vallumix, filtered by profile. It is useful for exploring which controls would be executed before launching an `apply` or `audit`.

## Syntax

```bash
vallumix list [FLAGS]
```

## list-Specific Flags

| Flag | Description |
|---|---|
| `--profile` | **Required.** Filters controls by profile (`web`, `database`, `bastion`). |
| `--no-color` | Disables colors in output. |

## Output Format

The default output is a terminal table with the following columns:

```text
ID      | SEVERITY | PROFILE    | DESCRIPTION
--------|----------|------------|------------------------------------------
1.1.1.1 | Low      | web,db,ba  | Disable cramfs support
1.1.1.2 | Low      | web,db,ba  | Disable freevxfs support
5.2.4   | Critical | web,db,ba  | Ensure SSH root login is disabled
3.4.1   | Medium   | web,db     | Ensure firewalld is installed
```

### Columns

- **ID:** CIS identifier of the control (for example, `5.2.4`).
- **SEVERITY:** criticality level (`Critical`, `High`, `Medium`, `Low`).
- **PROFILE:** profiles that include this control (`web`, `database`, `bastion`, abbreviated as `w`, `db`, `ba`).
- **DESCRIPTION:** human-readable description of the recommendation.

## Examples

### List Web Profile Controls

```bash
vallumix list --profile web
```

### List Bastion Profile Controls Without Colors

```bash
vallumix list --profile bastion --no-color
```

### Filter Critical Controls with grep

```bash
vallumix list --profile web --no-color | grep Critical
```

### Count Controls by Profile

```bash
vallumix list --profile web --no-color | wc -l
vallumix list --profile database --no-color | wc -l
vallumix list --profile bastion --no-color | wc -l
```

```tip
Use `vallumix list` as a discovery step in provisioning scripts. Before applying a profile, list its controls and notify the operator what changes to expect. This reduces anxiety in teams that have never used Vallumix.
```
