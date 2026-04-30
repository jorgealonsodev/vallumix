# reporter-junit Specification

## Purpose

Implement a JUnit XML reporter using `quick-xml` that produces CI-consumable test result XML per the PRD §5.5 requirement for JUnit format, compatible with Jenkins, GitLab CI, and GitHub Actions.

## Requirements

### Requirement: JunitReporter Implementation

The system MUST provide a `JunitReporter` struct implementing the `Reporter` trait via `generate(&self, report: &Report) -> Result<String, ReportError>`. It MUST use `quick-xml` for XML generation.

#### Scenario: JunitReporter generates valid JUnit XML

- GIVEN a `Report` with 2 controls
- WHEN `JunitReporter::new().generate(&report)` is called
- THEN it returns `Ok(String)` containing valid XML with `<testsuites>` and `<testsuite>` root elements

#### Scenario: XML is well-formed and parseable

- GIVEN JUnit XML output
- WHEN parsed by an XML parser
- THEN it contains no syntax errors and follows the JUnit XML schema

### Requirement: JUnit Testcase Per Control

Each control in the report MUST map to a `<testcase>` element with `classname` set to the control's CIS ID category prefix and `name` set to the control's description. Control status MUST map as follows:

| Control Status | JUnit Element |
|---|---|
| Compliant | `<testcase>` (no child element) |
| NonCompliant | `<testcase><failure>` |
| Skipped | `<testcase><skipped>` |
| Error | `<testcase><error>` |

#### Scenario: Compliant control produces passing testcase

- GIVEN a control with `status = "Compliant"`
- WHEN JUnit XML is generated
- THEN the testcase element has no `<failure>`, `<skipped>`, or `<error>` child

#### Scenario: NonCompliant control produces failure

- GIVEN a control with `status = "NonCompliant"` and `evidence = "PermitRootLogin yes"`
- WHEN JUnit XML is generated
- THEN the testcase contains `<failure message="...">PermitRootLogin yes</failure>`

#### Scenario: Skipped control produces skipped element

- GIVEN a control with `status = "Skipped"`
- WHEN JUnit XML is generated
- THEN the testcase contains `<skipped/>`

### Requirement: JUnit Testsuite Attributes

The `<testsuite>` element MUST include `tests` (total), `failures` (NonCompliant count), `skipped` (Skipped count), and `errors` (Error count) attributes matching the report summary.

#### Scenario: Testsuite attributes match summary counts

- GIVEN a `Report` with `summary { total: 5, fail: 2, skip: 1 }`
- WHEN JUnit XML is generated
- THEN `<testsuite tests="5" failures="2" skipped="1" errors="0">`

### Requirement: Reporter Trait Integration

`JunitReporter` MUST implement the existing `Reporter` trait from `vallumix-core::profile`, using `Report`, `Summary`, and `ControlReport` structs. No new trait methods are needed.

#### Scenario: JunitReporter as Box<dyn Reporter>

- GIVEN `let reporter: Box<dyn Reporter> = Box::new(JunitReporter::new());`
- WHEN `reporter.generate(&report)` is called
- THEN it compiles and returns `Ok(String)` containing JUnit XML

### Requirement: XML Escaping

The JUnit XML output MUST properly escape special characters (`&`, `<`, `>`, `"`, `'`) in evidence and message fields to produce valid XML.

#### Scenario: Evidence with special characters

- GIVEN a control with `evidence = "PermitRootLogin yes & no"`
- WHEN JUnit XML is generated
- THEN the `&` is escaped as `&amp;` in the output

## Acceptance Criteria

- [ ] `JunitReporter` implements `Reporter` trait
- [ ] Valid JUnit XML compatible with Jenkins/GitLab/GitHub Actions
- [ ] Control status correctly mapped to JUnit testcase elements
- [ ] Testsuite attributes match summary counts
- [ ] Special characters properly XML-escaped