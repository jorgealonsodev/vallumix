# Controls

A control is the atomic unit of work in Vallumix. It represents an individual security check aligned with the CIS Benchmark, along with the necessary logic to evaluate the current system state and, if needed, apply a remediation.

## What Is a CIS Control?

The Center for Internet Security (CIS) publishes benchmarks that break down secure operating system configuration into hundreds of numbered recommendations. Each recommendation is called a "control" and has an identifier such as `5.2.4` (disable direct root login via SSH) or `1.1.1.1` (disable cramfs support).

Vallumix implements a selection of these controls — 70 in version 1.0 — distributed across six functional categories:

1. **Initial setup:** filesystems, automatic updates, package integrity.
2. **Services:** identification and deactivation of unnecessary services.
3. **Network:** kernel parameters for TCP/IP, IPv6, ICMP, firewall.
4. **Logging and auditing:** `rsyslog`, `journald`, `auditd`.
5. **Access, authentication, and authorization:** PAM, passwords, SSH, sudo, cron.
6. **System maintenance:** permissions, integrity of `/etc/passwd`, `umask`.

## The `Control` Trait

In Vallumix, each control is implemented as a Rust structure that implements the `Control` trait. This abstraction allows the main engine to iterate over `Box<dyn Control>` without coupling to the specific logic of each verification.

```rust
pub trait Control: Send + Sync {
    fn id(&self) -> &str;                          // e.g. "5.2.4"
    fn description(&self) -> &str;
    fn severity(&self) -> Severity;
    fn applicable_distros(&self) -> &[Distro];

    fn check(&self, ctx: &Context) -> Result<CheckResult, ControlError>;
    fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError>;
    fn rollback(&self, ctx: &Context, backup: &Backup) -> Result<(), ControlError>;
}
```

Each method has a clear responsibility:

- **`id`**: returns the CIS identifier of the control (for example, `5.2.4`).
- **`description`**: human-readable description of what it checks and remediates.
- **`severity`**: criticality level of the control.
- **`applicable_distros`**: list of distributions where the control makes sense.
- **`check`**: evaluates the current state without modifying anything. Returns `Compliant` or `NonCompliant`.
- **`apply`**: applies the remediation. Only invoked in `apply` mode, never in `audit`.
- **`rollback`**: reverts changes applied by this control using the provided backup.

## Severity Levels

Each control is classified according to the potential impact of its non-compliance:

| Severity | Meaning | Example |
|---|---|---|
| **Critical** | Gap that allows direct system compromise | Allowing SSH login as root |
| **High** | Insecure configuration with significant impact | Not disabling IPv6 if unused |
| **Medium** | Deviation that increases attack surface | Unnecessary services active |
| **Low** | Good practice recommendation | MOTD banner configuration |

## Code Organization

Controls are organized in `vallumix-controls` by functional domain:

```
vallumix-controls/src/
├── filesystem/
│   ├── mod.rs
│   └── disable_cramfs.rs      # CIS 1.1.1.1
├── ssh/
│   ├── mod.rs
│   └── disable_root_login.rs  # CIS 5.2.4
├── network/
│   └── ...
└── ...
```

Each file implements a single control structure and should not exceed 200 lines of effective code. If a control requires complex logic, it is decomposed into private helper functions.

```tip
You can list all available controls with `vallumix list --profile web`. This shows the CIS ID, description, severity, and current compliance status without applying any changes.
```
