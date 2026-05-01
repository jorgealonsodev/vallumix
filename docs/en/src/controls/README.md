# Control Catalog

Vallumix implements **70 CIS Benchmark controls** organized across **7 security domains**. Each control is an atomic unit of verification and remediation that checks a specific configuration and, if non-compliant, applies the recommended hardening change.

Controls follow the CIS Benchmark numbering scheme (for example, `1.1.1.1` for filesystem, `5.2.4` for SSH). Every control implements the `Control` trait: it verifies the current state, applies remediation if needed, and verifies again to confirm the fix.

## The 7 Domains

| Domain | Count | CIS Section | Focus |
|--------|-------|-------------|-------|
| **Filesystem** | 9 | 1.1.x | Disable unused filesystem modules, harden mount options |
| **Services** | 12 | 2.2.x | Stop and mask unnecessary network services |
| **Network** | 9 | 3.1.x – 3.3.x | Kernel networking parameters, firewall configuration |
| **Logging and Audit** | 11 | 4.1.x | rsyslog, journald, auditd, log rotation, file permissions |
| **SSH** | 11 | 5.2.x | SSH server hardening: protocol, crypto, access, session |
| **Access and Authentication** | 10 | 5.1.x, 5.3.x – 5.5.x | PAM, password policy, umask, shell timeout |
| **System Maintenance** | 8 | 6.1.x | Critical file permissions, SUID/SGID audit, cron security |

```tip
Not all controls apply to all server roles. Use a **profile** (`web`, `database`, `bastion`) to select only the controls relevant to your server's function. Profiles are defined in TOML files under `profiles/`.
```

## Control Reference Table

The following table lists a representative sample of the 70 controls implemented by Vallumix. Severity levels are assigned based on CIS recommendations and the potential impact of non-compliance.

| CIS ID | Description | Severity | Profiles | Distros | NIST 800-53 | ISO 27001 |
|--------|-------------|----------|----------|---------|-------------|-----------|
| `1.1.1.1` | Disable cramfs filesystem module | Low | web, database, bastion | All | CM-7 | A.8.1 |
| `1.1.1.7` | Disable udf filesystem module | Low | web, database, bastion | All | CM-7 | A.8.1 |
| `1.1.10` | Disable USB storage module | Medium | web, database, bastion | All | MP-7 | A.8.10 |
| `2.2.3` | Disable Avahi mDNS/DNS-SD daemon | Medium | web, database, bastion | All | CM-7 | A.8.1 |
| `2.2.8` | Disable DNS server (named) | Medium | database, bastion | All | CM-7 | A.8.1 |
| `2.2.14` | Disable SNMP daemon | Medium | web, database, bastion | All | CM-7 | A.13.1 |
| `3.1.1` | Disable IP forwarding | High | web, database, bastion | All | SC-7 | A.13.1 |
| `3.2.7` | Enable TCP SYN cookies | Medium | web, database, bastion | All | SC-5 | A.13.1 |
| `3.3.1` | Ensure firewalld is configured | High | web, database, bastion | All | SC-7 | A.13.1 |
| `4.1.1.1` | Ensure rsyslog is installed | Medium | web, database, bastion | All | AU-6 | A.12.4 |
| `4.1.3.1` | Ensure auditd is installed | Medium | web, database, bastion | All | AU-6 | A.12.4 |
| `5.2.4` | Disable SSH root login | High | web, database, bastion | All | IA-2 | A.9.2 |
| `5.2.4b` | Set SSH MaxAuthTries to 4 or less | Medium | web, database, bastion | All | IA-6 | A.9.4 |
| `5.3.4` | Ensure PAM faillock is configured | Medium | web, database, bastion | All | AC-7 | A.9.4 |
| `6.1.1` | Ensure permissions on /etc/passwd | Medium | web, database, bastion | All | AC-3 | A.9.1 |
| `6.1.6` | Audit SUID and SGID executables | Medium | web, database, bastion | All | AC-3 | A.9.1 |

## How to Use This Catalog

- **Browse by domain**: the left sidebar groups controls into the 7 domains listed above.
- **Search by CIS ID**: each page lists controls in CIS order with their exact ID.
- **Check profile coverage**: the `Profiles` column indicates which server roles include the control.
- **Verify manually**: every control page includes the manual verification command so you can confirm Vallumix's work independently.

```tip
Vallumix is idempotent: running `apply` multiple times on the same profile produces the same final state. Controls that are already compliant are skipped automatically.
```

For the full list of all 70 controls, inspect the source in `crates/vallumix-controls/src/lib.rs` or the profile definitions in `profiles/web.toml`, `profiles/database.toml`, and `profiles/bastion.toml`.
