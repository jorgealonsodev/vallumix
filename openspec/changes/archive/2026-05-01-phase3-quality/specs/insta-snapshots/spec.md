# insta-snapshots Specification

## Purpose

Generate and verify `insta` snapshot files (.snap) for all four reporters (JSON, HTML, JUnit, Text), ensuring output structure stability across changes.

## Requirements

### Requirement: Snapshot Generation for Reporters

Each reporter (JSONReporter, HtmlReporter, JunitReporter, TextReporter) MUST have an `insta` snapshot test that captures its complete output against a `.snap` reference file. Snapshot files MUST be committed to the repository.

#### Scenario: JSON reporter snapshot is generated

- GIVEN a `Report` with 2 controls (1 Compliant, 1 NonCompliant)
- WHEN `json_reporter_snapshot` test runs
- THEN `insta::assert_snapshot!` captures the JSON output and a `.snap` file exists at `crates/vallumix-reporters/snapshots/`

#### Scenario: HTML reporter snapshot is generated

- GIVEN a `Report` with 2 controls (1 Compliant, 1 Skipped)
- WHEN `html_reporter_snapshot` test runs
- THEN `insta::assert_snapshot!` captures the HTML output and a `.snap` file exists

#### Scenario: JUnit reporter snapshot is generated

- GIVEN a `Report` with 2 controls (1 Compliant, 1 NonCompliant)
- WHEN `junit_reporter_snapshot` test runs
- THEN `insta::assert_snapshot!` captures the JUnit XML output and a `.snap` file exists

#### Scenario: Text reporter snapshot is generated

- GIVEN a `Report` with 2 controls (1 Compliant, 1 Error)
- WHEN `text_reporter_snapshot` test runs
- THEN `insta::assert_snapshot!` captures the text output and a `.snap` file exists

### Requirement: Snapshot Content Verification

Snapshot outputs MUST contain the required structural elements from each reporter spec: `host` object (JSON), `<style>` block (HTML), `<testsuites>` root (JUnit), and icon prefixes (Text).

#### Scenario: JSON snapshot contains host and summary

- GIVEN the JSON reporter snapshot file
- WHEN its content is parsed
- THEN it contains `host` (with hostname, distro, kernel, timestamp) and `summary` (with total, pass, fail, skip, compliance_rate)

#### Scenario: HTML snapshot contains embedded CSS

- GIVEN the HTML reporter snapshot file
- WHEN its content is inspected
- THEN it contains a `<style>` block with embedded CSS

#### Scenario: JUnit snapshot contains testsuites and testcases

- GIVEN the JUnit reporter snapshot file
- WHEN its content is parsed
- THEN it contains `<testsuites>`, `<testsuite>` with count attributes, and `<testcase>` elements

#### Scenario: Text snapshot contains icons and summary

- GIVEN the text reporter snapshot file
- WHEN its content is inspected
- THEN it contains icon prefixes (`✓`, `✗`, `⚠`) and a compliance summary table

### Requirement: Snapshot Stability

Reporters MUST produce deterministic output across runs. Timestamps, durations, and host-specific values MUST be replaced with fixed test values in snapshot fixtures to prevent false snapshot failures.

#### Scenario: Snapshot does not change across runs

- GIVEN a snapshot test suite with fixed `Context` (hostname="test-host", distro=Debian12, kernel="5.10.0", fixed timestamp)
- WHEN `cargo insta test` runs twice
- THEN no `.snap.new` files are produced — output is deterministic

### Requirement: CI Snapshot Enforcement

CI MUST fail if snapshots are missing or outdated. The command `cargo insta test --require-snapshots` MUST be a CI step.

#### Scenario: CI rejects missing snapshots

- GIVEN a new snapshot test is added but `.snap` file is not committed
- WHEN CI runs `cargo insta test --require-snapshots`
- THEN the step exits with non-zero code

#### Scenario: CI detects output drift

- GIVEN a reporter output format changes
- WHEN CI runs `cargo insta test`
-THEN the step exits with non-zero code indicating snapshot mismatch