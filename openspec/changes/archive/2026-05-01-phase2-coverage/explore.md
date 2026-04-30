# Exploration: Phase 2 Coverage — 60+ CIS Controls, Reporters, Backup/Rollback

## Current State

### What Exists (Phase 0 + Phase 1, archived)

**2,273 total lines of Rust** across 5 crates, with **14 test modules** and **71 individual test functions**.

#### vallumix-core (6 files, ~565 lines)
- `control.rs` — `Control` trait with `check()`, `apply()`, `rollback()`, `clone_box()`. Types: `Severity`, `CheckStatus`, `ApplyStatus`, `CheckResult`, `ApplyResult`.
- `context.rs` — `Context` struct with hostname, distro, work_dir, backup_dir, profile_dir, dry_run. Supports env var overrides.
- `distro.rs` — `Distro` enum (Debian12, Ubuntu2204, Ubuntu2404, Rocky9). Parser for `/etc/os-release`.
- `error.rs` — `ControlError`, `VallumixError`, `ReportError` (all thiserror).
- `profile.rs` — `Profile` struct (name, description, controls Vec), `Reporter` trait, `Report`/`HostInfo`/`Summary`/`ControlReport` structs, `Backup` struct.
- `lib.rs` — Re-exports, `ControlRegistry` type alias, tests for profile parsing.

#### vallumix-controls (5 pilot controls, ~830 lines)
| CIS ID | Module | Category | Lines | Tests | Pattern |
|--------|--------|----------|-------|-------|---------|
| 1.1.1.1 | `filesystem/disable_cramfs.rs` | Filesystem | 209 | 6 | modprobe.d drop-in, `with_paths()` for testability |
| 2.2.3 | `services/disable_avahi.rs` | Services | 264 | 4 | systemctl commands, service_exists() check |
| 3.1.1 | `network/sysctl_ip_forwarding.rs` | Network | 166 | 5 | sysctl.d drop-in, /proc read |
| 5.2.4 | `ssh/disable_root_login.rs` | SSH | 225 | 8 | sshd_config parsing + rewriting |
| 6.1.1 | `maintenance/ensure_perms_passwd.rs` | Maintenance | 146 | 3 | file permission check + set |

**Established patterns:**
- Every control has `new()` (default paths) + `with_paths()` / `with_path()` for testability
- `check()` reads state, never modifies
- `apply()` respects `ctx.dry_run`, writes drop-in config files
- `rollback()` removes vallumix-created files or restores from backup
- Fixture-based tests in `tests/fixtures/`
- All controls applicable to all 4 distros

#### vallumix-reporters (1 file, ~154 lines)
- **ONLY `JsonReporter` implemented** — serializes `Report` via `serde_json`
- `build_report()` helper function
- HTML, JUnit XML, Text reporters declared in CLI `ReportFormat` enum but NOT implemented

#### vallumix-backup (1 file, ~71 lines)
- `BackupManager` with `create_backup()` and `restore()`
- Basic file copy with timestamped directories
- **Missing:** list backups, delete old backups, rollback by control-ID, rollback entire execution session, backup metadata/index

#### vallumix-cli (7 files, ~270 lines)
- `main.rs` — Full CLI with clap derive, all 5 subcommands defined
- `commands/apply.rs` — **STUB** (TODO: detect distro, TODO: implement)
- `commands/audit.rs` — **STUB** (TODO: detect distro, TODO: implement)
- `commands/rollback.rs` — **STUB** (only returns Ok(0))
- `commands/list.rs` — **WORKING** (iterates registry, prints controls)
- `commands/completion.rs` — **STUB** (only returns Ok(0))

#### Profiles
| Profile | Controls listed | Controls implemented |
|---------|----------------|---------------------|
| web.toml | 22 control IDs | 5 implemented (rest are placeholders) |
| database.toml | 0 (empty) | 0 |
| bastion.toml | 0 (empty) | 0 |

### What's Missing for Phase 2

#### 1. Controls: ~55 additional controls needed (target: 60+ total)

**CIS Categories currently covered:** filesystem (1), services (1), network (1), ssh (1), maintenance (1)

**CIS Categories NOT covered at all:**
- **Logging & Auditing (CIS 4.x)** — rsyslog, journald, auditd (0 controls)
- **Access/Auth/PAM (CIS 5.1, 5.3-5.5)** — PAM config, password policy, sudo, cron restrictions (0 controls)
- **Additional SSH (CIS 5.2)** — Only root login done; need 8+ more SSH controls
- **Additional filesystem (CIS 1.1)** — Only cramfs done; need 10+ more filesystem controls
- **Additional services (CIS 2.x)** — Only avahi done; need 8+ more service controls
- **Additional network (CIS 3.x)** — Only ip_forward done; need 8+ more sysctl controls
- **Additional maintenance (CIS 6.x)** — Only passwd perms done; need 10+ more file permission/integrity controls

#### 2. Reporters: 3 of 4 missing
- `html.rs` — needs `askama` dependency + templates
- `junit.rs` — needs `quick-xml` dependency
- `text.rs` — needs `owo-colors` for terminal output

#### 3. Backup/Rollback: needs significant expansion
- Session-level backup tracking (group by execution timestamp)
- Rollback by control-ID (`vallumix rollback --control-id 5.2.4`)
- Rollback entire session (`vallumix rollback`)
- Backup listing and cleanup
- Backup metadata index file

#### 4. CLI Commands: 4 of 5 are stubs
- `apply` — needs full orchestration: load profile → resolve controls → backup → check → apply → report
- `audit` — needs parallel execution with rayon
- `rollback` — needs integration with BackupManager
- `completion` — needs clap shell generation

#### 5. Missing dependencies (from PRD section 7.1)
| Dependency | Purpose | Needed for |
|-----------|---------|-----------|
| `askama` | HTML templating | HTML reporter |
| `quick-xml` | XML generation | JUnit reporter |
| `rayon` | Parallel iterators | audit mode parallelism |
| `indicatif` | Progress bar | CLI UX |
| `miette` | Rich error formatting | CLI user errors |
| `insta` | Snapshot testing | Phase 3 test coverage |
| `assert_cmd` | Integration testing | Phase 3 test coverage |
| `predicates` | Integration test assertions | Phase 3 test coverage |
| `serde_yaml` | YAML serialization | (PRD lists it, not yet used) |

## Affected Areas

- `crates/vallumix-controls/src/` — **PRIMARY**: 10+ new module directories, ~55 new control files
- `crates/vallumix-reporters/src/` — html.rs, junit.rs, text.rs (new files)
- `crates/vallumix-reporters/templates/` — askama HTML templates (new directory)
- `crates/vallumix-backup/src/lib.rs` — significant expansion needed
- `crates/vallumix-cli/src/commands/` — apply.rs, audit.rs, rollback.rs, completion.rs (implement stubs)
- `profiles/database.toml` — populate with ~30 control IDs
- `profiles/bastion.toml` — populate with ~35 control IDs
- `profiles/web.toml` — already populated, needs implementation of 17 placeholder controls
- `Cargo.toml` (workspace) — add rayon, askama, quick-xml, indicatif, miette
- `crates/vallumix-core/src/control.rs` — may need `nist_mappings()` or `category()` method on Control trait
- `crates/vallumix-core/src/profile.rs` — `is_applicable()` is a no-op, needs real logic

## Approaches

### Approach 1: Implement all controls first, then reporters, then CLI wiring
- **Pros:** Clear sequential phases, each builds on the previous
- **Cons:** Can't test reporters until controls exist, can't test CLI until everything works
- **Effort:** High (linear, no parallelism in development)

### Approach 2: Implement by CIS category, each category includes controls + profile updates
- **Pros:** Each category is self-contained and testable, profiles can be populated incrementally
- **Cons:** Reporters and CLI remain stubs until later
- **Effort:** Medium (good for parallel work across categories)

### Approach 3: Infrastructure first (reporters, CLI, backup), then controls in parallel batches
- **Pros:** Full pipeline works early, controls can be added incrementally, testing infrastructure ready
- **Cons:** Reporters need at least some controls to produce meaningful output
- **Effort:** Medium (best for iterative development)

### Approach 4: Hybrid — implement 2-3 controls per category in parallel batches, with reporters and CLI wiring in between
- **Pros:** Balances incremental delivery with working pipeline, each batch produces visible progress
- **Cons:** More context switching between categories
- **Effort:** Medium (recommended)

## Recommendation

**Approach 4 (Hybrid)** organized as sub-phases:

### Sub-phase 2A: Infrastructure (Week 1)
1. Add missing dependencies (rayon, askama, quick-xml, indicatif, miette)
2. Implement `HtmlReporter`, `JUnitReporter`, `TextReporter`
3. Expand `BackupManager` with session tracking, list, cleanup
4. Wire `apply` and `audit` commands (basic orchestration, no rayon yet)
5. Implement `completion` command

### Sub-phase 2B: Filesystem + Services (Week 1-2)
- 10 filesystem controls (CIS 1.1.x): cramfs (done), freevxfs, jffs2, hfs, hfsplus, squashfs, udf, fat, tmp_nodev, tmp_nosuid, tmp_noexec
- 8 service controls (CIS 2.x): avahi (done), cups, dhcp, slapd, nfs, rpcbind, snmp, telnet, rsh, rsync, talk, tftp

### Sub-phase 2C: Network + Logging (Week 2)
- 8 network controls (CIS 3.x): ip_forward (done), send_redirects, accept_redirects, accept_source_route, log_martians, bogus_error, tcp_syncookies, ignore_broadcasts, ipv6_disable
- 8 logging controls (CIS 4.x): rsyslog installed, rsyslog configured, journald configured, auditd installed, auditd configured, auditd rules (identity, login, sessions, time)

### Sub-phase 2D: SSH + Auth/PAM (Week 3)
- 9 SSH controls (CIS 5.2): root_login (done), protocol, max_auth_tries, ignore_rhosts, host_based_auth, x11_forwarding, max_sessions, ciphers, macs, kex_algorithms, banner, allow_users, allow_groups, login_grace_time, idle_timeout
- 8 auth/PAM controls (CIS 5.1, 5.3-5.5): pam_faillock, password_hash, password_complexity, password_history, password_age, sudo_restrictions, cron_restrictions, at_restrictions

### Sub-phase 2E: Maintenance + Profiles (Week 3-4)
- 12 maintenance controls (CIS 6.x): passwd_perms (done), shadow_perms, group_perms, gshadow_perms, passwd_integrity, shadow_integrity, group_integrity, umask, su_restrictions, world_writable_files, unowned_files, sticky_bit_tmp
- Populate database.toml and bastion.toml profiles
- Integrate rayon parallelism in audit mode

## Control Mapping (~60 controls total)

### CIS 1: Filesystem Configuration (12 controls)
| CIS ID | Description | Category |
|--------|-------------|----------|
| 1.1.1.1 | Ensure cramfs disabled | ✅ Done |
| 1.1.1.2 | Ensure freevxfs disabled | filesystem |
| 1.1.2.1 | Ensure /tmp separate partition | filesystem |
| 1.1.3.1 | Ensure /tmp nodev | filesystem |
| 1.1.3.2 | Ensure /tmp nosuid | filesystem |
| 1.1.3.3 | Ensure /tmp noexec | filesystem |
| 1.1.4.1 | Ensure /var separate partition | filesystem |
| 1.1.5.1 | Ensure /var/tmp nodev | filesystem |
| 1.1.5.2 | Ensure /var/tmp nosuid | filesystem |
| 1.1.5.3 | Ensure /var/tmp noexec | filesystem |
| 1.1.6.1 | Ensure /var/log separate partition | filesystem |
| 1.1.7.1 | Ensure /home separate partition | filesystem |

### CIS 2: Services (10 controls)
| CIS ID | Description | Category |
|--------|-------------|----------|
| 2.1.1 | Ensure time sync configured | services |
| 2.1.2 | Ensure X11 not enabled | services |
| 2.2.1 | Ensure Avahi not installed | services |
| 2.2.2 | Ensure CUPS not enabled | services |
| 2.2.3 | ✅ Ensure Avahi not enabled | ✅ Done |
| 2.2.4 | Ensure DHCP not enabled | services |
| 2.2.5 | Ensure LDAP not enabled | services |
| 2.2.6 | Ensure NFS not enabled | services |
| 2.2.7 | Ensure RPC not enabled | services |
| 2.2.8 | Ensure DNS not enabled | services |
| 2.2.9 | Ensure FTP not enabled | services |
| 2.2.10 | Ensure HTTP not enabled | services |
| 2.2.11 | Ensure IMAP/POP3 not enabled | services |
| 2.2.12 | Ensure Samba not enabled | services |
| 2.2.13 | Ensure SNMP not enabled | services |
| 2.2.14 | Ensure telnet not enabled | services |
| 2.2.15 | Ensure mail transfer not loopback | services |
| 2.2.16 | Ensure rsync not enabled | services |
| 2.2.17 | Ensure NIS not enabled | services |

### CIS 3: Network (10 controls)
| CIS ID | Description | Category |
|--------|-------------|----------|
| 3.1.1 | ✅ Ensure IP forwarding disabled | ✅ Done |
| 3.1.2 | Ensure packet redirect not sent | network |
| 3.2.1 | Ensure source routing ignored | network |
| 3.2.2 | Ensure ICMP redirects not accepted | network |
| 3.2.3 | Ensure secure ICMP redirects not accepted | network |
| 3.2.4 | Ensure suspicious packets logged | network |
| 3.2.5 | Ensure broadcast ICMP not accepted | network |
| 3.2.6 | Ensure bogus ICMP not accepted | network |
| 3.3.1 | Ensure TCP SYN cookies enabled | network |
| 3.3.2 | Ensure IPv6 disabled (if not used) | network |

### CIS 4: Logging & Auditing (10 controls)
| CIS ID | Description | Category |
|--------|-------------|----------|
| 4.1.1.1 | Ensure rsyslog installed | logging |
| 4.1.1.2 | Ensure rsyslog configured | logging |
| 4.1.1.3 | Ensure rsyslog default permissions | logging |
| 4.1.2.1 | Ensure journald configured | logging |
| 4.1.2.2 | Ensure journald not overridden | logging |
| 4.2.1.1 | Ensure auditd installed | logging |
| 4.2.1.2 | Ensure auditd configured | logging |
| 4.2.2.1 | Ensure audit for identity events | logging |
| 4.2.2.2 | Ensure audit for login/logout | logging |
| 4.2.2.3 | Ensure audit for session events | logging |

### CIS 5: Access, Auth & SSH (14 controls)
| CIS ID | Description | Category |
|--------|-------------|----------|
| 5.1.1 | Ensure cron daemon enabled | auth |
| 5.2.1 | Ensure SSH Protocol 2 | ssh |
| 5.2.2 | Ensure SSH LogLevel INFO | ssh |
| 5.2.3 | Ensure SSH MaxAuthTries ≤ 4 | ssh |
| 5.2.4 | ✅ Ensure SSH root login disabled | ✅ Done |
| 5.2.5 | Ensure SSH PermitEmptyPasswords disabled | ssh |
| 5.2.6 | Ensure SSH IdleTimeoutInterval configured | ssh |
| 5.2.7 | Ensure SSH LoginGraceTime configured | ssh |
| 5.2.8 | Ensure SSH access limited | ssh |
| 5.2.9 | Ensure SSH warning banner configured | ssh |
| 5.2.10 | Ensure SSH PAM enabled | ssh |
| 5.2.11 | Ensure SSH AllowUsers/AllowGroups set | ssh |
| 5.2.12 | Ensure SSH X11 forwarding disabled | ssh |
| 5.2.13 | Ensure SSH crypto policy configured | ssh |
| 5.3.1 | Ensure sudo custom log file | auth |
| 5.3.2 | Ensure sudo log commands | auth |
| 5.3.3 | Ensure sudo logfile set | auth |
| 5.4.1 | Ensure password hashing = SHA-512 | auth |
| 5.4.2 | Ensure lockout for failed passwords | auth |
| 5.4.3 | Ensure password reuse limited | auth |
| 5.4.4 | Ensure password complexity configured | auth |
| 5.4.5 | Ensure password age configured | auth |
| 5.5.1.1 | Ensure min days between password changes | auth |
| 5.5.1.2 | Ensure password expiration ≤ 365 days | auth |
| 5.5.1.3 | Ensure password warning ≥ 7 days | auth |
| 5.5.2 | Ensure system accounts non-login | auth |
| 5.5.3 | Ensure default group = GID 1000 | auth |
| 5.5.4 | Ensure default umask = 027 | auth |
| 5.5.5 | Ensure su restricted | auth |

### CIS 6: Maintenance (12 controls)
| CIS ID | Description | Category |
|--------|-------------|----------|
| 6.1.1 | ✅ Ensure /etc/passwd permissions | ✅ Done |
| 6.1.2 | Ensure /etc/passwd- permissions | maintenance |
| 6.1.3 | Ensure /etc/group permissions | maintenance |
| 6.1.4 | Ensure /etc/group- permissions | maintenance |
| 6.1.5 | Ensure /etc/shadow permissions | maintenance |
| 6.1.6 | Ensure /etc/shadow- permissions | maintenance |
| 6.1.7 | Ensure /etc/gshadow permissions | maintenance |
| 6.1.8 | Ensure /etc/gshadow- permissions | maintenance |
| 6.1.9 | Ensure /etc/shells permissions | maintenance |
| 6.1.10 | Ensure /etc/security/opasswd permissions | maintenance |
| 6.1.11 | Ensure world-writable files locked down | maintenance |
| 6.1.12 | Ensure no unowned files | maintenance |
| 6.1.13 | Ensure no ungrouped files | maintenance |
| 6.1.14 | Ensure SUID executables reviewed | maintenance |
| 6.1.15 | Ensure SGID executables reviewed | maintenance |
| 6.2.1 | Ensure accounts in /etc/passwd exist | maintenance |
| 6.2.2 | Ensure /etc/shadow groups match passwd | maintenance |
| 6.2.3 | Ensure no duplicate UIDs | maintenance |
| 6.2.4 | Ensure no duplicate GIDs | maintenance |
| 6.2.5 | Ensure root is UID 0 | maintenance |
| 6.2.6 | Ensure root PATH integrity | maintenance |
| 6.2.7 | Ensure home directories exist | maintenance |
| 6.2.8 | Ensure home directory permissions | maintenance |
| 6.2.9 | Ensure user dot files not group/world writable | maintenance |
| 6.2.10 | Ensure no .forward files | maintenance |
| 6.2.11 | Ensure no .netrc files | maintenance |
| 6.2.12 | Ensure group /etc/passwd owned by root | maintenance |

**Note:** The web.toml profile references 22 control IDs. The database and bastion profiles are empty. The final Phase 2 target is 60+ controls total across all profiles, with each profile selecting a subset appropriate to its role.

## Risks

1. **Scope creep:** CIS Benchmark has 200+ controls. Must be disciplined about selecting ~60 that provide maximum security value.
2. **Distro divergence:** RHEL-family uses `firewalld`/`yum`, Debian-family uses `ufw`/`apt`. Controls need distro-specific logic.
3. **Testing system-modifying controls:** Many controls require root, modify system files, or interact with systemd. Testability depends on the `with_paths()` pattern established in Phase 1.
4. **Backup complexity:** Rolling back sysctl, modprobe, and PAM changes is non-trivial. Some changes require service restarts.
5. **Parallel execution safety:** `rayon` parallelism is safe for audit (read-only) but NOT for apply (write operations must be sequential to avoid race conditions on shared files like sshd_config).
6. **HTML template compilation:** `askama` compiles templates at build time. Template errors are compile-time errors, which is good for safety but slows iteration.
7. **Profile population:** database.toml and bastion.toml need careful selection — applying web controls to a database server could break it.

## Ready for Proposal

**Yes.** The codebase has a clean, well-structured foundation with established patterns. The exploration reveals:

1. **Clear gap analysis:** 5 controls implemented, ~55 needed across 7 CIS categories
2. **Established patterns:** Every control follows the same structure, making new controls predictable
3. **Missing infrastructure:** 3 reporters, backup expansion, 4 CLI stubs, 5+ dependencies
4. **Realistic scope:** 4 weeks is achievable if organized into the sub-phases outlined above
5. **Key architectural decision needed:** Whether to add a `category()` method to the `Control` trait for filtering/grouping

The orchestrator should proceed to `sdd-propose` to create a formal change proposal with this exploration as context.
