# Compliance Audit (UC-02)

This guide describes how to perform a security audit without modifying the target system, generating reports that serve as evidence for ISO 27001, PCI-DSS compliance audits, or internal security reviews.

## Scenario

- **Actor:** Security consultant with temporary access to a client server.
- **Restriction:** Cannot modify system configurations.
- **Objective:** Evaluate security posture and deliver an executive report.
- **Distribution:** Unknown until connection (could be Debian, Ubuntu, RHEL, or derivative).

## Step 1: Transfer Vallumix to the Target Server

Since Vallumix is a single static binary, you can copy it without installing dependencies:

```bash
# From your local machine
scp vallumix user@client-server:/tmp/
ssh user@client-server
sudo cp /tmp/vallumix /usr/local/bin/
```

```tip
If you do not have root access on the client's server, request that a local administrator run the binary for you. The `audit` subcommand requires root to read protected configuration files such as `/etc/shadow` or `/etc/ssh/sshd_config`.
```

## Step 2: Detect Distribution and Select Profile

```bash
sudo vallumix list
# Shows available profiles and associated controls
```

Based on the server's role, select the appropriate profile:

- Web server (Nginx, Apache) → `--profile web`
- Database server (PostgreSQL, MariaDB) → `--profile database`
- SSH bastion → `--profile bastion`

If you are unsure of the role, run all three profiles and compare the results.

## Step 3: Execute the Audit

```bash
sudo vallumix audit --profile database --report html,json --output /tmp/compliance-audit
```

The `audit` subcommand evaluates all controls in the profile but **never executes `apply`**. It only performs `pre_check` on each control and generates the report.

### Expected Output

```text
[  OK  ] 1.1.1.1  Disable cramfs support                 Compliant
[FAIL]  1.1.1.2  Disable freevxfs support               NonCompliant
[  OK  ] 2.1.1    Ensure autofs is not installed         Compliant
[FAIL]  3.1.1    Ensure IP forwarding is disabled       NonCompliant
[FAIL]  5.2.4    Ensure SSH root login is disabled      NonCompliant
[  OK  ] 5.2.5    Ensure SSH strict mode is enabled      Compliant
[FAIL]  6.1.1    Ensure permissions on /etc/passwd      NonCompliant

─────────────────────────────────────────────
Execution complete
Profile:        database
Controls run:   70
Compliant:      31
NonCompliant:   35
Skipped:        4
Compliance:     46.9%
Threshold:      not set
Status:         AUDIT ONLY (no changes applied)
Reports:        /tmp/compliance-audit.html
                /tmp/compliance-audit.json
─────────────────────────────────────────────
```

```note
The `AUDIT ONLY` status in the output confirms that no system file was modified. This is an important guarantee that you should highlight to the auditor or client.
```

## Step 4: Generate the HTML Report for the Client

The HTML report is self-contained: it includes embedded CSS, requires no internet connection, and can be opened directly in any browser. It contains:

- **Cover page:** hostname, distribution, kernel, date, and duration.
- **Executive summary:** compliance rate, comparison with previous executions.
- **Control detail:** CIS ID, description, severity, status, technical evidence.
- **Manual remediation recommendations** for controls that cannot be automated.
- **Standards mapping:** cross-references to NIST 800-53, ISO 27001 Annex A, and PCI-DSS.

## Step 5: Extract Metrics from JSON for Dashboards

If the client has a compliance dashboard (Grafana, Splunk, etc.), the JSON is the data source:

```bash
# Extract global compliance rate
jq '.summary.compliance_rate' /tmp/compliance-audit.json
# Output: 46.9

# List all non-compliant controls
jq '.controls[] | select(.status == "NonCompliant") | {id: .id, severity: .severity, description: .description}' /tmp/compliance-audit.json

# Count failed critical controls
jq '[.controls[] | select(.status == "NonCompliant" and .severity == "Critical")] | length' /tmp/compliance-audit.json
```

## Step 6: Deliver Results

The typical delivery package for an audit includes:

1. **HTML Report:** main document for executive review.
2. **JSON Report:** structured data for integration with client tools.
3. **Execution log:** output from `vallumix audit` with `--verbose` for technical traceability.
4. **Non-modification declaration:** text certifying that the audit was read-only.

```tip
Run the audit with `--verbose` to capture the exact commands Vallumix used to evaluate each control. This provides technical traceability to auditors who question the evaluation methodology.
```
