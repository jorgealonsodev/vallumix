# Security Policy

## Important Notice

Vallumix is a security tool that modifies system-level configurations. We take all security reports seriously. If you discover a vulnerability in Vallumix itself — not in the systems it hardens — please report it responsibly so we can fix it before public disclosure.

## Supported Versions

Only the latest released version of Vallumix receives security updates. Please keep your installations up to date.

| Version | Supported |
|--------|-----------|
| Latest | Yes |
| Older | No |

## How to Report a Vulnerability

### Preferred Method

Submit a private security advisory through GitHub:

[https://github.com/jorgealonsodev/vallumix/security/advisories/new](https://github.com/jorgealonsodev/vallumix/security/advisories/new)

This ensures the report remains confidential while we investigate and prepare a fix.

### Alternative Method

If you cannot use GitHub Security Advisories, you may email:

```
security@vallumix.example
```

> **Note**: This is a placeholder address. Please use GitHub Security Advisories as the primary channel.

## Response Timeline

| Phase | Timeframe |
|-------|-----------|
| Acknowledgment | Within 48 hours |
| Initial assessment | Within 7 days |
| Fix and release | Within 30 days (critical), 90 days (non-critical) |
| Public disclosure | Coordinated with reporter after fix is available |

We will keep you informed of our progress throughout the process. If we are unable to meet these timelines, we will explain why and provide an updated estimate.

## What to Include in Your Report

- A clear description of the vulnerability and its impact
- Steps to reproduce the issue
- Affected versions
- Any proposed mitigations or patches
- Your preferred disclosure timeline

## Artifact Verification

All release artifacts are signed and attested:

- **Cosign**: Release binaries and packages are signed with Sigstore cosign using keyless signing.
- **SLSA**: Provenance attestations are generated at SLSA Level 3 via the [slsa-github-generator](https://github.com/slsa-framework/slsa-github-generator).
- **SBOM**: CycloneDX SBOMs are published with each release for dependency transparency.

You can verify signatures using:

```bash
cosign verify-blob \
  --certificate-identity-regexp '^https://github.com/jorgealonsodev/vallumix/' \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --signature vallumix.sig \
  --certificate vallumix.crt.pem \
  vallumix
```

## Scope

Reports should focus on vulnerabilities in Vallumix code, build pipeline, or distribution mechanisms. Reports about the security posture of systems hardened by Vallumix (e.g., "this SSH configuration is weak") should be filed as regular issues or feature requests, not security advisories.

## Disclosure Policy

We follow a coordinated disclosure model:

1. Report received and acknowledged.
2. Fix developed and tested privately.
3. Patch released and users notified.
4. Full details disclosed publicly after a reasonable grace period (typically 30 days).

We credit reporters who follow this process in our advisory notes unless they request anonymity.
