# HTML Report

The HTML reporter produces a self-contained, single-file report suitable for auditors, compliance officers, and management presentations. All CSS is embedded in the document; no external network requests or stylesheets are required.

## When to Use HTML

- **Compliance audits** — send a single `.html` file to an external auditor.
- **Management dashboards** — open in a browser for a visual summary of the server's security posture.
- **Offline review** — works without internet access because all styles are inline.
- **Email attachments** — a single file contains everything.

## Generating an HTML Report

```bash
vallumix audit --profile web --report html --output /var/reports/vallumix/audit
```

The resulting file is `/var/reports/vallumix/audit.html`.

## Structure of the Report

The HTML report includes the following sections:

1. **Header** — Vallumix logo, generation timestamp, hostname, and distribution.
2. **Summary Card** — donut or bar chart showing total, pass, fail, skip counts and the compliance rate percentage.
3. **Controls Table** — sortable table with columns for CIS ID, Description, Severity, Status, Evidence, and Message. Status is color-coded: green for Compliant, red for NonCompliant, yellow for Skipped.
4. **Footer** — Vallumix version, profile name, and a note about manual verification.

## Screenshot Description

When opened in a browser, the report presents a clean, professional layout:

- The top banner shows the hostname (`web01`) and distribution (`debian/12`) on the left, and the generation timestamp on the right.
- A large compliance rate figure (e.g., **87.3%**) is displayed prominently below the banner.
- Three summary boxes show **Pass: 38**, **Fail: 5**, **Skip: 2** with green, red, and amber backgrounds respectively.
- The controls table alternates row colors for readability. Each row has a status badge (`Compliant`, `Non-Compliant`, `Skipped`).
- Hovering over a severity badge (`High`, `Medium`, `Low`) shows a tooltip with the CIS severity definition.
- Rows with messages (e.g., `should be no`) display an expandable details arrow.

```tip
The HTML reporter uses the Askama templating engine. The template `report.html` is compiled into the binary, so the report generation requires no runtime template files.
```

## Example Output Snippet

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Vallumix Compliance Report — web01</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 2rem; }
    .summary { display: flex; gap: 1rem; }
    .badge-pass { background: #22c55e; color: #fff; padding: .25rem .5rem; border-radius: .25rem; }
    .badge-fail { background: #ef4444; color: #fff; padding: .25rem .5rem; border-radius: .25rem; }
  </style>
</head>
<body>
  <h1>Vallumix Compliance Report</h1>
  <p>Host: <strong>web01</strong> | Distro: <strong>debian/12</strong> | Generated: 2024-06-01 14:32:00 +0000</p>
  <div class="summary">
    <div>Total: 45</div>
    <div class="badge-pass">Pass: 38</div>
    <div class="badge-fail">Fail: 5</div>
    <div>Skip: 2</div>
  </div>
  <p>Compliance Rate: <strong>84.4%</strong></p>
  <!-- Controls table follows -->
</body>
</html>
```

## Customization

Because the template is compiled into the binary, customizing the HTML output requires modifying `crates/vallumix-reporters/templates/report.html` and recompiling Vallumix. Future versions may support user-provided templates via the `--output` directory.
