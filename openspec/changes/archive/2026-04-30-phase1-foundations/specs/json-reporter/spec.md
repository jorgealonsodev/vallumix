# json-reporter Specification

## Purpose

Implement the `Reporter` trait's `generate()` method for JSON output, producing a structured report containing host information, compliance summary, and per-control details, serialized via `serde_json`.

## Requirements

### Requirement: JSONReporter Struct

The `JSONReporter` struct MUST be defined in `vallumix-reporters` implementing the `Reporter` trait from `vallumix-core::profile`. It MUST take no configuration fields for Phase 1 — the output format is always JSON.

#### Scenario: JSONReporter implements Reporter trait

- GIVEN `JSONReporter` is defined in `vallumix-reporters`
- WHEN `let reporter: Box<dyn Reporter> = Box::new(JSONReporter)` is compiled
- THEN it compiles without object-safety errors

#### Scenario: JSONReporter generates valid JSON string

- GIVEN a `JSONReporter` instance and a non-empty list of control results
- WHEN `reporter.generate(&results, &ctx)` is called
- THEN it returns `Ok(String)` containing valid JSON parseable by `serde_json::from_str`

### Requirement: Report Structure — Host Info

The JSON report MUST contain a top-level `host` object with fields: `hostname` (String), `distro` (String), `kernel` (String), `timestamp` (ISO 8601 String), `duration_secs` (f64).

#### Scenario: Host info is populated from Context

- GIVEN `Context { hostname: "web01", distro: Distro::Debian12, ... }`
- WHEN the JSON report is generated
- THEN the `host` object contains `"hostname": "web01"` and `"distro": "debian12"`

### Requirement: Report Structure — Summary

The JSON report MUST contain a top-level `summary` object with fields: `total` (u32), `pass` (u32), `fail` (u32), `skip` (u32), `compliance_rate` (f64, percentage 0–100).

#### Scenario: Summary counts match results

- GIVEN 3 results with status `Compliant`, 1 with `NonCompliant`, and 1 with `Skipped`
- WHEN the JSON report is generated
- THEN `summary.total` is `5`, `summary.pass` is `3`, `summary.fail` is `1`, `summary.skip` is `1`, `compliance_rate` is `60.0`

### Requirement: Report Structure — Per-Control Detail

The JSON report MUST contain a top-level `controls` array where each element has: `id` (String), `description` (String), `severity` (String: "low"/"medium"/"high"), `status` (String: "compliant"/"non_compliant"/"error"/"skipped"), `evidence` (String), `message` (String or null).

#### Scenario: Control detail includes all required fields

- GIVEN a control result for CIS 5.2.4 with status `NonCompliant` and evidence `"PermitRootLogin yes"`
- WHEN the JSON report is generated
- THEN the `controls` array entry has `"id": "5.2.4"`, `"severity": "high"`, `"status": "non_compliant"`, `"evidence": "PermitRootLogin yes"`

### Requirement: JSON Output Formatting

The JSON output MUST be pretty-printed with 2-space indentation for human readability. The `serde_json::to_string_pretty` function MUST be used for serialization. All struct fields MUST use `snake_case` naming via `#[serde(rename_all = "snake_case")]`.

#### Scenario: Output is pretty-printed JSON

- GIVEN a report is generated
- WHEN the JSON string is inspected
- THEN it contains newlines and 2-space indentation (not a single-line minified string)

#### Scenario: Field names use snake_case

- GIVEN a report with `compliance_rate` and `backup_path` fields
- WHEN serialized to JSON
- THEN field names appear as `compliance_rate` and `backup_path` (not camelCase)