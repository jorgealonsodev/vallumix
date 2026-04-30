use askama::Template;
use vallumix_core::error::ReportError;
use vallumix_core::profile::{ControlReport, HostInfo, Report, Summary};

use askama::Template as _;

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate {
    host: HostInfo,
    summary: Summary,
    controls: Vec<ControlReport>,
    timestamp: String,
}

#[derive(Debug, Clone)]
pub struct HtmlReporter {
    timestamp: Option<String>,
}

impl HtmlReporter {
    pub fn new() -> Self {
        HtmlReporter { timestamp: None }
    }

    pub fn with_timestamp(timestamp: impl Into<String>) -> Self {
        HtmlReporter {
            timestamp: Some(timestamp.into()),
        }
    }
}

impl Default for HtmlReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl vallumix_core::profile::Reporter for HtmlReporter {
    fn generate(&self, report: &Report) -> Result<String, ReportError> {
        let tmpl = ReportTemplate {
            host: report.host.clone(),
            summary: report.summary.clone(),
            controls: report.controls.clone(),
            timestamp: self
                .timestamp
                .clone()
                .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string()),
        };
        tmpl.render()
            .map_err(|e| ReportError::Serialize(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vallumix_core::profile::Reporter;

    fn sample_report() -> Report {
        Report {
            host: HostInfo {
                hostname: "web01".into(),
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
        }
    }

    #[test]
    fn html_reporter_includes_host() {
        let reporter = HtmlReporter::new();
        let html = reporter.generate(&sample_report()).unwrap();
        assert!(html.contains("web01"));
        assert!(html.contains("debian/12"));
    }

    #[test]
    fn html_reporter_includes_compliance_summary() {
        let reporter = HtmlReporter::new();
        let html = reporter.generate(&sample_report()).unwrap();
        assert!(html.contains("33.3"));
        assert!(html.contains("Compliant"));
        assert!(html.contains("Non-Compliant"));
    }

    #[test]
    fn html_reporter_has_self_contained_css() {
        let reporter = HtmlReporter::new();
        let html = reporter.generate(&sample_report()).unwrap();
        assert!(html.contains("<style>"));
        assert!(html.contains("</style>"));
    }

    #[test]
    fn html_reporter_contains_control_rows() {
        let reporter = HtmlReporter::new();
        let html = reporter.generate(&sample_report()).unwrap();
        assert!(html.contains("Disable cramfs"));
        assert!(html.contains("Disable root login"));
        assert!(html.contains("Disable IP forwarding"));
    }

    #[test]
    fn html_reporter_snapshot() {
        let reporter = HtmlReporter::with_timestamp("2024-01-15 12:00:00 +0000");
        let output = reporter.generate(&sample_report()).unwrap();
        insta::assert_snapshot!(output);
    }
}
