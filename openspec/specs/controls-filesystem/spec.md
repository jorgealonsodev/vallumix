# controls-filesystem Specification

## Purpose

Implement CIS 1.1.x filesystem controls that disable unused filesystems and enforce mount options, each implementing the `Control` trait with idempotent check/apply/rollback.

## Requirements

### Requirement: Filesystem Module Disable Controls

The system MUST provide controls that disable unnecessary filesystem kernel modules via `/etc/modprobe.d/` entries using the `install <module> /bin/true` pattern. Each control MUST implement `Control` with `category()` returning `Category::Filesystem`.

#### Scenario: disable_freevxfs check and apply

- GIVEN `freevxfs` module is not blacklisted
- WHEN `check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence that the module is loadable
- WHEN `apply(&ctx)` is called
- THEN `install freevxfs /bin/true` is written to `/etc/modprobe.d/` and backup is recorded

#### Scenario: disable_jffs2 — idempotent re-apply

- GIVEN `install jffs2 /bin/true` already exists in modprobe config
- WHEN `apply(&ctx)` is called
- THEN `AlreadyCompliant` is returned with no duplicate entries

### Requirement: Disable HFS Filesystems

The system MUST provide `disable_hfs` and `disable_hfsplus` controls (CIS 1.1.1.4, 1.1.1.5) that prevent loading `hfs` and `hfsplus` modules. `severity()` MUST return `Severity::Low`.

#### Scenario: disable_hfs check detects loaded module

- GIVEN `hfs` appears in `/proc/filesystems`
- WHEN `check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence containing the module name

### Requirement: Disable squashfs and udf

The system MUST provide `disable_squashfs` (CIS 1.1.1.6) and `disable_udf` (CIS 1.1.1.7) controls following the same modprobe pattern, with `severity()` returning `Severity::Low`.

#### Scenario: disable_udf rollback restores original config

- GIVEN `disable_udf.apply()` has been executed and created a backup
- WHEN `rollback(&ctx, &backup)` is called
- THEN the original modprobe config file is restored from backup

### Requirement: USB Storage Control

The system MUST provide `disable_usb_storage` (CIS 1.1.10) that blocks the `usb-storage` module. `severity()` MUST return `Severity::Medium`. `applicable_distros()` MUST return all supported distros.

#### Scenario: disable_usb_storage applies modprobe rule

- GIVEN `usb-storage` is not blocked
- WHEN `apply(&ctx)` is called
- THEN `install usb-storage /bin/true` is written and the running module is unloaded if present

### Requirement: Tmpfs Hardening Control

The system MUST provide `harden_tmpfs` (CIS 1.1.2.x) that enforces mount options `nodev`, `nosuid`, `noexec` on `/tmp` when mounted as tmpfs. `check()` MUST parse `/proc/mounts` or `findmnt` output.

#### Scenario: harden_tmpfs detects missing nodev option

- GIVEN `/tmp` is mounted as tmpfs without `nodev`
- WHEN `check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence listing the missing options

### Requirement: cramfs Already Implemented

The `disable_cramfs` control (CIS 1.1.1.1) already exists in the pilot. It MUST receive `category()` returning `Category::Filesystem`. No new implementation is needed.

#### Scenario: disable_cramfs returns Filesystem category

- GIVEN `DisableCramfs` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Filesystem`

### Requirement: Filesystem Controls with_paths Pattern

Every filesystem control MUST implement `new()` for production use and `with_paths(paths)` for testability with fixture paths, consistent with the established pilot pattern.

#### Scenario: with_paths allows test fixture override

- GIVEN `DisableFreevxfs::with_paths(MockPaths { modprobe_dir, proc_filesystems })`
- WHEN `check(&ctx)` is called with the mock context
- THEN it reads from the fixture paths instead of real `/etc/modprobe.d/` and `/proc/filesystems`

## Acceptance Criteria

- [ ] 8-10 filesystem controls implemented with CIS IDs
- [ ] All controls implement `Control` trait including `category()`
- [ ] Each control has unit tests using `with_paths` fixture pattern
- [ ] Idempotent apply — second call returns `AlreadyCompliant`
- [ ] Rollback restores original state from backup