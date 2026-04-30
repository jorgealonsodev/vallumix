# Tasks: phase2-coverage — 60+ Controls, 3 Reporters, Backup/CLI Orchestration

**Change**: phase2-coverage
**Total Tasks**: 67
**Completed**: 58 (T-001 through T-058)
**Remaining**: 9 (T-059 through T-067)
**Implementation Order**: Core infra → Reporters → Controls by CIS category → Profiles → CLI orchestration → Integration tests

## Completion Status

- [x] Phase 1: T-001–T-006 (Core traits, profile, CLI)
- [x] Phase 2: T-007–T-011 (BackupManager expansion)
- [x] Phase 3: T-012–T-015 (Reporters: HTML, JUnit, Text, JSON refactor)
- [x] Phase 4: T-016–T-022 (Filesystem controls)
- [x] Phase 5: T-023–T-028 (Services controls)
- [x] Phase 6: T-029–T-034 (Network controls)
- [x] Phase 7: T-035–T-040 (Logging controls)
- [x] Phase 8: T-041–T-045 (SSH controls)
- [x] Phase 9: T-046–T-050 (Auth controls)
- [x] Phase 10: T-051–T-055 (Maintenance controls)
- [x] Phase 11: T-056–T-058 (Profile population)
- [x] Phase 12: T-059–T-063 (CLI orchestration)
- [x] Phase 13: T-064–T-066 (Integration tests)
- [ ] Phase 13: T-067 (CI verification — deferred)

---

## Phase 1: Infrastructure & Core Traits

### T-001 — core-traits
**Capability**: Add `Category` enum and `Control::category()` to core
**Description**: Add `Category` enum (Filesystem, Services, Network, Logging, Ssh, Auth, Maintenance) to `control.rs`, add `fn category(&self) -> Category` with default impl, add `CheckStatus::Warning(Option<String>)` and `ApplyStatus::PartialApply(Option<String>)`, add `Serialize/Deserialize` derives to `CheckStatus`, `ApplyStatus`.
**Test RED→GREEN→REFACTOR**:
- RED: Write test constructing `Category::Filesystem`, calling `category()` on pilot control, asserting `CheckStatus::Warning` serializes
- GREEN: Implement enum, trait method, variants, derives
- REFACTOR: Ensure no dead code, derives consistent across enums
**Dependencies**: None
**Files affected**: `crates/vallumix-core/src/control.rs`

---

### T-002 — core-traits
**Capability**: Pilot controls gain `category()` implementation
**Description**: Modify 5 pilot controls (disable_cramfs, disable_avahi, sysctl_ip_forwarding, ssh_disable_root_login, ensure_perms_passwd) to implement `category()` returning the correct `Category` variant per their CIS domain.
**Test RED→GREEN→REFACTOR**:
- RED: Write test `pilot_controls_return_category()` — call `category()` on each pilot and assert expected `Category` variant
- GREEN: Add `category()` impl to each pilot control
- REFACTOR: Move category to trait default? No — explicit per spec
**Dependencies**: T-001 (Category must exist first)
**Files affected**: `crates/vallumix-controls/src/filesystem/disable_cramfs.rs`, `crates/vallumix-controls/src/services/disable_avahi.rs`, `crates/vallumix-controls/src/network/sysctl_ip_forwarding.rs`, `crates/vallumix-controls/src/ssh/disable_root_login.rs`, `crates/vallumix-controls/src/maintenance/ensure_perms_passwd.rs`

---

### T-003 — profile-model
**Capability**: Add `Profile::controls_by_category()` method
**Description**: Add `controls_by_category(&self, registry: &ControlRegistry) -> HashMap<Category, Vec<String>>` to `Profile` struct that groups control IDs by their `Category`.
**Test RED→GREEN→REFACTOR**:
- RED: Write test with profile having ["1.1.1.1", "5.2.4", "3.1.1"] — assert HashMap has 3 keys with correct grouping
- GREEN: Implement method using `registry.get(id).category()`
- REFACTOR: Cache result? Not needed for v0.5
**Dependencies**: T-001 (Category), T-002 (pilot category impls)
**Files affected**: `crates/vallumix-core/src/profile.rs`

---

### T-004 — profile-model
**Capability**: Real `Profile::is_applicable()` distro filtering
**Description**: Implement real `is_applicable(&self, distro: &Distro) -> bool` that resolves controls against registry and checks each control's `applicable_distros()`. Empty control list returns `true`.
**Test RED→GREEN→REFACTOR**:
- RED: Test profile with control applicable only to Rocky9 against Debian12 → returns false; same profile against Rocky9 → true; empty profile → true
- GREEN: Implement using `profile.resolve_controls()` then iterates checking `applicable_distros().contains(distro)`
- REFACTOR: Short-circuit on first non-match
**Dependencies**: T-001, T-002, controls registry populated
**Files affected**: `crates/vallumix-core/src/profile.rs`

---

### T-005 — cli-structure
**Capability**: Expand `--report` to html/junit/text, add `--output`
**Description**: Modify clap derive on `apply` and `audit` subcommands to accept comma-separated report formats (json, html, junit, text), add `--output <PATH>` flag. Parse into `Vec<String>` report_formats.
**Test RED→GREEN→REFACTOR**:
- RED: Write test parsing `vallumix audit --profile web --report html,json --output /tmp/out`
- GREEN: Add value_parser for comma-split, add output Arg
- REFACTOR: Extract format parsing to helper fn
**Dependencies**: None
**Files affected**: `crates/vallumix-cli/src/main.rs` (or cli structure file)

---

### T-006 — cli-structure
**Capability**: Make `--control-id` optional on rollback subcommand
**Description**: Change `rollback` subcommand so `--control-id` is `Option<String>`. When `None`, rollback entire most-recent session.
**Test RED→GREEN→REFACTOR**:
- RED: Test parse `rollback --control-id 5.2.4` → Some("5.2.4"); parse `rollback` → None
- GREEN: Add arg with `required(false)` and `value_name("ID")`
- REFACTOR: None needed
**Dependencies**: None
**Files affected**: `crates/vallumix-cli/src/main.rs`

---

## Phase 2: BackupManager Expansion

### T-007 — backup-manager
**Capability**: Add `BackupSession` struct and session-based directory layout
**Description**: Add `BackupSession { id: String, backup_dir: PathBuf, timestamp: DateTime<Utc>, profile: String, control_ids: Vec<String> }`. Modify `create_backup` to use `/var/backups/vallumix/<session_id>/<control_id>/v<N>/` path. Write `session.json` metadata file per session.
**Test RED→GREEN→REFACTOR**:
- RED: Write test creating 2 backups in same session, verify session.json exists with both control IDs
- GREEN: Add struct, update create_backup path logic, write session.json
- REFACTOR: Extract session ID generation to helper
**Dependencies**: None
**Files affected**: `crates/vallumix-backup/src/lib.rs`

---

### T-008 — backup-manager
**Capability**: Add `BackupManager::list()` method
**Description**: Add `list(&self, session_id: &str) -> Result<Vec<BackupMeta>>` returning all backups for a session ordered by control_id. Add `list_all_sessions() -> Result<Vec<SessionInfo>>` returning all sessions ordered newest-first.
**Test RED→GREEN→REFACTOR**:
- RED: Write test with 2 sessions (8 backups total), call list_all_sessions() → assert 2 sessions, newest first; call list("session1") → assert correct backups
- GREEN: Implement by scanning backup_dir subdirs, parsing session.json
- REFACTOR: Use `read_dir` sorted by name descending for sessions
**Dependencies**: T-007
**Files affected**: `crates/vallumix-backup/src/lib.rs`

---

### T-009 — backup-manager
**Capability**: Add `rollback_by_control(control_id)` and `rollback_session(session_id)`
**Description**: Add `rollback_by_control(&self, control_id: &str) -> Result<()>` that finds most recent backup for control and restores it. Add `rollback_session(&self, session_id: &str) -> Result<usize>` that restores all controls in session, returns count.
**Test RED→GREEN→REFACTOR**:
- RED: Write test: apply 2 controls, call rollback_by_control("5.2.4"), verify only that control restored; call rollback_session, verify both restored
- GREEN: Implement search by scanning sessions newest-first for control_id match, then restore
- REFACTOR: Reuse restore logic between both methods
**Dependencies**: T-007, T-008
**Files affected**: `crates/vallumix-backup/src/lib.rs`

---

### T-010 — backup-manager
**Capability**: Add `prune(keep_count)` and `verify()` methods
**Description**: Add `prune(&self, session_id: &str, keep: usize) -> Result<usize>` removing oldest versions beyond `keep`. Add `verify(&self, session_id: &str) -> Result<Vec<IntegrityFailure>>` checking backup files exist and have non-zero size.
**Test RED→GREEN→REFACTOR**:
- RED: Write test: create 8 backups, prune(5) → 3 deleted, 5 remain; verify with deleted file → reports failure
- GREEN: Implement prune by sorting backup versions, delete oldest; verify by checking file existence and size
- REFACTOR: Extract version sorting to helper
**Dependencies**: T-007, T-008
**Files affected**: `crates/vallumix-backup/src/lib.rs`

---

### T-011 — backup-manager
**Capability**: Add SHA-256 checksum to backup integrity
**Description**: Add `checksum(&self, path: &Path) -> Result<String>` using `sha2` crate. Store checksum as sidecar `<file>.sha256`. Verify on restore.
**Test RED→GREEN→REFACTOR**:
- RED: Write test: create backup, read sha256 sidecar, modify original file, call verify() → reports integrity failure
- GREEN: Add sha2 dependency, implement checksum and verify
- REFACTOR: Use `digest::Digest` trait for abstraction
**Dependencies**: T-007
**Files affected**: `crates/vallumix-backup/src/lib.rs`, `Cargo.toml` (workspace)

---

## Phase 3: Reporters (can parallelize T-012, T-013, T-014)

### T-012 — reporter-html
**Capability**: Implement HtmlReporter with askama template
**Description**: Create `crates/vallumix-reporters/src/html.rs` with `HtmlReporter` struct implementing `Reporter`. Create `crates/vallumix-reporters/templates/report.html` with embedded CSS. Generate compliance summary, per-control detail grouped by severity.
**Test RED→GREEN→REFACTOR**:
- RED: Write insta snapshot test generating HTML from 2-control report → assert snapshot matches; test compliance rate shows percentage
- GREEN: Implement HtmlReporter with askama template, embedded CSS, summary section, per-control table
- REFACTOR: Extract template data mapping to `ReportTemplate` struct
**Dependencies**: T-005 (report format parsing)
**Files affected**: `crates/vallumix-reporters/src/html.rs`, `crates/vallumix-reporters/templates/report.html`, `crates/vallumix-reporters/src/lib.rs` (add mod html)

---

### T-013 — reporter-junit
**Capability**: Implement JunitReporter with quick-xml
**Description**: Create `crates/vallumix-reporters/src/junit.rs` with `JunitReporter` struct implementing `Reporter`. Map control status: Compliant→passing testcase, NonCompliant→failure, Skipped→skipped, Error→error. Escape special XML chars.
**Test RED→GREEN→REFACTOR**:
- RED: Write test with mixed status controls → assert XML well-formed, testsuite attrs match summary counts, special chars (`&`, `<`) escaped
- GREEN: Implement with quick-xml manual element construction
- REFACTOR: Extract XML escaping to utility fn
**Dependencies**: T-005
**Files affected**: `crates/vallumix-reporters/src/junit.rs`, `crates/vallumix-reporters/src/lib.rs` (add mod junit)

---

### T-014 — reporter-text
**Capability**: Implement TextReporter with owo-colors
**Description**: Create `crates/vallumix-reporters/src/text.rs` with `TextReporter` struct implementing `Reporter`. Use ✓/✗/⚠ icons (green/red/yellow). Support `no_color: bool` constructor, respect NO_COLOR env var. Display severity as [HIGH]/[MED]/[LOW].
**Test RED→GREEN→REFACTOR**:
- RED: Write test: generate text report, assert ✓/✗/⚠ icons present, plain mode (NO_COLOR=1) uses OK/FAIL/SKIP; test host info header present
- GREEN: Implement with owo-colors `OwoColorize` trait, conditional color via `set_override`
- REFACTOR: Extract icon mapping to helper
**Dependencies**: T-005
**Files affected**: `crates/vallumix-reporters/src/text.rs`, `crates/vallumix-reporters/src/lib.rs` (add mod text)

---

### T-015 — reporter-json
**Capability**: Refactor reporters lib as module with json sub-module
**Description**: Refactor `crates/vallumix-reporters/src/lib.rs` to be a module root — move `JsonReporter` to `json.rs`, export all reporters. Update `build_report` to accept `&[Distro]` for host info.
**Test RED→GREEN→REFACTOR**:
- RED: Write test importing `JsonReporter` from `json` module, assert `Box<dyn Reporter>` works
- GREEN: Create `json.rs`, update lib.rs exports, update build_report signature
- REFACTOR: Ensure no duplicate code between reporters
**Dependencies**: T-012, T-013, T-014 (all reporters exist)
**Files affected**: `crates/vallumix-reporters/src/lib.rs`, `crates/vallumix-reporters/src/json.rs`

---

## Phase 4: Controls — Filesystem (CIS 1.1.x)

### T-016 — controls-filesystem
**Capability**: Implement `disable_freevxfs` (CIS 1.1.1.2)
**Description**: Create `disable_freevxfs.rs` — check `/proc/filesystems` for freevxfs, apply `install freevxfs /bin/true` to modprobe.d, `category()` returns `Category::Filesystem`.
**Test RED→GREEN→REFACTOR**:
- RED: Test check on fixture without freevxfs → Compliant; with freevxfs → NonCompliant; test idempotent apply
- GREEN: Implement control following disable_cramfs pattern
- REFACTOR: Extract modprobe write logic to shared helper
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/filesystem/disable_freevxfs.rs`, `crates/vallumix-controls/src/filesystem/mod.rs`

---

### T-017 — controls-filesystem
**Capability**: Implement `disable_hfs` (CIS 1.1.1.4) and `disable_hfsplus` (CIS 1.1.1.5)
**Description**: Create `disable_hfs.rs` and `disable_hfsplus.rs` — same modprobe pattern, severity Low, category Filesystem.
**Test RED→GREEN→REFACTOR**:
- RED: Test both controls on fixture with/without hfs/hfsplus modules
- GREEN: Implement both controls
- REFACTOR: Consider shared HFS-disables helper if patterns diverge
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/filesystem/disable_hfs.rs`, `crates/vallumix-controls/src/filesystem/disable_hfsplus.rs`, `crates/vallumix-controls/src/filesystem/mod.rs`

---

### T-018 — controls-filesystem
**Capability**: Implement `disable_squashfs` (CIS 1.1.1.6) and `disable_udf` (CIS 1.1.1.7)
**Description**: Create `disable_squashfs.rs` and `disable_udf.rs` — modprobe pattern, severity Low, category Filesystem.
**Test RED→GREEN→REFACTOR**:
- RED: Test both on fixture with/without respective modules
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/filesystem/disable_squashfs.rs`, `crates/vallumix-controls/src/filesystem/disable_udf.rs`, `crates/vallumix-controls/src/filesystem/mod.rs`

---

### T-019 — controls-filesystem
**Capability**: Implement `disable_jffs2` (CIS 1.1.1.3)
**Description**: Create `disable_jffs2.rs` — modprobe pattern, check for jffs2 in /proc/filesystems.
**Test RED→GREEN→REFACTOR**:
- RED: Test check on fixture with jffs2 → NonCompliant; without → Compliant; test idempotent apply
- GREEN: Implement control
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/filesystem/disable_jffs2.rs`, `crates/vallumix-controls/src/filesystem/mod.rs`

---

### T-020 — controls-filesystem
**Capability**: Implement `disable_usb_storage` (CIS 1.1.10)
**Description**: Create `disable_usb_storage.rs` — check `/proc/filesystems` for usb-storage, apply `install usb-storage /bin/true`, severity Medium.
**Test RED→GREEN→REFACTOR**:
- RED: Test check on fixture with/without usb-storage; test apply writes correct content
- GREEN: Implement control
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/filesystem/disable_usb_storage.rs`, `crates/vallumix-controls/src/filesystem/mod.rs`

---

### T-021 — controls-filesystem
**Capability**: Implement `harden_tmpfs` (CIS 1.1.2.x)
**Description**: Create `harden_tmpfs.rs` — check `/proc/mounts` or `findmnt` for /tmp tmpfs mount options. Require nodev, nosuid, noexec. `apply()` creates drop-in under `/etc/systemd/system/tmp.mount.d/`.
**Test RED→GREEN→REFACTOR**:
- RED: Test check on tmpfs mount with nodev but missing nosuid → NonCompliant; with all 3 → Compliant; test apply creates drop-in
- GREEN: Implement control with tmpfs mount option parsing
- REFACTOR: Handle case where /tmp is not tmpfs (skip)
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/filesystem/harden_tmpfs.rs`, `crates/vallumix-controls/src/filesystem/mod.rs`

---

### T-022 — controls-filesystem
**Capability**: Register all filesystem controls in registry
**Description**: Update `crates/vallumix-controls/src/lib.rs` registry() to include all 8-10 filesystem controls with correct CIS IDs. Remove pilot-only entry for 1.1.1.1 (already there), add new entries.
**Test RED→GREEN→REFACTOR**:
- RED: Write test registry_has_all_filesystem_controls() — assert len >= 8 and contains specific IDs
- GREEN: Add all filesystem control entries to registry()
- REFACTOR: Group by category in registry comments
**Dependencies**: T-016 through T-021
**Files affected**: `crates/vallumix-controls/src/lib.rs`

---

## Phase 5: Controls — Services (CIS 2.x)

### T-023 — controls-services
**Capability**: Implement `disable_cups` (CIS 2.2.2)
**Description**: Create `disable_cups.rs` — check `systemctl is-active cups` and `systemctl is-enabled cups`, apply `systemctl disable --now cups`, category Services.
**Test RED→GREEN→REFACTOR**:
- RED: Test check on running/enabled cups → NonCompliant; stopped/disabled → Compliant; test idempotent apply
- GREEN: Implement using with_paths() for systemctl mock
- REFACTOR: Extract systemctl interaction to helper
**Dependencies**: T-001, T-002 (avahi already has category)
**Files affected**: `crates/vallumix-controls/src/services/disable_cups.rs`, `crates/vallumix-controls/src/services/mod.rs`

---

### T-024 — controls-services
**Capability**: Implement `disable_dhcp` (CIS 2.2.4), `disable_ldap` (CIS 2.2.5), `disable_nfs` (CIS 2.2.6)
**Description**: Create individual control files for each — dhcp (isc-dhcp-server), ldap (slapd), nfs (nfs-kernel-server). Check service status via systemctl, apply disables.
**Test RED→GREEN→REFACTOR**:
- RED: Test each on system with/without service — absent package = Compliant
- GREEN: Implement all 3 controls
- REFACTOR: Common service-disable pattern in shared module
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/services/disable_dhcp.rs`, `crates/vallumix-controls/src/services/disable_ldap.rs`, `crates/vallumix-controls/src/services/disable_nfs.rs`, `crates/vallumix-controls/src/services/mod.rs`

---

### T-025 — controls-services
**Capability**: Implement `disable_rpcbind` (CIS 2.2.7), `disable_bind` (CIS 2.2.8), `disable_vsftpd` (CIS 2.2.9)
**Description**: Create 3 control files following same service-disable pattern.
**Test RED→GREEN→REFACTOR**:
- RED: Test each with with_paths() mock systemctl; absent service → Compliant
- GREEN: Implement all 3
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/services/disable_rpcbind.rs`, `crates/vallumix-controls/src/services/disable_bind.rs`, `crates/vallumix-controls/src/services/disable_vsftpd.rs`, `crates/vallumix-controls/src/services/mod.rs`

---

### T-026 — controls-services
**Capability**: Implement `disable_httpd` (CIS 2.2.10), `disable_dovecot` (CIS 2.2.11), `disable_snmpd` (CIS 2.2.14), `disable_rsync` (CIS 2.2.15)
**Description**: Create 4 control files following same pattern.
**Test RED→GREEN→REFACTOR**:
- RED: Test each with with_paths(); absent package → Compliant
- GREEN: Implement all 4
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/services/disable_httpd.rs`, `crates/vallumix-controls/src/services/disable_dovecot.rs`, `crates/vallumix-controls/src/services/disable_snmpd.rs`, `crates/vallumix-controls/src/services/disable_rsync.rs`, `crates/vallumix-controls/src/services/mod.rs`

---

### T-027 — controls-services
**Capability**: Implement `disable_xinetd` (CIS 2.2.12)
**Description**: Create `disable_xinetd.rs` — check xinetd package not installed → Compliant; if installed, check disabled.
**Test RED→GREEN→REFACTOR**:
- RED: Test absent xinetd → Compliant; present and enabled → NonCompliant
- GREEN: Implement control
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/services/disable_xinetd.rs`, `crates/vallumix-controls/src/services/mod.rs`

---

### T-028 — controls-services
**Capability**: Register all service controls in registry
**Description**: Update registry() to include all 9+ service controls. Add disable_avahi (already pilot).
**Test RED→GREEN→REFACTOR**:
- RED: Test registry_has_all_service_controls() — assert len >= 9
- GREEN: Add all service control entries
- REFACTOR: None
**Dependencies**: T-023 through T-027
**Files affected**: `crates/vallumix-controls/src/lib.rs`

---

## Phase 6: Controls — Network (CIS 3.x)

### T-029 — controls-network
**Capability**: Implement `sysctl_disable_send_redirects` (CIS 3.1.2)
**Description**: Create `sysctl_disable_send_redirects.rs` — check `net.ipv4.conf.all.send_redirects` and `net.ipv4.conf.default.send_redirects` both == 0. Apply writes drop-in `/etc/sysctl.d/99-vallumix.conf`.
**Test RED→GREEN→REFACTOR**:
- RED: Test check on fixture with value 1 → NonCompliant; 0 → Compliant; test apply creates sysctl drop-in
- GREEN: Implement sysctl control with with_paths() overriding /proc/sys and /etc/sysctl.d
- REFACTOR: Extract sysctl read/write to shared module
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/network/sysctl_disable_send_redirects.rs`, `crates/vallumix-controls/src/network/mod.rs`

---

### T-030 — controls-network
**Capability**: Implement `sysctl_disable_source_route` (CIS 3.2.1) and `sysctl_disable_accept_redirects` (CIS 3.2.2)
**Description**: Create 2 control files — source_route checks `accept_source_route` for all+default; accept_redirects checks `accept_redirects` for all+default.
**Test RED→GREEN→REFACTOR**:
- RED: Test each control with fixture values; all must be 0 for compliant
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/network/sysctl_disable_source_route.rs`, `crates/vallumix-controls/src/network/sysctl_disable_accept_redirects.rs`, `crates/vallumix-controls/src/network/mod.rs`

---

### T-031 — controls-network
**Capability**: Implement `sysctl_enable_rp_filter` (CIS 3.2.6)
**Description**: Create `sysctl_enable_rp_filter.rs` — check `rp_filter` all+default == 1. Apply sets via sysctl drop-in.
**Test RED→GREEN→REFACTOR**:
- RED: Test check with value 0 → NonCompliant; 1 → Compliant; test apply writes drop-in
- GREEN: Implement control
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/network/sysctl_enable_rp_filter.rs`, `crates/vallumix-controls/src/network/mod.rs`

---

### T-032 — controls-network
**Capability**: Implement `sysctl_enable_syncookies` (CIS 3.2.7) and `sysctl_disable_icmp_redirects` (CIS 3.2.3)
**Description**: Create 2 control files — syncookies checks tcp_syncookies == 1; icmp_redirects checks icmp_redirects all+default == 0.
**Test RED→GREEN→REFACTOR**:
- RED: Test each with fixture values
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/network/sysctl_enable_syncookies.rs`, `crates/vallumix-controls/src/network/sysctl_disable_icmp_redirects.rs`, `crates/vallumix-controls/src/network/mod.rs`

---

### T-033 — controls-network
**Capability**: Implement `configure_firewalld` (CIS 3.3.x) — SHOULD
**Description**: Create `configure_firewalld.rs` — check firewalld or nftables active with default zone drop. If no firewall → NonCompliant with evidence.
**Test RED→GREEN→REFACTOR**:
- RED: Test on system with firewalld active (drop zone) → Compliant; with no firewall → NonCompliant
- GREEN: Implement control detecting firewall backend
- REFACTOR: Extract firewall detection to helper
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/network/configure_firewalld.rs`, `crates/vallumix-controls/src/network/mod.rs`

---

### T-034 — controls-network
**Capability**: Register all network controls in registry
**Description**: Update registry() to include all 9+ network controls. Add sysctl_ip_forwarding (already pilot).
**Test RED→GREEN→REFACTOR**:
- RED: Test registry_has_all_network_controls() — assert len >= 9
- GREEN: Add all network control entries
- REFACTOR: None
**Dependencies**: T-029 through T-033
**Files affected**: `crates/vallumix-controls/src/lib.rs`

---

## Phase 7: Controls — Logging (CIS 4.x)

### T-035 — controls-logging
**Capability**: Create logging module and rsyslog controls (CIS 4.1.1.x)
**Description**: Create `crates/vallumix-controls/src/logging/mod.rs`. Create `ensure_rsyslog_installed.rs` (check package), `ensure_rsyslog_configured.rs` (check auth,authpriv.* directive), `ensure_rsyslog_perms.rs` (check file ownership mode 0640).
**Test RED→GREEN→REFACTOR**:
- RED: Test each on fixture with/without rsyslog configured; test perms check
- GREEN: Implement all 3 rsyslog controls
- REFACTOR: Extract rsyslog conf parsing to helper
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/logging/mod.rs`, `crates/vallumix-controls/src/logging/ensure_rsyslog_installed.rs`, `crates/vallumix-controls/src/logging/ensure_rsyslog_configured.rs`, `crates/vallumix-controls/src/logging/ensure_rsyslog_perms.rs`

---

### T-036 — controls-logging
**Capability**: Implement journald controls (CIS 4.1.2.x)
**Description**: Create `ensure_journald_configured.rs` (check Storage=persistent), `ensure_journald_override.rs` (create drop-in under /etc/systemd/journald.conf.d/).
**Test RED→GREEN→REFACTOR**:
- RED: Test on fixture with/without persistent storage; test apply creates drop-in with backup
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/logging/ensure_journald_configured.rs`, `crates/vallumix-controls/src/logging/ensure_journald_override.rs`, `crates/vallumix-controls/src/logging/mod.rs`

---

### T-037 — controls-logging
**Capability**: Implement auditd controls (CIS 4.1.3.x)
**Description**: Create `ensure_auditd_installed.rs` (check package), `ensure_auditd_configured.rs` (check max_log_file_action = keep_logs).
**Test RED→GREEN→REFACTOR**:
- RED: Test on fixture with/without auditd; test configured vs non-configured
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/logging/ensure_auditd_installed.rs`, `crates/vallumix-controls/src/logging/ensure_auditd_configured.rs`, `crates/vallumix-controls/src/logging/mod.rs`

---

### T-038 — controls-logging
**Capability**: Implement audit rules controls (CIS 4.1.4.x, 4.1.5.x, 4.1.6.x)
**Description**: Create `ensure_audit_identity_rules.rs`, `ensure_audit_login_events.rs`, `ensure_audit_session_events.rs` — check rules in /etc/audit/rules.d/vallumix.rules, apply appends rules with backup.
**Test RED→GREEN→REFACTOR**:
- RED: Test check on fixture with/without rules; test apply creates backup and appends rules
- GREEN: Implement all 3 audit rule controls
- REFACTOR: Extract audit rules path to constant
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/logging/ensure_audit_identity_rules.rs`, `crates/vallumix-controls/src/logging/ensure_audit_login_events.rs`, `crates/vallumix-controls/src/logging/ensure_audit_session_events.rs`, `crates/vallumix-controls/src/logging/mod.rs`

---

### T-039 — controls-logging
**Capability**: Implement logrotate control (CIS 4.1.7) — SHOULD
**Description**: Create `ensure_logrotate.rs` — check /etc/logrotate.d/rsyslog exists with weekly rotation.
**Test RED→GREEN→REFACTOR**:
- RED: Test on fixture with/without logrotate config
- GREEN: Implement control
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/logging/ensure_logrotate.rs`, `crates/vallumix-controls/src/logging/mod.rs`

---

### T-040 — controls-logging
**Capability**: Register all logging controls in registry
**Description**: Update registry() to include all 10 logging controls with correct CIS IDs. Add `logging` module to controls lib.
**Test RED→GREEN→REFACTOR**:
- RED: Test registry_has_all_logging_controls() — assert len >= 10
- GREEN: Add all logging control entries
- REFACTOR: None
**Dependencies**: T-035 through T-039
**Files affected**: `crates/vallumix-controls/src/lib.rs`, `crates/vallumix-controls/src/logging/mod.rs`

---

## Phase 8: Controls — SSH (CIS 5.2.x)

### T-041 — controls-ssh
**Capability**: Create SSH module with 8 controls
**Description**: Create `crates/vallumix-controls/src/ssh/mod.rs` already exists. Add `ssh_ensure_protocol_2.rs` (Protocol 2 or default compliant), `ssh_set_loglevel.rs` (LogLevel INFO/VERBOSE only), `ssh_disable_empty_passwords.rs`, `ssh_max_auth_tries.rs` (MaxAuthTries ≤ 4).
**Test RED→GREEN→REFACTOR**:
- RED: Test each on fixture sshd_config with correct/incorrect values; test commented directive treated as non-compliant
- GREEN: Implement all 4 SSH controls
- REFACTOR: Extract sshd_config parsing to shared helper
**Dependencies**: T-001, T-002 (ssh_disable_root_login already pilot)
**Files affected**: `crates/vallumix-controls/src/ssh/ssh_ensure_protocol_2.rs`, `crates/vallumix-controls/src/ssh/ssh_set_loglevel.rs`, `crates/vallumix-controls/src/ssh/ssh_disable_empty_passwords.rs`, `crates/vallumix-controls/src/ssh/ssh_max_auth_tries.rs`, `crates/vallumix-controls/src/ssh/mod.rs`

---

### T-042 — controls-ssh
**Capability**: Implement SSH idle timeout and grace controls (CIS 5.2.6, 5.2.7)
**Description**: Create `ssh_client_alive_interval.rs` (ClientAliveInterval 300, ClientAliveCountMax 0), `ssh_login_grace_time.rs` (LoginGraceTime 60).
**Test RED→GREEN→REFACTOR**:
- RED: Test each on fixture with correct/incorrect values; test apply with backup
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/ssh/ssh_client_alive_interval.rs`, `crates/vallumix-controls/src/ssh/ssh_login_grace_time.rs`, `crates/vallumix-controls/src/ssh/mod.rs`

---

### T-043 — controls-ssh
**Capability**: Implement SSH access restriction and banner controls (CIS 5.2.8, 5.2.9)
**Description**: Create `ssh_limit_access.rs` (AllowUsers or AllowGroups configured), `ssh_set_banner.rs` (Banner /etc/issue.net).
**Test RED→GREEN→REFACTOR**:
- RED: Test: neither AllowUsers nor AllowGroups → NonCompliant; with AllowUsers → Compliant; test banner apply
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/ssh/ssh_limit_access.rs`, `crates/vallumix-controls/src/ssh/ssh_set_banner.rs`, `crates/vallumix-controls/src/ssh/mod.rs`

---

### T-044 — controls-ssh
**Capability**: Implement X11 and crypto policy controls (CIS 5.2.10, 5.2.11)
**Description**: Create `ssh_disable_x11_forwarding.rs` (X11Forwarding no), `ssh_set_crypto_policy.rs` (check crypto policy).
**Test RED→GREEN→REFACTOR**:
- RED: Test X11Forwarding yes → NonCompliant; no → Compliant; test crypto policy check
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/ssh/ssh_disable_x11_forwarding.rs`, `crates/vallumix-controls/src/ssh/ssh_set_crypto_policy.rs`, `crates/vallumix-controls/src/ssh/mod.rs`

---

### T-045 — controls-ssh
**Capability**: Register all SSH controls in registry
**Description**: Update registry() to include all 8 SSH controls. Ensure ssh_disable_root_login (pilot) has category() implemented.
**Test RED→GREEN→REFACTOR**:
- RED: Test registry_has_all_ssh_controls() — assert len >= 8
- GREEN: Add all SSH control entries
- REFACTOR: None
**Dependencies**: T-041 through T-044, T-002
**Files affected**: `crates/vallumix-controls/src/lib.rs`

---

## Phase 9: Controls — Auth/PAM (CIS 5.1/5.3-5.5)

### T-046 — controls-auth
**Capability**: Create auth module with 10 controls
**Description**: Create `crates/vallumix-controls/src/auth/mod.rs`. Create `ensure_cron_daemon.rs` (CIS 5.1.1), `ensure_pam_password_quality.rs` (CIS 5.3.1), `ensure_pam_minlen.rs` (CIS 5.3.2, minlen ≥ 14), `ensure_pam_credit.rs` (CIS 5.3.x, dcredit/ucredit/ocredit/lcredit all -1).
**Test RED→GREEN→REFACTOR**:
- RED: Test each on fixture with correct/incorrect PAM config; test apply writes correct settings with backup
- GREEN: Implement all 4 auth controls
- REFACTOR: Extract PAM config path detection to helper
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/auth/mod.rs`, `crates/vallumix-controls/src/auth/ensure_cron_daemon.rs`, `crates/vallumix-controls/src/auth/ensure_pam_password_quality.rs`, `crates/vallumix-controls/src/auth/ensure_pam_minlen.rs`, `crates/vallumix-controls/src/auth/ensure_pam_credit.rs`

---

### T-047 — controls-auth
**Capability**: Implement PAM lockout and history controls (CIS 5.3.3, 5.3.4)
**Description**: Create `ensure_pam_faillock.rs` (lockout after 5 failed attempts for 900s), `ensure_pam_remember.rs` (remember ≥ 5).
**Test RED→GREEN→REFACTOR**:
- RED: Test on fixture with/without faillock config; test apply adds pam_faillock entries
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/auth/ensure_pam_faillock.rs`, `crates/vallumix-controls/src/auth/ensure_pam_remember.rs`, `crates/vallumix-controls/src/auth/mod.rs`

---

### T-048 — controls-auth
**Capability**: Implement password hashing and umask controls (CIS 5.4.x, 5.5.x)
**Description**: Create `ensure_password_hashing.rs` (ENCRYPT_METHOD SHA512/yescrypt), `ensure_umask.rs` (umask 0077 or more restrictive in /etc/profile and /etc/bashrc).
**Test RED→GREEN→REFACTOR**:
- RED: Test on fixture with wrong encryption method; test umask less restrictive than 0027 → NonCompliant
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/auth/ensure_password_hashing.rs`, `crates/vallumix-controls/src/auth/ensure_umask.rs`, `crates/vallumix-controls/src/auth/mod.rs`

---

### T-049 — controls-auth
**Capability**: Implement shell timeout control (CIS 5.5.x)
**Description**: Create `ensure_shell_timeout.rs` — check TMOUT set to 300s or less in /etc/profile.
**Test RED→GREEN→REFACTOR**:
- RED: Test on fixture with TMOUT=600 → NonCompliant; TMOUT=300 → Compliant; no TMOUT → NonCompliant
- GREEN: Implement control
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/auth/ensure_shell_timeout.rs`, `crates/vallumix-controls/src/auth/mod.rs`

---

### T-050 — controls-auth
**Capability**: Register all auth controls in registry
**Description**: Update registry() to include all 10 auth controls with correct CIS IDs. Add `auth` module to controls lib.
**Test RED→GREEN→REFACTOR**:
- RED: Test registry_has_all_auth_controls() — assert len >= 10
- GREEN: Add all auth control entries
- REFACTOR: None
**Dependencies**: T-046 through T-049
**Files affected**: `crates/vallumix-controls/src/lib.rs`, `crates/vallumix-controls/src/auth/mod.rs`

---

## Phase 10: Controls — Maintenance (CIS 6.x)

### T-051 — controls-maintenance
**Capability**: Implement file permission controls (CIS 6.1.2, 6.1.3, 6.1.4)
**Description**: Create `ensure_perms_shadow.rs` (mode 0600), `ensure_perms_group.rs` (mode 0644), `ensure_perms_gshadow.rs` (mode 0600). ensure_perms_passwd (pilot 6.1.1) already exists.
**Test RED→GREEN→REFACTOR**:
- RED: Test each on fixture with correct/incorrect perms; test apply corrects perms with backup
- GREEN: Implement all 3 permission controls
- REFACTOR: Extract permission check logic to shared helper
**Dependencies**: T-001, T-002 (ensure_perms_passwd has category)
**Files affected**: `crates/vallumix-controls/src/maintenance/ensure_perms_shadow.rs`, `crates/vallumix-controls/src/maintenance/ensure_perms_group.rs`, `crates/vallumix-controls/src/maintenance/ensure_perms_gshadow.rs`, `crates/vallumix-controls/src/maintenance/mod.rs`

---

### T-052 — controls-maintenance
**Capability**: Implement audit-only controls (CIS 6.1.5, 6.1.6)
**Description**: Create `audit_world_writable.rs` (find o+w files, apply returns Skipped with warning), `audit_suid_sgid.rs` (find SUID/SGID outside allowlist, apply returns Skipped).
**Test RED→GREEN→REFACTOR**:
- RED: Test check finds world-writable files; test apply returns Skipped; test SUID control same
- GREEN: Implement both audit-only controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/maintenance/audit_world_writable.rs`, `crates/vallumix-controls/src/maintenance/audit_suid_sgid.rs`, `crates/vallumix-controls/src/maintenance/mod.rs`

---

### T-053 — controls-maintenance
**Capability**: Implement unowned files and duplicate ID controls (CIS 6.1.7, 6.1.8)
**Description**: Create `audit_unowned_files.rs` (find files with no owner), `audit_duplicate_ids.rs` (find duplicate UIDs/GIDs in passwd/group).
**Test RED→GREEN→REFACTOR**:
- RED: Test each on fixture with/without issues; test apply returns Skipped (audit-only)
- GREEN: Implement both controls
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/maintenance/audit_unowned_files.rs`, `crates/vallumix-controls/src/maintenance/audit_duplicate_ids.rs`, `crates/vallumix-controls/src/maintenance/mod.rs`

---

### T-054 — controls-maintenance
**Capability**: Implement cron permissions control (CIS 6.1.9) — SHOULD
**Description**: Create `ensure_cron_perms.rs` — check /etc/cron.d, cron.daily, etc. have appropriate perms (0700 for dirs, 0644 for files).
**Test RED→GREEN→REFACTOR**:
- RED: Test on fixture with correct/incorrect cron directory perms
- GREEN: Implement control
- REFACTOR: None
**Dependencies**: T-001, T-002
**Files affected**: `crates/vallumix-controls/src/maintenance/ensure_cron_perms.rs`, `crates/vallumix-controls/src/maintenance/mod.rs`

---

### T-055 — controls-maintenance
**Capability**: Register all maintenance controls in registry
**Description**: Update registry() to include all 8 maintenance controls. Ensure ensure_perms_passwd (pilot) has category() implemented.
**Test RED→GREEN→REFACTOR**:
- RED: Test registry_has_all_maintenance_controls() — assert len >= 8
- GREEN: Add all maintenance control entries
- REFACTOR: None
**Dependencies**: T-051 through T-054, T-002
**Files affected**: `crates/vallumix-controls/src/lib.rs`

---

## Phase 11: Profile Population

### T-056 — profile-model
**Capability**: Populate database.toml with ≥20 control IDs
**Description**: Add to `profiles/database.toml` controls relevant to database servers: filesystem mount options, kernel hardening, SUID/SGID audit, SSH hardening, audit controls, PAM policies. Use actual CIS IDs from implemented controls.
**Test RED→GREEN→REFACTOR**:
- RED: Test `Profile::from_file("profiles/database.toml")` → len >= 20; test `resolve_controls(&registry)` succeeds
- GREEN: Write control IDs to database.toml
- REFACTOR: Ensure all IDs resolve in registry
**Dependencies**: T-022, T-028, T-034, T-040, T-045, T-050, T-055 (all controls registered)
**Files affected**: `profiles/database.toml`

---

### T-057 — profile-model
**Capability**: Populate bastion.toml with ≥25 control IDs
**Description**: Add to `profiles/bastion.toml` controls for aggressive hardening: SSH only exposed, key-based auth, extensive auditd, strict PAM, maximum service disable. Use actual CIS IDs.
**Test RED→GREEN→REFACTOR**:
- RED: Test `Profile::from_file("profiles/bastion.toml")` → len >= 25; test resolve_controls succeeds
- GREEN: Write control IDs to bastion.toml
- REFACTOR: None
**Dependencies**: T-056 (same controls, different profile)
**Files affected**: `profiles/bastion.toml`

---

### T-058 — profile-model
**Capability**: Update web.toml to include only implemented control IDs
**Description**: Audit web.toml — remove placeholder IDs not yet implemented (4.1.1.1 etc.). Verify all listed IDs exist in registry.
**Test RED→GREEN→REFACTOR**:
- RED: Test all web.toml IDs resolve in registry — any unknown ID causes failure
- GREEN: Remove unimplemented IDs, keep all pilots + implemented controls
- REFACTOR: None
**Dependencies**: All control implementation tasks
**Files affected**: `profiles/web.toml`

---

## Phase 12: CLI Orchestration

### T-059 — cli-orchestration
**Capability**: Wire apply command full workflow
**Description**: Implement `apply.rs run()` — load profile, resolve controls, create backup session, iterate (check → backup → apply → post-verify), generate reports in all requested formats, exit 0/1/2/3. Dry-run skips apply and backup.
**Test RED→GREEN→REFACTOR**:
- RED: Write assert_cmd test: `vallumix apply --profile web` → exit 0 when compliant; `vallumix apply --profile web --dry-run` → no backups created
- GREEN: Implement full orchestration
- REFACTOR: Extract profile loading and report generation to helpers
**Dependencies**: T-003, T-004, T-007 through T-011, T-012 through T-015, T-059 parallelization with rayon
**Files affected**: `crates/vallumix-cli/src/commands/apply.rs`, `crates/vallumix-cli/src/commands/mod.rs`

---

### T-060 — cli-orchestration
**Capability**: Wire audit command with rayon parallelization
**Description**: Implement `audit.rs run()` — load profile, resolve controls, use `rayon::par_iter()` to call check() on all controls concurrently, collect results, generate reports. Must NOT modify system.
**Test RED→GREEN→REFACTOR**:
- RED: Write assert_cmd test: `vallumix audit --profile web --report html` → generates HTML; verify no files modified after run
- GREEN: Implement with rayon par_iter for check phase
- REFACTOR: None
**Dependencies**: T-059 (apply wired first as reference)
**Files affected**: `crates/vallumix-cli/src/commands/audit.rs`

---

### T-061 — cli-orchestration
**Capability**: Wire rollback command with BackupManager
**Description**: Implement `rollback.rs run()` — with --control-id: call rollback_by_control(); without: call rollback_session(most_recent). Show restored control count.
**Test RED→GREEN→REFACTOR**:
- RED: Write assert_cmd test: apply then `vallumix rollback --control-id 5.2.4` → restore; `vallumix rollback` → restore entire session
- GREEN: Implement using BackupManager methods
- REFACTOR: None
**Dependencies**: T-009, T-010 (BackupManager rollback methods)
**Files affected**: `crates/vallumix-cli/src/commands/rollback.rs`

---

### T-062 — cli-orchestration
**Capability**: Wire completion command with clap_complete
**Description**: Implement `completion.rs run()` — use `clap_complete::generate()` to output shell completion for bash/zsh/fish/nushell.
**Test RED→GREEN→REFACTOR**:
- RED: Write assert_cmd test: `vallumix completion bash` → output starts with `# bash completion for vallumix`
- GREEN: Implement with clap_complete
- REFACTOR: None
**Dependencies**: None
**Files affected**: `crates/vallumix-cli/src/commands/completion.rs`

---

### T-063 — cli-orchestration
**Capability**: Add indicatif progress bar to apply and audit
**Description**: Add progress bar to apply/audit showing "[N/M] control-id description..." during control iteration. Suppressed by --quiet.
**Test RED→GREEN→REFACTOR**:
- RED: Test progress bar appears during apply (not --quiet); test suppressed with --quiet flag
- GREEN: Add indicatif ProgressBar with MultiProgress
- REFACTOR: None
**Dependencies**: T-059, T-060
**Files affected**: `crates/vallumix-cli/src/commands/apply.rs`, `crates/vallumix-cli/src/commands/audit.rs`

---

## Phase 13: Integration Tests

### T-064 — integration-tests
**Capability**: Add insta snapshots for all 4 reporters
**Description**: Add insta snapshot tests for HtmlReporter, JunitReporter, TextReporter, JsonReporter — capture full output as snapshots. Add `cargo insta review` to CI.
**Test RED→GREEN→REFACTOR**:
- RED: Run `cargo insta test` → snapshots created for all 4 reporters
- GREEN: Verify snapshots are committed
- REFACTOR: Update snapshots if format changes intentionally
**Dependencies**: T-012, T-013, T-014, T-015
**Files affected**: `crates/vallumix-reporters/src/html.rs`, `crates/vallumix-reporters/src/junit.rs`, `crates/vallumix-reporters/src/text.rs`, `crates/vallumix-reporters/src/json.rs` (test modules)

---

### T-065 — integration-tests
**Capability**: Add assert_cmd integration tests for CLI
**Description**: Add integration tests in `vallumix-cli/tests/` using assert_cmd — test apply, audit, rollback, completion end-to-end with real binary.
**Test RED→GREEN→REFACTOR**:
- RED: Write test binaries in tmpdir, assert exit codes and output format
- GREEN: Implement tests verifying exit codes (0/1/2/3), report format correctness
- REFACTOR: None
**Dependencies**: T-059 through T-063
**Files affected**: `crates/vallumix-cli/tests/`

---

### T-066 — integration-tests
**Capability**: Add profile resolution integration tests
**Description**: Add test loading database.toml and bastion.toml, resolving all control IDs against registry, verifying no missing IDs.
**Test RED→GREEN→REFACTOR**:
- RED: Test resolve_controls on database and bastion profiles — any unresolved ID fails test
- GREEN: Ensure all profile IDs are registered
- REFACTOR: None
**Dependencies**: T-056, T-057
**Files affected**: `crates/vallumix-core/src/profile.rs` (test module) or integration test file

---

### T-067 — integration-tests
**Capability**: Run full test suite and clippy
**Description**: Run `cargo test --workspace` and `cargo clippy -- -D warnings` — fix any issues from Phase 2 additions. Verify all 71 existing tests still pass.
**Test RED→GREEN→REFACTOR**:
- RED: Run test suite → any failure is regression
- GREEN: Fix any regressions or new compilation errors
- REFACTOR: None
**Dependencies**: All previous tasks
**Files affected**: All crates — workspace-wide verification

---

## Summary

| Phase | Tasks | Focus |
|-------|-------|-------|
| Phase 1 | T-001–T-006 | Infrastructure: Category enum, trait methods, CLI arg expansion |
| Phase 2 | T-007–T-011 | BackupManager: session tracking, list, prune, rollback, checksum |
| Phase 3 | T-012–T-015 | Reporters: HTML (askama), JUnit (quick-xml), Text (owo-colors), JSON refactor |
| Phase 4 | T-016–T-022 | Filesystem controls: 8 controls, modprobe pattern, category |
| Phase 5 | T-023–T-028 | Services controls: 9 controls, systemctl pattern |
| Phase 6 | T-029–T-034 | Network controls: 9 controls, sysctl drop-in pattern |
| Phase 7 | T-035–T-040 | Logging controls: 10 controls, rsyslog/journald/auditd |
| Phase 8 | T-041–T-045 | SSH controls: 8 controls, sshd_config + drop-in pattern |
| Phase 9 | T-046–T-050 | Auth/PAM controls: 10 controls, PAM module pattern |
| Phase 10 | T-051–T-055 | Maintenance controls: 8 controls, permission + audit pattern |
| Phase 11 | T-056–T-058 | Profile population: database.toml (≥20), bastion.toml (≥25), web.toml update |
| Phase 12 | T-059–T-063 | CLI orchestration: apply, audit (rayon), rollback, completion, progress bars |
| Phase 13 | T-064–T-067 | Integration tests: insta snapshots, assert_cmd, clippy |
| **Total** | **67** | |

**Implementation Order Rationale**:
1. Core traits (Category, trait methods) are needed by ALL controls
2. BackupManager session tracking needed before apply CLI can work
3. Reporters are independent — parallelize
4. Controls by CIS category build on each other (filesystem → services → network → logging → ssh → auth → maintenance)
5. Profile population requires all controls registered first
6. CLI orchestration wires everything — requires all above
7. Integration tests verify end-to-end

**Files Created/Modified**: ~80 files across 5 crates + 3 profile TOMLs
**New Dependencies**: rayon, askama, quick-xml, indicatif, sha2, clap_complete, insta, assert_cmd