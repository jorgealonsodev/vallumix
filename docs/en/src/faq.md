# Frequently Asked Questions

## 1. Does Vallumix replace OpenSCAP or Lynis?

No. Vallumix occupies a different segment: it is a modern, self-contained tool written in Rust that applies changes and allows rollback. OpenSCAP is more comprehensive in benchmark coverage but requires SCAP content infrastructure. Lynis audits but does not apply automatic remediations. Use the tool that best fits your workflow.

## 2. Is it safe to run Vallumix in production?

Yes, if you follow the recommended precautions: run `--dry-run` first, review the report, ensure you have access to the physical console or VNC in case of SSH lockout, and use the appropriate profile for the server's role. The rollback system allows you to revert changes if something goes wrong.

## 3. Can I skip specific controls?

Yes. Edit the TOML profile file (`/etc/vallumix/profiles/web.toml` or a local copy) and comment out or remove the control you want to skip. Then use your custom profile with `--profile /path/to/my-profile.toml`.

## 4. How much disk space do I need?

Vallumix requires at least 100 MB free in `/var/backups/vallumix` for the initial backup session. The binary occupies less than 8 MB. HTML reports are typically 50-200 KB.

## 5. Can I use Vallumix in Docker containers?

This is not the intended use case. CIS controls are designed for complete operating systems, not for containers. Container hardening requires the CIS Docker Benchmark, which is a different approach. Vallumix will abort in most containers because it will not detect a fully supported distribution.

## 6. How do I update Vallumix?

Download the new version from the release and install it over the previous one. Backups in `/var/backups/vallumix` are preserved. After updating, run `vallumix --version` to confirm.

```bash
wget https://github.com/tu-org/vallumix/releases/download/v1.1.0/vallumix_1.1.0_amd64.deb
sudo dpkg -i vallumix_1.1.0_amd64.deb
```

## 7. Do backups expire?

No. Vallumix does not delete backups automatically. Configure a cron task to clean up old sessions if disk space is limited.

## 8. Can I apply multiple profiles on the same server?

This is not recommended. Profiles may have overlapping controls with conflicting configurations. If you need features from multiple profiles, create a custom profile that combines the desired controls without duplicates.

## 9. What happens if a control fails?

A control in `Failed` state means the remediation could not be applied or the post-check did not confirm the change. The server continues to function; Vallumix does not abort execution because of a failed control. Review the detailed report to understand the cause and fix manually if necessary.

## 10. Does Vallumix modify user passwords?

No. Vallumix does not manage user passwords, nor does it create or delete accounts. Authentication controls are limited to system configurations such as PAM, SSH, and sudoers.

## 11. How do I report a bug or request a feature?

Open an issue in the GitHub repository. For bugs, include the distribution, Vallumix version, the executed command, and the complete error message. For feature requests, describe the use case and why the current functionality does not cover it.

## 12. Does Vallumix guarantee the security of my server?

No. Vallumix automates CIS controls that reduce the attack surface, but security is a continuous process, not a product. Keep applying patches, monitor logs, perform periodic audits, and complement with intrusion detection tools. Vallumix is one piece of the puzzle, not the complete puzzle.

```tip
If your question is not here, review the [troubleshooting section](troubleshooting.md) or search the open issues in the repository. Someone has likely already encountered the same problem.
```
