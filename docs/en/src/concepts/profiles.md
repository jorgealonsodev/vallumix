# Profiles

Profiles are the mechanism by which Vallumix adapts the selection and severity of controls to the operational role of the server. Instead of applying generic hardening that might break legitimate services, each profile prioritizes relevant controls and omits or softens those that do not make sense for the context.

## Web Profile

The `web` profile is optimized for servers that host HTTP/HTTPS applications through Nginx, Apache, or other web servers.

### What It Includes

- Network filtering controls focused on ports 80 and 443.
- Resource limits (`ulimit`, `systemd` limits) for web server processes.
- TLS configuration hardening: minimum protocol versions, secure ciphersuites, disabling self-signed certificates.
- Restrictive permissions on deployment directories (`/var/www`, `/usr/share/nginx/html`).
- Structured logging policies for forensic analysis of access and errors.
- Disabling unnecessary services that do not affect web functionality (avahi, cups, etc.).

### When to Use It

- Front-end servers that expose web applications.
- Reverse proxy load balancers.
- Static file servers.

```note
The web profile does not disable the HTTP service or restrict access to web ports. Its goal is to harden the underlying host without breaking the server's functionality.
```

## Database Profile

The `database` profile is designed for hosts that run database engines such as PostgreSQL, MariaDB, or MongoDB.

### What It Includes

- Network access restriction to internal interfaces or localhost.
- Strict controls on filesystem mounts, especially `/tmp` and `/var`, with `noexec`, `nodev`, `nosuid` options.
- Kernel hardening for I/O-intensive workloads (`vm.swappiness`, `dirty_ratio`).
- Deactivation of non-essential SUID/SGID binaries.
- PAM configuration and resource limits for the database engine user.
- Audit logging for connections and administrative queries.

### When to Use It

- Dedicated database servers.
- Database replica or cluster nodes.
- Instances where the database engine is the main service.

## Bastion Profile

The `bastion` profile is the most aggressive of the three. It is intended for jump hosts whose sole purpose is to serve as an authenticated SSH entry point to internal infrastructure.

### What It Includes

- Only port 22 (SSH) exposed; everything else filtered.
- Mandatory public key authentication; passwords disabled.
- Optional multi-factor authentication (MFA) support with Google Authenticator.
- Exhaustive session logging with `auditd` and `script`.
- Extensive command and environment restrictions via `ForceCommand`, `ChrootDirectory`, and `Match` blocks.
- Maximum hardening of PAM, sudo, and cron.

### When to Use It

- Remote access bastions to internal infrastructure.
- Centralized administration servers.
- Any host whose only function is SSH.

```danger
The bastion profile can break services that are not SSH. Do not apply it to web servers, databases, or any host that needs to run other services. Always run `--dry-run` before applying this profile in a new environment.
```

## Quick Comparison

| Aspect | Web | Database | Bastion |
|---|---|---|---|
| Default open ports | 80, 443 | Database engine port | 22 |
| Aggressiveness | Medium | Medium-High | Maximum |
| Risk of service breakage | Low | Low-Medium | High if not SSH-only |
| Network controls | Firewall for web | Restricted interfaces | SSH only |
| Logging | Web access | Query auditing | Complete SSH sessions |

## Profile Selection

Use the `--profile` flag in any subcommand:

```bash
vallumix apply --profile web
vallumix audit --profile database --report html
vallumix list --profile bastion
```
