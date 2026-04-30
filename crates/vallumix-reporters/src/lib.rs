#![allow(missing_docs)]

pub mod html;
pub mod json;
pub mod junit;
pub mod text;

pub use html::HtmlReporter;
pub use json::JsonReporter;
pub use junit::JunitReporter;
pub use text::TextReporter;

use vallumix_core::profile::{ControlReport, HostInfo, Report, Summary};

pub fn build_report(
    hostname: String,
    distro: String,
    controls: Vec<ControlReport>,
) -> Report {
    let total = controls.len();
    let pass = controls.iter().filter(|c| c.status == "Compliant").count();
    let fail = controls.iter().filter(|c| c.status == "NonCompliant").count();
    let skip = controls.iter().filter(|c| c.status == "Skipped").count();
    let compliance_rate = if total > 0 {
        (pass as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    Report {
        host: HostInfo { hostname, distro },
        summary: Summary {
            total,
            pass,
            fail,
            skip,
            compliance_rate,
        },
        controls,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_report_compliance_rate() {
        let controls = vec![
            ControlReport {
                id: "1".into(),
                description: "d".into(),
                severity: "Low".into(),
                status: "Compliant".into(),
                evidence: "e".into(),
                message: None,
            },
            ControlReport {
                id: "2".into(),
                description: "d".into(),
                severity: "Low".into(),
                status: "NonCompliant".into(),
                evidence: "e".into(),
                message: None,
            },
            ControlReport {
                id: "3".into(),
                description: "d".into(),
                severity: "Low".into(),
                status: "Skipped".into(),
                evidence: "e".into(),
                message: None,
            },
        ];
        let report = build_report("host".into(), "distro".into(), controls);
        assert_eq!(report.summary.total, 3);
        assert_eq!(report.summary.pass, 1);
        assert_eq!(report.summary.fail, 1);
        assert_eq!(report.summary.skip, 1);
        assert!((report.summary.compliance_rate - 33.33333333333333).abs() < f64::EPSILON);
    }

    #[test]
    fn all_reporters_are_exported() {
        let _ = JsonReporter::new();
        let _ = HtmlReporter::new();
        let _ = JunitReporter::new();
        let _ = TextReporter::new();
    }
}
