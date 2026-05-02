use std::env;

use owo_colors::OwoColorize;
use vallumix_core::error::ReportError;
use vallumix_core::profile::Report;

#[derive(Debug, Clone)]
pub struct TextReporter {
    no_color: bool,
}

impl TextReporter {
    pub fn new() -> Self {
        let no_color = env::var("NO_COLOR").is_ok_and(|v| !v.is_empty());
        TextReporter { no_color }
    }

    pub fn with_no_color(no_color: bool) -> Self {
        TextReporter { no_color }
    }

    fn styled_status(&self, status: &str) -> String {
        if self.no_color {
            return match status {
                "Compliant" => "OK".into(),
                "NonCompliant" => "FAIL".into(),
                _ => "SKIP".into(),
            };
        }
        match status {
            "Compliant" => "✓ Compliant".green().to_string(),
            "NonCompliant" => "✗ Non-Compliant".red().to_string(),
            "Skipped" => "⚠ Skipped".yellow().to_string(),
            _ => format!("⚠ {}", status).yellow().to_string(),
        }
    }

    fn styled_severity(&self, severity: &str) -> String {
        let label = match severity {
            "High" => "[HIGH]",
            "Medium" => "[MED]",
            _ => "[LOW]",
        };
        if self.no_color {
            label.to_string()
        } else {
            match severity {
                "High" => label.red().bold().to_string(),
                "Medium" => label.yellow().to_string(),
                _ => label.green().to_string(),
            }
        }
    }
}

impl Default for TextReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl vallumix_core::profile::Reporter for TextReporter {
    fn generate(&self, report: &Report) -> Result<String, ReportError> {
        let mut lines = Vec::new();
        lines.push(format!(
            "Vallumix Compliance Report — {} ({})\n",
            report.host.hostname, report.host.distro
        ));
        lines.push("━".repeat(60));
        lines.push(format!(
            "  Total: {:3}   Pass: {:3}   Fail: {:3}   Skip: {:3}",
            report.summary.total,
            report.summary.pass,
            report.summary.fail,
            report.summary.skip
        ));
        lines.push(format!(
            "  Compliance Rate: {:.1}%\n",
            report.summary.compliance_rate
        ));
        lines.push("━".repeat(60));

        for control in &report.controls {
            let sev = self.styled_severity(&control.severity);
            let status = self.styled_status(&control.status);
            lines.push(format!(
                "{} {} {} — {}",
                sev,
                status,
                control.id,
                control.description
            ));
            if !control.evidence.is_empty() {
                lines.push(format!("      → {}", control.evidence));
            }
        }

        lines.push("━".repeat(60));
        Ok(lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vallumix_core::profile::{ControlReport, HostInfo, Report, Reporter, Summary};

    fn sample_report() -> Report {
        Report {
            host: HostInfo {
                hostname: "srv01".into(),
                distro: "rocky/9".into(),
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
    fn text_reporter_includes_icons() {
        let reporter = TextReporter::with_no_color(true);
        let text = reporter.generate(&sample_report()).unwrap();
        assert!(text.contains("OK"));
        assert!(text.contains("FAIL"));
        assert!(text.contains("SKIP"));
    }

    #[test]
    fn text_reporter_plain_mode_no_color() {
        let reporter = TextReporter::with_no_color(true);
        let text = reporter.generate(&sample_report()).unwrap();
        assert!(text.contains("[HIGH]"));
        assert!(text.contains("[MED]"));
        assert!(text.contains("[LOW]"));
        // Plain mode should not contain ANSI escape codes
        assert!(!text.contains("\x1b["));
    }

    #[test]
    fn text_reporter_colored_mode_has_escapes() {
        let reporter = TextReporter::with_no_color(false);
        let text = reporter.generate(&sample_report()).unwrap();
        // Colored output should contain ANSI escape codes
        assert!(text.contains("\x1b["));
    }

    #[test]
    fn text_reporter_has_host_header() {
        let reporter = TextReporter::with_no_color(true);
        let text = reporter.generate(&sample_report()).unwrap();
        assert!(text.contains("srv01"));
        assert!(text.contains("rocky/9"));
    }

    #[test]
    fn text_reporter_has_summary_table() {
        let reporter = TextReporter::with_no_color(true);
        let text = reporter.generate(&sample_report()).unwrap();
        assert!(text.contains("Total:"));
        assert!(text.contains("Pass:"));
        assert!(text.contains("Compliance Rate:"));
    }

    #[test]
    fn text_reporter_snapshot() {
        let reporter = TextReporter::with_no_color(true);
        let output = reporter.generate(&sample_report()).unwrap();
        insta::assert_snapshot!(output);
    }
}
