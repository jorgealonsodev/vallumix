# reporter-text Specification

## Purpose

Implement a colored terminal text reporter using `owo-colors` that produces a human-readable compliance report with icon prefixes and a summary table, per PRD §5.5 and §6.5.

## Requirements

### Requirement: TextReporter Implementation

The system MUST provide a `TextReporter` struct implementing the `Reporter` trait via `generate(&self, report: &Report) -> Result<String, ReportError>`. It MUST use `owo-colors` for colored output and `✓`/`✗`/`⚠` icons.

#### Scenario: TextReporter generates formatted output

- GIVEN a `Report` with 3 controls (1 Compliant, 1 NonCompliant, 1 Skipped)
- WHEN `TextReporter::new(false).generate(&report)` is called
- THEN it returns `Ok(String)` containing lines with icon prefixes: `✓`, `✗`, `⚠`

#### Scenario: Output includes compliance summary table

- GIVEN a report with `summary { total: 3, pass: 1, fail: 1, skip: 1, compliance_rate: 33.33 }`
- WHEN the text report is generated
- THEN it contains a summary table with total, pass, fail, skip, and compliance rate

### Requirement: Colored Output with NO_COLOR Support

`TextReporter` MUST use `owo-colors` for conditional coloring. It MUST respect the `NO_COLOR` environment variable (per PRD §6.5) and the `--no-color` CLI flag. When colors are disabled, output MUST use plain ASCII: `OK`, `FAIL`, `SKIP` instead of colored icons.

#### Scenario: Colored output with NO_COLOR unset

- GIVEN `NO_COLOR` is not set and `no_color` is `false`
- WHEN `TextReporter::new(false).generate(&report)` is called
- THEN output contains ANSI color codes for Compliant (green), NonCompliant (red), Skipped (yellow)

#### Scenario: Plain output with NO_COLOR set

- GIVEN `NO_COLOR` environment variable is set
- WHEN `TextReporter::new(true).generate(&report)` is called
- THEN output contains `OK`, `FAIL`, `SKIP` text without ANSI codes

### Requirement: Per-Control Status with Icons

Each control in the text report MUST display with an icon prefix:
- Compliant → `✓` (green, or `OK` in plain mode)
- NonCompliant → `✗` (red, or `FAIL` in plain mode)
- Skipped → `⚠` (yellow, or `SKIP` in plain mode)
- Error → `✗` (red, or `ERROR` in plain mode)

#### Scenario: NonCompliant control displays with red icon

- GIVEN a `NonCompliant` control with ID "5.2.4" and description "Disable root login"
- WHEN the text report is generated
- THEN the output contains a line starting with `✗` (or `FAIL`) followed by "[5.2.4] Disable root login"

### Requirement: Host Info Header

The text report MUST include a header section with hostname, distribution, and timestamp before the control details.

#### Scenario: Header includes host information

- GIVEN a `Report` with `host { hostname: "web-01", distro: "debian/12" }`
- WHEN the text report is generated
- THEN the output starts with a header containing "web-01" and "debian/12"

### Requirement: Reporter Trait Integration

`TextReporter` MUST implement the existing `Reporter` trait from `vallumix-core::profile`. The constructor MUST accept a `no_color: bool` parameter to control ANSI output.

#### Scenario: TextReporter as Box<dyn Reporter>

- GIVEN `let reporter: Box<dyn Reporter> = Box::new(TextReporter::new(false));`
- WHEN `reporter.generate(&report)` is called
- THEN it compiles and returns colored text output

### Requirement: Control Severity Display

Each control line in the text report SHOULD display severity level alongside the icon and description: `[HIGH]`, `[MED]`, or `[LOW]`.

#### Scenario: High severity control displays [HIGH]

- GIVEN a control with `severity = "High"`
- WHEN the text report is generated
- THEN the control line contains `[HIGH]`

## Acceptance Criteria

- [ ] `TextReporter` implements `Reporter` trait
- [ ] Colored output with `owo-colors` (green/red/yellow)
- [ ] `NO_COLOR` and `--no-color` support switching to plain ASCII
- [ ] Compliance summary table with rate percentage
- [ ] Per-control icon prefixes: ✓/✗/⚠ (colored) or OK/FAIL/SKIP (plain)