# JSON Report

The JSON reporter produces a machine-parseable, pretty-printed JSON document containing the complete audit or apply result. It is the preferred format for integration with dashboards, SIEM platforms, and custom automation scripts.

## When to Use JSON

- **Grafana / custom dashboards** — ingest the JSON into a time-series database or render it in a web panel.
- **SIEM integration** — forward the report to Splunk, Elastic Security, or Sentinel via a log shipper.
- **Automation scripts** — parse the JSON with `jq`, Python, or any language with a JSON library.
- **Diffing over time** — store daily JSON reports in Git or object storage and compare compliance trends.

## Generating a JSON Report

```bash
vallumix audit --profile web --report json --output /var/reports/vallumix/audit
```

Result: `/var/reports/vallumix/audit.json`.

## JSON Structure

```json
{
  "host": {
    "hostname": "web01",
    "distro": "debian/12"
  },
  "summary": {
    "total": 45,
    "pass": 38,
    "fail": 5,
    "skip": 2,
    "compliance_rate": 84.4
  },
  "controls": [
    {
      "id": "1.1.1.1",
      "description": "Disable cramfs",
      "severity": "Low",
      "status": "Compliant",
      "evidence": "not present",
      "message": null
    },
    {
      "id": "5.2.4",
      "description": "Disable root login",
      "severity": "High",
      "status": "NonCompliant",
      "evidence": "PermitRootLogin yes",
      "message": "should be no"
    },
    {
      "id": "3.1.1",
      "description": "Disable IP forwarding",
      "severity": "Medium",
      "status": "Skipped",
      "evidence": "dry-run",
      "message": null
    }
  ]
}
```

## Field Reference

| Field | Type | Description |
|-------|------|-------------|
| `host.hostname` | string | Server hostname at execution time. |
| `host.distro` | string | Detected distribution and version (e.g., `rocky/9`). |
| `summary.total` | integer | Total controls evaluated. |
| `summary.pass` | integer | Controls with status `Compliant`. |
| `summary.fail` | integer | Controls with status `NonCompliant`. |
| `summary.skip` | integer | Controls with status `Skipped`. |
| `summary.compliance_rate` | float | Percentage of pass / total. |
| `controls[].id` | string | CIS control identifier (e.g., `5.2.4`). |
| `controls[].description` | string | Human-readable control title. |
| `controls[].severity` | string | `Low`, `Medium`, or `High`. |
| `controls[].status` | string | `Compliant`, `NonCompliant`, or `Skipped`. |
| `controls[].evidence` | string | Current state or reason for the status. |
| `controls[].message` | string or null | Remediation hint or error detail. |

## Processing with jq

```bash
# Extract only failed high-severity controls
vallumix audit --profile web --report json | \
  jq '.controls[] | select(.status == "NonCompliant" and .severity == "High")'

# Calculate compliance rate from a saved report
jq '.summary.compliance_rate' /var/reports/vallumix/audit.json

# Count controls by severity
jq '[.controls[].severity] | group_by(.) | map({severity: .[0], count: length})' audit.json
```

## SIEM Ingestion Example

For Splunk or Fluent Bit, wrap the JSON report in a single-line envelope and forward it:

```bash
vallumix audit --profile web --report json --output /var/log/vallumix/last-audit
jq -c '.' /var/log/vallumix/last-audit.json >> /var/log/vallumix/audit-stream.log
```

Your log shipper can then parse each line as a JSON object and index fields like `host.hostname`, `summary.compliance_rate`, and `controls[].id`.

```tip
The JSON reporter uses `serde_json::to_string_pretty`, so the output is human-readable by default. For production pipelines, pipe through `jq -c` to minify and reduce transfer size.
```
