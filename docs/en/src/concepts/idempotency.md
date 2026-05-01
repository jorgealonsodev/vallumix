# Idempotency

Idempotency is one of the fundamental properties of Vallumix. It means that running the tool once, twice, or a hundred times on the same server produces the same final state, with no cumulative side effects or duplicated changes.

## Why Does It Matter?

In production environments, hardening is not a one-time event. Servers are reprovisioned, configurations are adjusted, and security policies evolve. If a hardening tool is not idempotent, each additional execution can:

- Accumulate duplicate rules in configuration files.
- Add multiple identical entries in cron or systemd.
- Overwrite previous backups with later versions, losing traceability.
- Generate inconsistent reports between executions.

Vallumix avoids all these problems by designing each control to be inherently idempotent.

## How Vallumix Guarantees Idempotency

### Pre-check Before Applying

Before modifying any file, each control executes a `pre_check` phase that evaluates whether the system already complies with the recommendation. If the control determines that the current state is compliant, it skips directly to the next without touching anything.

```rust
fn apply(&self, ctx: &Context) -> Result<ApplyResult, ControlError> {
    match self.check(ctx)? {
        CheckResult::Compliant => {
            return Ok(ApplyResult::SkippedAlreadyCompliant);
        }
        CheckResult::NonCompliant => {
            // Proceed to backup and application
        }
    }
}
```

### Conditional Backup

Backups are only created when a control is actually going to modify something. If all controls in an execution are already compliant, no new empty backup is generated.

### Atomic Applications

When a control modifies a file, it does so atomically: it writes the new content to a temporary file and then uses `rename` to replace the original. This guarantees that the file never ends up in a corrupt intermediate state, even if the process is interrupted.

### Post-check Verification

After applying a change, the control executes `post_check` to confirm that the system state matches what was expected. If it does not match, the result is marked as failed and the report documents the discrepancy.

## Dry-run as Idempotency Validation

The `--dry-run` mode is the most effective tool for verifying idempotency without risk:

```bash
sudo vallumix apply --profile web --dry-run --verbose
```

In this mode, Vallumix executes `pre_check` for all controls and reports what changes *would be made*, but does not execute `backup` or `apply`. If you run `--dry-run` immediately after a successful `apply`, you should see all controls appearing as `Compliant` or `SkippedAlreadyCompliant`.

```tip
Incorporate `vallumix apply --profile web --dry-run` into your Ansible playbooks or provisioning scripts as a validation step before marking a server as ready. If the dry-run reports pending changes, it means something in your previous process was not idempotent.
```

## Idempotency and Reports

Reports reflect idempotency through the following states:

- **Compliant:** the control already complied before execution.
- **Remediated:** the control did not comply and was successfully applied.
- **SkippedAlreadyCompliant:** explicit variant of `Compliant` due to idempotency.
- **Failed:** the control did not comply, remediation was attempted, but the `post_check` failed.

An idempotent report on an already hardened server should show predominantly `Compliant` and `SkippedAlreadyCompliant`, with zero `Remediated` entries.
