# execution-context Specification

## Purpose

Provide a `Context` struct that carries host information, resolved distribution, working paths, and configuration — initialized once at CLI startup and passed to all control methods.

## Requirements

### Requirement: Context Struct Fields

The `Context` struct MUST be defined in `vallumix-core::context` with fields: `hostname: String`, `distro: Distro`, `work_dir: PathBuf`, `backup_dir: PathBuf`, `profile_dir: PathBuf`, `dry_run: bool`. It MUST derive `Debug, Clone`.

#### Scenario: Context is constructed with all fields

- GIVEN `Distro::Debian12` was detected and hostname resolved to `"web01"`
- WHEN `Context::new(distro)` is called
- THEN `ctx.hostname` is `"web01"`, `ctx.distro` is `Debian12`, `ctx.dry_run` is `false`

#### Scenario: Context fields are accessible to controls

- GIVEN a `Context` instance with `backup_dir: PathBuf::from("/var/backups/vallumix")`
- WHEN a control accesses `ctx.backup_dir`
- THEN it returns the configured backup directory path

### Requirement: Default Directory Paths

When `Context::new(distro)` is called, it MUST set `work_dir` to `/var/lib/vallumix`, `backup_dir` to `/var/backups/vallumix`, and `profile_dir` to `/etc/vallumix/profiles`. These paths MUST be overridable via environment variables `VALLUMIX_WORK_DIR`, `VALLUMIX_BACKUP_DIR`, and `VALLUMIX_PROFILE_DIR` respectively.

#### Scenario: Default paths are set

- GIVEN no environment variables are set
- WHEN `Context::new(Distro::Rocky9)` is constructed
- THEN `work_dir` is `/var/lib/vallumix`, `backup_dir` is `/var/backups/vallumix`, `profile_dir` is `/etc/vallumix/profiles`

#### Scenario: Environment variables override default paths

- GIVEN `VALLUMIX_BACKUP_DIR=/tmp/test-backups` is set in the environment
- WHEN `Context::new(Distro::Rocky9)` is constructed
- THEN `ctx.backup_dir` is `/tmp/test-backups`

#### Scenario: Direct constructor accepts all paths

- GIVEN custom paths for work, backup, and profile directories
- WHEN `Context::with_paths(hostname, distro, work_dir, backup_dir, profile_dir, dry_run)` is called
- THEN all custom paths are set correctly

### Requirement: Hostname Resolution

`Context::new()` MUST resolve the system hostname using `nix::unistd::gethostname()`. If hostname resolution fails, it MUST fall back to `"localhost"` and log a warning via `tracing::warn!`.

#### Scenario: Hostname is resolved successfully

- GIVEN the system hostname is `"prod-web-01"`
- WHEN `Context::new(Distro::Ubuntu2404)` is called
- THEN `ctx.hostname` is `"prod-web-01"`

#### Scenario: Hostname resolution fails gracefully

- GIVEN hostname resolution returns an error
- WHEN `Context::new(Distro::Rocky9)` is called
- THEN `ctx.hostname` is `"localhost"` and a tracing warning is emitted

### Requirement: Dry-Run Flag Propagation

The `dry_run` field MUST default to `false` and MUST be settable via CLI `--dry-run` flag. Controls MUST read `ctx.dry_run` to skip file modifications while still performing checks.

#### Scenario: Dry-run mode is enabled

- GIVEN `--dry-run` flag is passed at CLI
- WHEN `Context` is constructed
- THEN `ctx.dry_run` is `true`

#### Scenario: Controls respect dry-run flag

- GIVEN `ctx.dry_run` is `true`
- WHEN a control's `apply()` method is invoked
- THEN the control MUST NOT modify any system files
