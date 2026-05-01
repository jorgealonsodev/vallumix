# Web Server Hardening (UC-01)

This guide describes the complete hardening of a newly provisioned Ubuntu 24.04 LTS server that will host a web application served by Nginx. The goal is to apply the controls of the `web` profile without interrupting the HTTP service.

## Scenario

- **Server:** Ubuntu 24.04 LTS (Noble), 2 vCPU, 4 GB RAM.
- **Service:** Nginx 1.24 serving a PHP-FPM application on port 443.
- **Initial state:** Minimal server installation, no previous hardening.
- **Objective:** Apply CIS controls from the web profile, generate HTML report for audit.

## Step 1: Verify the Environment

```bash
# Check distribution and version
cat /etc/os-release | grep PRETTY_NAME
# Expected output: PRETTY_NAME="Ubuntu 24.04 LTS"

# Confirm Nginx is active
systemctl is-active nginx
# Expected output: active

# Verify port 443 is listening
ss -tlnp | grep :443
# Expected output: LISTEN 0 4096 *:443
```

## Step 2: Install Vallumix

```bash
wget https://github.com/jorgealonsodev/vallumix/releases/download/v1.0.0/vallumix_1.0.0_amd64.deb
sudo dpkg -i vallumix_1.0.0_amd64.deb
vallumix --version
# Expected output: vallumix 1.0.0
```

## Step 3: Initial Audit (Optional but Recommended)

Before modifying anything, evaluate the current posture to establish a baseline:

```bash
sudo vallumix audit --profile web --report html,json --output /tmp/pre-audit
```

This generates `/tmp/pre-audit.html` and `/tmp/pre-audit.json`. On a new server, the compliance rate is typically between 30% and 50%.

## Step 4: Execute Hardening

```bash
sudo vallumix apply --profile web --report html --output /tmp/hardening-report
```

### Expected Output (Summary)

```text
[  OK  ] 1.1.1.1  Disable cramfs support                 Compliant
[ FIX  ] 1.1.1.2  Disable freevxfs support               Remediated
[ FIX  ] 1.1.1.3  Disable jffs2 support                  Remediated
[  OK  ] 2.1.1    Ensure autofs is not installed         Compliant
[ FIX  ] 3.1.1    Ensure IP forwarding is disabled       Remediated
[  OK  ] 3.2.1    Ensure packet redirect sending is dis… Compliant
[ FIX  ] 3.4.1    Ensure firewalld is installed          Remediated
[ FIX  ] 5.2.4    Ensure SSH root login is disabled      Remediated
[  OK  ] 5.2.5    Ensure SSH strict mode is enabled      Compliant
[ FIX  ] 6.1.1    Ensure permissions on /etc/passwd      Remediated

─────────────────────────────────────────────
Execution complete
Profile:        web
Controls run:   70
Compliant:      28
Remediated:     38
Failed:         2
Skipped:        2
Compliance:     94.3%
Threshold:      90.0%
Status:         PASS
Report:         /tmp/hardening-report.html
─────────────────────────────────────────────
```

```warning
The web profile does not disable Nginx or close ports 80/443. However, some firewall controls (`3.4.x`) configure default rules. If your application requires additional ports (for example, 8080 for an internal API), review them after execution.
```

## Step 5: Verify Nginx Is Still Working

```bash
curl -I https://localhost/
# Must return HTTP/2 200 with your application's headers

systemctl status nginx
# Must show active (running)
```

## Step 6: Idempotent Re-execution

To confirm that Vallumix is idempotent, run it again:

```bash
sudo vallumix apply --profile web --report json --output /tmp/post-audit
```

In this second execution, most controls should appear as `Compliant` or `SkippedAlreadyCompliant`, with very few or no `Remediated` entries.

## Step 7: Document for Audit

Keep these three artifacts:

1. `/tmp/pre-audit.html` — initial baseline.
2. `/tmp/hardening-report.html` — evidence of applied hardening.
3. `/tmp/post-audit.json` — confirmation of idempotency.

```tip
Schedule a monthly re-run of `vallumix audit --profile web --report html` as a cron task. This detects configuration deviations caused by package updates or manual changes by other administrators.
```
