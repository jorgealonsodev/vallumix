# Report System Overview

Vallumix generates structured reports after every `audit` or `apply` execution. Reports summarize the host information, compliance statistics, and the detailed status of each evaluated control. The report system is implemented in the `vallumix-reporters` crate and supports four output formats.

## Report Formats at a Glance

| Format | Use Case | Output | File Extension |
|--------|----------|--------|----------------|
| **HTML** | Auditors, management presentations, compliance evidence | Self-contained file with embedded CSS | `.html` |
| **JSON** | Dashboards, SIEM ingestion, programmatic analysis | Pretty-printed JSON object | `.json` |
| **JUnit** | CI/CD pipelines, Jenkins, GitLab CI, GitHub Actions | Standard JUnit XML | `.xml` |
| **Text** | Quick CLI feedback, terminal review, operator triage | Colored plain text with icons | `.txt` |

## Generating a Report

Use the `--report` flag to select the format and `--output` to specify the destination path (without extension):

```bash
# HTML report for an auditor
vallumix audit --profile web --report html --output /var/reports/vallumix/audit-2024-06-01

# JSON report for a dashboard
vallumix audit --profile web --report json --output /var/reports/vallumix/audit-2024-06-01

# JUnit report for CI/CD
vallumix audit --profile web --report junit --output /var/reports/vallumix/ci-results

# Text report for terminal review
vallumix audit --profile web --report text --output /var/reports/vallumix/audit-2024-06-01
```

If `--output` is omitted, Vallumix writes the report to a timestamped file in `/tmp`.

```tip
You can generate multiple reports from the same execution by running `audit` once and converting the JSON output, or by running `audit` with different `--report` values in separate invocations. Since Vallumix is idempotent, re-running `audit` does not change system state.
```

## Report Data Model

Every report contains the same underlying data, regardless of format:

- **`host.hostname`** — the server hostname.
- **`host.distro`** — the detected distribution (e.g., `debian/12`, `rocky/9`).
- **`summary.total`** — total number of controls evaluated.
- **`summary.pass`** — controls marked `Compliant`.
- **`summary.fail`** — controls marked `NonCompliant`.
- **`summary.skip`** — controls marked `Skipped` (e.g., dry-run or not applicable).
- **`summary.compliance_rate`** — percentage of compliant controls (`pass / total * 100`).
- **`controls[]`** — array of individual control results, each with `id`, `description`, `severity`, `status`, `evidence`, and optional `message`.

## Threshold Enforcement

Vallumix supports a compliance threshold. If the compliance rate falls below the threshold, the CLI exits with code `1`, which signals failure to CI/CD pipelines and orchestration tools:

```bash
vallumix audit --profile web --threshold 95
```

This is especially useful with the **JUnit** reporter: a pipeline stage can gate deployment on a 100% compliance result.

## Format-Specific Notes

- **HTML** reports are fully self-contained; no external CSS or JavaScript files are required. You can email a single `.html` file to an auditor or open it offline.
- **JSON** reports use snake-case field names and pretty-printing for human readability. They can be piped directly into `jq` for filtering.
- **JUnit** reports map `Compliant` to passing tests, `NonCompliant` to `<failure>`, and `Skipped` to `<skipped>`. The `classname` attribute is the control description, and the `name` is the CIS ID.
- **Text** reports automatically detect `NO_COLOR` and strip ANSI escape codes. In a pipeline, they fall back to plain text labels (`OK`, `FAIL`, `SKIP`).

For detailed examples of each format, see the dedicated pages in this section.
