# JUnit XML Report

The JUnit reporter produces standard JUnit XML that integrates natively with Jenkins, GitLab CI, GitHub Actions, Azure DevOps, and any other CI/CD platform that understands the JUnit test-result format.

## When to Use JUnit

- **CI/CD gating** — fail a build or deployment if compliance drops below 100%.
- **Jenkins dashboards** — visualize compliance trends alongside unit-test results.
- **GitLab Merge Request widgets** — display control failures directly in the MR.
- **GitHub Actions annotations** — surface failed controls as check-run annotations.

## Generating a JUnit Report

```bash
vallumix audit --profile web --report junit --output /var/reports/vallumix/ci-results
```

Result: `/var/reports/vallumix/ci-results.xml`.

## XML Structure

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="vallumix" tests="4" failures="1" skipped="1" errors="0">
  <testcase name="1.1.1.1" classname="Disable cramfs">
    <!-- Compliant: no child element -->
  </testcase>
  <testcase name="5.2.4" classname="Disable root login">
    <failure message="should be no">PermitRootLogin yes</failure>
  </testcase>
  <testcase name="3.1.1" classname="Disable IP forwarding">
    <skipped message="skipped"/>
  </testcase>
  <testcase name="4" classname="D &amp; E &lt;test&gt;">
    <!-- Compliant: special chars escaped -->
  </testcase>
</testsuite>
```

## Field Mapping

| JUnit Attribute | Vallumix Source | Notes |
|-----------------|-----------------|-------|
| `testsuite.name` | `"vallumix"` | Fixed identifier. |
| `testsuite.tests` | `summary.total` | Total controls evaluated. |
| `testsuite.failures` | `summary.fail` | Non-compliant controls. |
| `testsuite.skipped` | `summary.skip` | Skipped controls. |
| `testsuite.errors` | `0` | Errors are reported as failures. |
| `testcase.name` | `control.id` | CIS identifier. |
| `testcase.classname` | `control.description` | Human-readable title. |
| `failure.message` | `control.message` | Remediation hint. |
| `failure` text | `control.evidence` | Current state. |

## CI/CD Integration Examples

### GitLab CI

```yaml
compliance:
  stage: test
  script:
    - vallumix audit --profile web --report junit --output compliance
  artifacts:
    reports:
      junit: compliance.xml
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
```

GitLab will display failed controls in the MR widget and mark the pipeline as failed if any control is non-compliant.

### GitHub Actions

```yaml
- name: Run compliance audit
  run: vallumix audit --profile web --report junit --output compliance

- name: Publish JUnit results
  uses: mikepenz/action-junit-report@v4
  if: always()
  with:
    report_paths: 'compliance.xml'
```

### Jenkins

Use the JUnit plugin to publish the XML:

```groovy
post {
    always {
        junit 'compliance.xml'
    }
}
```

## Threshold Enforcement

Combine `--threshold` with the JUnit report to make the CLI exit non-zero when compliance is insufficient. CI/CD platforms treat exit code `1` as a pipeline failure:

```bash
vallumix audit --profile web --report junit --output ci-results --threshold 100
```

```tip
If you want to track compliance trends over time, archive the JUnit XML files in your CI system. Jenkins and GitLab both support historical test-result graphs that will show your compliance rate across builds.
```
