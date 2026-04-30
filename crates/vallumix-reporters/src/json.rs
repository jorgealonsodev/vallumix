use vallumix_core::error::ReportError;
use vallumix_core::profile::Report;

#[derive(Debug, Clone, Default)]
pub struct JsonReporter;

impl JsonReporter {
    pub fn new() -> Self {
        JsonReporter
    }
}

impl vallumix_core::profile::Reporter for JsonReporter {
    fn generate(&self, report: &Report) -> Result<String, ReportError> {
        serde_json::to_string_pretty(report)
            .map_err(|e| ReportError::Serialize(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vallumix_core::profile::{ControlReport, HostInfo, Report, Reporter, Summary};

    #[test]
    fn json_reporter_produces_valid_json() {
        let reporter = JsonReporter::new();
        let report = Report {
            host: HostInfo {
                hostname: "testhost".into(),
                distro: "debian/12".into(),
            },
            summary: Summary {
                total: 2,
                pass: 1,
                fail: 1,
                skip: 0,
                compliance_rate: 50.0,
            },
            controls: vec![
                ControlReport {
                    id: "1.1.1.1".into(),
                    description: "Disable cramfs".into(),
                    severity: "Low".into(),
                    status: "Compliant".into(),
                    evidence: "not present".into(),
                    message: None,
                },
                ControlReport {
                    id: "5.2.4".into(),
                    description: "Disable root login".into(),
                    severity: "High".into(),
                    status: "NonCompliant".into(),
                    evidence: "PermitRootLogin yes".into(),
                    message: Some("should be no".into()),
                },
            ],
        };
        let json = reporter.generate(&report).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["host"]["hostname"], "testhost");
        assert_eq!(parsed["summary"]["total"], 2);
        assert_eq!(parsed["controls"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn json_reporter_snake_case_fields() {
        let reporter = JsonReporter::new();
        let report = Report {
            host: HostInfo {
                hostname: "h".into(),
                distro: "d".into(),
            },
            summary: Summary {
                total: 0,
                pass: 0,
                fail: 0,
                skip: 0,
                compliance_rate: 0.0,
            },
            controls: vec![],
        };
        let json = reporter.generate(&report).unwrap();
        assert!(json.contains("\"host\""));
        assert!(json.contains("\"hostname\""));
        assert!(json.contains("\"compliance_rate\""));
    }

    #[test]
    fn json_reporter_snapshot() {
        let reporter = JsonReporter::new();
        let report = Report {
            host: HostInfo {
                hostname: "snaphost".into(),
                distro: "debian/12".into(),
            },
            summary: Summary {
                total: 3,
                pass: 1,
                fail: 1,
                skip: 1,
                compliance_rate: 33.33,
            },
            controls: vec![
                ControlReport {
                    id: "1.1.1.1".into(),
                    description: "Disable cramfs".into(),
                    severity: "Low".into(),
                    status: "Compliant".into(),
                    evidence: "not present".into(),
                    message: None,
                },
                ControlReport {
                    id: "5.2.4".into(),
                    description: "Disable root login".into(),
                    severity: "High".into(),
                    status: "NonCompliant".into(),
                    evidence: "PermitRootLogin yes".into(),
                    message: Some("should be no".into()),
                },
                ControlReport {
                    id: "3.1.1".into(),
                    description: "Disable IP forwarding".into(),
                    severity: "Medium".into(),
                    status: "Skipped".into(),
                    evidence: "dry-run".into(),
                    message: None,
                },
            ],
        };
        let output = reporter.generate(&report).unwrap();
        insta::assert_snapshot!(output);
    }
}
