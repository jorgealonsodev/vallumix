# Guides

This section contains practical guides oriented to real-world use cases. Each guide starts from a concrete scenario, provides the exact commands, and explains the expected result.

## Covered Use Cases

Vallumix was designed in response to five recurring scenarios in Linux server operations:

| Use Case | Scenario | Guide |
|---|---|---|
| **UC-01: Initial Hardening** | New web server, you need to harden it without breaking Nginx | [Web Server Hardening](guides/hardening-web-server.md) |
| **UC-02: Compliance Audit** | Evaluate security posture without modifying anything | [Compliance Audit](guides/compliance-audit.md) |
| **UC-03: Pre-Validation** | Review planned changes before touching production | [Dry-run Validation](guides/dry-run-validation.md) |
| **UC-04: Rollback After Incident** | A service stopped working after applying hardening | [Rollback After Incident](guides/rollback-after-incident.md) |
| **UC-05: CI/CD Integration** | Automate hardening in image build pipelines | [CI/CD Integration](guides/ci-cd-integration.md) |

## Which Guide Do You Need?

- If you are a **system administrator** and just provisioned a server → UC-01.
- If you are a **security consultant** and need a report for the client → UC-02.
- If you are a **platform engineer** and do not touch production without validating first → UC-03.
- If you are an **SRE** and received an alert after a hardening change → UC-04.
- If you are a **DevOps** engineer building base images with Packer or Terraform → UC-05.

```tip
All guides assume Vallumix is installed and that you have root access on the target system. If you have not installed Vallumix yet, start with the [installation section](installation/README.md).
```

## Conventions Used in the Guides

- Commands preceded by `$` are run as a normal user.
- Commands preceded by `#` are run as root.
- Expected output is shown in code blocks without a prompt.
- Blocks ```` ```admonish ```` mark warnings about destructive operations or privilege requirements.
