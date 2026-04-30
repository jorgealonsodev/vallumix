# Proposal: Phase 2 Coverage — 60+ Controls, Reporters, Backup/Rollback

## Intent

Complete the v0.5 beta deliverable per PRD §8: 60+ CIS controls implemented, all three profiles (web/database/bastion) populated, four report formats (HTML/JSON/JUnit/text), and a full backup/rollback system. Currently 5 pilot controls exist across 5 CIS categories; ~55 more are needed across 7 categories, plus 3 missing reporters, expanded backup, and 4 CLI stubs wired up.

## Scope

### In Scope
- ~55 new CIS controls across 7 categories (filesystem, services, network, logging, auth/PAM, SSH, maintenance)
- 3 new reporters: HTML (askama), JUnit XML (quick-xml), Text (owo-colors)
- BackupManager expansion: session tracking, list, cleanup, rollback by control-ID
- CLI stubs wired: apply, audit (with rayon parallelism), rollback, completion
- Profiles database.toml and bastion.toml populated with control IDs
- New dependencies: rayon, askama, quick-xml, indicatif, miette
- Snapshot tests (insta) and integration tests (assert_cmd) for new code

### Out of Scope
- CIS Level 2 controls (deferred to post-v1.0)
- NIST/ISO/PCI-DSS cross-mapping on controls (PRD §5.5, deferred)
- Container/Kubernetes hardening (PRD §10)
- Remote orchestration or agent mode (PRD §10)
- TUI with ratatui (v2 roadmap)
- Packaging (.deb/.rpm) and release signing (Phase 5)

## Capabilities

### New Capabilities
- `filesystem-controls`: CIS 1.1.x — 11 additional filesystem module-disable and mount-option controls
- `services-controls`: CIS 2.x — 9+ service-disable controls (cups, dhcp, ldap, nfs, rpc, dns, ftp, http, snmp, telnet, rsync, nis)
- `network-controls`: CIS 3.x — 9 sysctl controls (send_redirects, accept_redirects, source_route, log_martians, bogus_error, tcp_syncookies, ignore_broadcasts, ipv6_disable)
- `logging-controls`: CIS 4.x — 10 controls (rsyslog installed/configured/permissions, journald configured/override, auditd installed/configured, audit rules for identity/login/session)
- `ssh-controls`: CIS 5.2.x — 12 additional SSH hardening controls (protocol, loglevel, max_auth_tries, empty_passwords, idle_timeout, login_grace_time, access_limit, banner, pam, allow_users, x11_forwarding, crypto_policy)
- `auth-controls`: CIS 5.1/5.3-5.5 — 14 auth/PAM controls (cron daemon, sudo config, password hashing/lockout/reuse/complexity/age, su_restrictions, umask)
- `maintenance-controls`: CIS 6.x — 15+ file-permission and user-integrity controls (shadow/group/gshadow perms, world-writable, unowned, SUID/SGID review, duplicate UIDs/GIDs, root PATH, home dirs, dot-files, .forward/.netrc)
- `html-reporter`: askama-based self-contained HTML report with embedded CSS
- `junit-reporter`: quick-xml JUnit XML for CI integration (Jenkins, GitLab, GitHub Actions)
- `text-reporter`: terminal-friendly output with owo-colors and icon prefixes
- `backup-manager`: session-level tracking, list, cleanup/prune, rollback by control-ID and by session
- `cli-orchestration`: apply/audit/rollback/completion commands fully wired with profile loading, control resolution, progress bars, and exit codes

### Modified Capabilities
- `core-traits`: Control trait may gain `category()` method for filtering/grouping by CIS domain
- `profile-model`: `is_applicable()` needs real distro-filtering logic (currently no-op)
- `cli-structure`: wiring of 4 stub subcommands to real orchestration logic
- `pilot-controls`: existing 5 controls may gain `category()` implementation if trait changes

## Approach

**Hybrid incremental** (Approach 4 from explore): deliver in 5 sub-phases organized by CIS category, interleaving infrastructure work.

| Sub-phase | Focus | Est. Tasks |
|-----------|-------|-----------|
| 2A: Infrastructure | Dependencies, reporters (HTML/JUnit/text), BackupManager expansion, apply/audit CLI wiring, completion | ~30 |
| 2B: Filesystem + Services | 11 filesystem + 9 service controls, web.toml updates | ~25 |
| 2C: Network + Logging | 9 network + 10 logging controls | ~25 |
| 2D: SSH + Auth/PAM | 12 SSH + 14 auth controls | ~30 |
| 2E: Maintenance + Profiles | 15+ maintenance controls, database.toml, bastion.toml, rayon integration | ~30 |

Each sub-phase produces a compilable, testable increment. Controls follow the established pattern: `new()` + `with_paths()` for testability, `check()` reads-only, `apply()` respects dry_run, `rollback()` restores from backup.

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `crates/vallumix-controls/src/` | New | ~55 new control files in 7 module directories |
| `crates/vallumix-reporters/src/html.rs` | New | HTML reporter with askama templates |
| `crates/vallumix-reporters/src/junit.rs` | New | JUnit XML reporter with quick-xml |
| `crates/vallumix-reporters/src/text.rs` | New | Text reporter with owo-colors |
| `crates/vallumix-reporters/templates/` | New | askama HTML template directory |
| `crates/vallumix-backup/src/lib.rs` | Modified | Session tracking, list, cleanup, rollback by ID |
| `crates/vallumix-cli/src/commands/apply.rs` | Modified | Full orchestration implementation |
| `crates/vallumix-cli/src/commands/audit.rs` | Modified | Parallel audit with rayon |
| `crates/vallumix-cli/src/commands/rollback.rs` | Modified | BackupManager integration |
| `crates/vallumix-cli/src/commands/completion.rs` | Modified | Clap shell generation |
| `crates/vallumix-core/src/control.rs` | Modified | Possible `category()` method addition |
| `crates/vallumix-core/src/profile.rs` | Modified | Real `is_applicable()` logic |
| `profiles/database.toml` | Modified | Populate with ~30 control IDs |
| `profiles/bastion.toml` | Modified | Populate with ~35 control IDs |
| `profiles/web.toml` | Modified | Update with newly implemented control IDs |
| `Cargo.toml` (workspace) | Modified | Add rayon, askama, quick-xml, indicatif, miette |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Scope creep beyond 60 controls | High | Hard cap at 65 controls; defer extras to Phase 3 |
| Distro divergence (RHEL vs Debian paths) | High | Each control uses `with_paths()` pattern; test fixtures per distro family |
| Parallel safety in apply mode | Medium | rayon only for audit (read-only); apply stays sequential |
| Backup complexity for PAM/sysctl changes | Medium | BackupManager stores originals before any modification; rollback restores exact files |
| askama compile-time template errors slow iteration | Low | Templates are simple; CI validates on every push |
| Profile mis-selection breaks services | Medium | database/bastion profiles reviewed for safe defaults; dry-run recommended in docs |

## Rollback Plan

1. All changes are additive (new files in controls/, reporters/, templates/) — revertable via `git revert`
2. Modified files (backup, CLI stubs, core traits) are backwards-compatible expansions
3. No existing tests are broken — new code adds test modules
4. If a sub-phase introduces regressions, revert that sub-phase's commits independently

## Dependencies

- `rayon` — parallel audit execution (PRD §7.1)
- `askama` — HTML templating compiled at build time (PRD §7.1)
- `quick-xml` — JUnit XML generation (PRD §7.1)
- `indicatif` — progress bars for CLI UX (PRD §7.1)
- `miette` — rich error formatting for user-facing errors (PRD §7.1)
- `owo-colors` — already declared, needs actual usage in text reporter
- `insta` — snapshot testing (Phase 3 prep)
- `assert_cmd` + `predicates` — CLI integration testing (Phase 3 prep)

## Success Criteria

- [ ] 60+ controls implemented and passing unit tests
- [ ] All 4 report formats produce valid output (HTML, JSON, JUnit XML, text)
- [ ] `vallumix apply --profile web` runs end-to-end with backup + report
- [ ] `vallumix audit --profile database` runs in parallel with rayon
- [ ] `vallumix rollback --control-id 5.2.4` restores from backup
- [ ] database.toml and bastion.toml populated and parseable
- [ ] All existing tests pass (71 test functions)
- [ ] `cargo clippy -- -D warnings` clean
- [ ] Estimated ~150 tasks across 5 sub-phases
