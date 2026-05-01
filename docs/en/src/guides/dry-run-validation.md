# Dry-Run Validation (UC-03)

Dry-run mode allows you to review exactly what changes Vallumix would make before applying them. It is a mandatory practice in production environments where any unplanned modification can cause service interruptions.

## Scenario

- **Environment:** Pre-production that faithfully replicates the production environment.
- **Server role:** SSH bastion on Rocky Linux 9.
- **Objective:** Validate that the `bastion` profile does not break administrative SSH access before applying it in production.
- **Concern:** The bastion profile disables password authentication and could block access if keys are not correctly distributed.

## Step 1: Verify SSH Key Access

Before any dry-run of the bastion profile, confirm that you can access via public key:

```bash
# From your workstation
ssh -o PasswordAuthentication=no admin@bastion-preprod
# If it fails, DO NOT apply the bastion profile until resolved
```

```danger
The bastion profile explicitly disables `PasswordAuthentication` in `sshd_config`. If you access by password and apply this profile, you will lock yourself out. Verify key access before continuing.
```

## Step 2: Execute Dry-Run

```bash
sudo vallumix apply --profile bastion --dry-run --verbose --report json --output /tmp/dry-run-bastion
```

### What Dry-Run Does Exactly

- Executes `check` on all controls in the profile, just like `audit`.
- For `NonCompliant` controls, generates a report entry describing **what change would be made**, including:
  - File that would be modified.
  - Current value vs target value.
  - CIS justification.
- **Does not execute `backup` or `apply`**. The system remains intact.

### Expected Output (Excerpt)

```text
[Dry-run] 5.2.4  Ensure SSH root login is disabled
  → Would set: PermitRootLogin no
  → In file:   /etc/ssh/sshd_config
  → Current:   PermitRootLogin yes
  → Justification: CIS 5.2.4 — Root login via SSH increases attack surface

[Dry-run] 5.2.8  Ensure SSH password authentication is disabled
  → Would set: PasswordAuthentication no
  → In file:   /etc/ssh/sshd_config
  → Current:   PasswordAuthentication yes
  → Justification: CIS 5.2.8 — Passwords are vulnerable to brute-force attacks

[Dry-run] 3.4.2  Ensure default deny firewall policy
  → Would execute: nftables rule set
  → Current:       no active firewall rules
  → Impact:        Port 22 will be explicitly allowed; all others dropped
```

## Step 3: Review High-Risk Controls

In the JSON report, filter controls that could impact critical operations:

```bash
jq '.controls[] | select(.impact == "High")' /tmp/dry-run-bastion.json
```

Pay special attention to:

- SSH controls that change authentication methods.
- Firewall controls that restrict ports.
- PAM controls that affect password policies or account lockout.
- Sudo controls that limit administrative privileges.

## Step 4: Adjust the Environment If Necessary

If the dry-run reveals that a control would break something essential, you have two options before applying:

1. **Modify the environment:** for example, distribute SSH keys to all administrators before disabling passwords.
2. **Skip the specific control:** edit the TOML profile and comment out the problematic control. This option is documented in the profile reference.

## Step 5: Apply After Successful Validation

Once the dry-run shows no surprises:

```bash
sudo vallumix apply --profile bastion --report html,json --output /tmp/bastion-applied
```

```tip
Automate dry-run in your CI pipeline: run `vallumix apply --profile $PROFILE --dry-run --report json` in the validation stage. If the report contains controls with `impact: "High"` that are not in an approved exception list, fail the pipeline and require manual review.
```

## Step 6: Compare Dry-Run vs Actual Execution

For internal audit, compare the reports:

```bash
# Controls that dry-run said it would remediate
jq '.controls[] | select(.dry_run_action == "WouldRemediate") | .id' /tmp/dry-run-bastion.json | sort > /tmp/expected.txt

# Controls that were actually remediated
jq '.controls[] | select(.status == "Remediated") | .id' /tmp/bastion-applied.json | sort > /tmp/actual.txt

# Should be identical
diff /tmp/expected.txt /tmp/actual.txt
```

If there are differences, investigate: it could indicate that the system state changed between the dry-run and the actual execution, or that a control failed silently.
