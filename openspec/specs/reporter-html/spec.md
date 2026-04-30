# reporter-html Specification

## Purpose

Implement an HTML reporter using `askama` templates that produces a self-contained, visually formatted compliance report with embedded CSS, per the PRD §5.5 requirement for HTML output.

## Requirements

### Requirement: HtmlReporter Implementation

The system MUST provide an `HtmlReporter` struct implementing the `Reporter` trait via `generate(&self, report: &Report) -> Result<String, ReportError>`. It MUST use `askama` compile-time templates for rendering.

#### Scenario: HtmlReporter generates valid HTML

- GIVEN a `Report` with 2 controls (1 Compliant, 1 NonCompliant)
- WHEN `HtmlReporter::new().generate(&report)` is called
- THEN it returns `Ok(String)` containing well-formed HTML with `<html>`, `<head>`, and `<body>` tags

#### Scenario: HtmlReporter contains embedded CSS

- GIVEN an HTML report is generated
- WHEN the output is inspected
- THEN it MUST contain a `<style>` block with embedded CSS — no external stylesheet references

### Requirement: HTML Report Compliance Summary

The HTML report MUST contain a compliance summary section showing total controls, pass count, fail count, skip count, and compliance rate percentage.

#### Scenario: Summary section present in output

- GIVEN a `Report` with `summary.compliance_rate = 75.0`
- WHEN the HTML is generated
- THEN it contains a summary section displaying "75.0%" and pass/fail/skip counts

### Requirement: HTML Report Per-Control Detail

The HTML report MUST contain a per-control detail section listing each control's ID, description, severity, status, and evidence. Controls MUST be grouped by severity (High, Medium, Low).

#### Scenario: Per-control detail for NonCompliant control

- GIVEN a `Report` with a `NonCompliant` control ID "5.2.4"
- WHEN the HTML is generated
- THEN it contains a row with ID "5.2.4", status "NonCompliant", and the evidence text

#### Scenario: Controls grouped by severity

- GIVEN a report with High and Medium severity controls
- WHEN the HTML is generated
- THEN High severity controls appear before Medium severity controls

### Requirement: Reporter Trait Integration

`HtmlReporter` MUST implement the existing `Reporter` trait from `vallumix-core::profile`, using the `Report`, `HostInfo`, `Summary`, and `ControlReport` structs. No new trait methods are needed.

#### Scenario: HtmlReporter as Box<dyn Reporter>

- GIVEN `let reporter: Box<dyn Reporter> = Box::new(HtmlReporter::new());`
- WHEN `reporter.generate(&report)` is called
- THEN it compiles and returns `Ok(String)` containing HTML

### Requirement: Askama Template Location

The askama template MUST be located at `crates/vallumix-reporters/templates/report.html` following the PRD §7.2 workspace structure. The template MUST use askama derive macros on a `ReportTemplate` struct.

#### Scenario: Template renders with Report data

- GIVEN `ReportTemplate` struct populated with `Report` data
- WHEN `.render()` is called
- THEN it produces complete HTML matching the `HtmlReporter::generate()` output

## Acceptance Criteria

- [ ] `HtmlReporter` implements `Reporter` trait
- [ ] Self-contained HTML with embedded CSS (no external dependencies)
- [ ] Compliance summary section with rate percentage
- [ ] Per-control detail section grouped by severity
- [ ] Askama template at `templates/report.html`