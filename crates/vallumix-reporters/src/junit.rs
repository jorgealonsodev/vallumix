use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use vallumix_core::error::ReportError;
use vallumix_core::profile::Report;

#[derive(Debug, Clone, Default)]
pub struct JunitReporter;

impl JunitReporter {
    pub fn new() -> Self {
        JunitReporter
    }

    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }
}

impl vallumix_core::profile::Reporter for JunitReporter {
    fn generate(&self, report: &Report) -> Result<String, ReportError> {
        let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

        writer
            .write_event(Event::Decl(
                quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), None),
            ))
            .map_err(|e| ReportError::Serialize(e.to_string()))?;

        let mut testsuite = BytesStart::new("testsuite");
        testsuite.push_attribute(("name", "vallumix"));
        testsuite.push_attribute(("tests", report.summary.total.to_string().as_str()));
        testsuite.push_attribute(("failures", report.summary.fail.to_string().as_str()));
        testsuite.push_attribute(("skipped", report.summary.skip.to_string().as_str()));
        testsuite.push_attribute(("errors", "0"));
        writer
            .write_event(Event::Start(testsuite))
            .map_err(|e| ReportError::Serialize(e.to_string()))?;

        for control in &report.controls {
            let mut testcase = BytesStart::new("testcase");
            testcase.push_attribute(("name", control.id.as_str()));
            testcase.push_attribute(("classname", control.description.as_str()));
            writer
                .write_event(Event::Start(testcase))
                .map_err(|e| ReportError::Serialize(e.to_string()))?;

            match control.status.as_str() {
                "NonCompliant" => {
                    let mut failure = BytesStart::new("failure");
                    failure.push_attribute((
                        "message",
                        control.message.as_deref().unwrap_or("non-compliant"),
                    ));
                    writer
                        .write_event(Event::Start(failure))
                        .map_err(|e| ReportError::Serialize(e.to_string()))?;
                    writer
                        .write_event(Event::Text(BytesText::new(
                            &Self::escape_xml(&control.evidence),
                        )))
                        .map_err(|e| ReportError::Serialize(e.to_string()))?;
                    writer
                        .write_event(Event::End(BytesEnd::new("failure")))
                        .map_err(|e| ReportError::Serialize(e.to_string()))?;
                }
                "Skipped" => {
                    let mut skipped = BytesStart::new("skipped");
                    skipped.push_attribute((
                        "message",
                        control.message.as_deref().unwrap_or("skipped"),
                    ));
                    writer
                        .write_event(Event::Empty(skipped))
                        .map_err(|e| ReportError::Serialize(e.to_string()))?;
                }
                "Error" => {
                    let mut error = BytesStart::new("error");
                    error.push_attribute((
                        "message",
                        control.message.as_deref().unwrap_or("error"),
                    ));
                    writer
                        .write_event(Event::Start(error))
                        .map_err(|e| ReportError::Serialize(e.to_string()))?;
                    writer
                        .write_event(Event::Text(BytesText::new(
                            &Self::escape_xml(&control.evidence),
                        )))
                        .map_err(|e| ReportError::Serialize(e.to_string()))?;
                    writer
                        .write_event(Event::End(BytesEnd::new("error")))
                        .map_err(|e| ReportError::Serialize(e.to_string()))?;
                }
                _ => {
                    // Compliant or Warning → passing, no child element
                }
            }

            writer
                .write_event(Event::End(BytesEnd::new("testcase")))
                .map_err(|e| ReportError::Serialize(e.to_string()))?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("testsuite")))
            .map_err(|e| ReportError::Serialize(e.to_string()))?;

        String::from_utf8(writer.into_inner())
            .map_err(|e| ReportError::Serialize(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vallumix_core::profile::{ControlReport, HostInfo, Report, Reporter, Summary};

    fn mixed_report() -> Report {
        Report {
            host: HostInfo {
                hostname: "h".into(),
                distro: "d".into(),
            },
            summary: Summary {
                total: 4,
                pass: 1,
                fail: 1,
                skip: 1,
                compliance_rate: 25.0,
            },
            controls: vec![
                ControlReport {
                    id: "1".into(),
                    description: "A".into(),
                    severity: "Low".into(),
                    status: "Compliant".into(),
                    evidence: "ok".into(),
                    message: None,
                },
                ControlReport {
                    id: "2".into(),
                    description: "B".into(),
                    severity: "High".into(),
                    status: "NonCompliant".into(),
                    evidence: "bad".into(),
                    message: Some("fix me".into()),
                },
                ControlReport {
                    id: "3".into(),
                    description: "C".into(),
                    severity: "Low".into(),
                    status: "Skipped".into(),
                    evidence: "skipped".into(),
                    message: None,
                },
                ControlReport {
                    id: "4".into(),
                    description: "D & E <test>".into(),
                    severity: "Low".into(),
                    status: "Compliant".into(),
                    evidence: "ok".into(),
                    message: None,
                },
            ],
        }
    }

    #[test]
    fn junit_xml_well_formed() {
        let reporter = JunitReporter::new();
        let xml = reporter.generate(&mixed_report()).unwrap();
        assert!(xml.starts_with("<?xml"));
        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("</testsuite>"));
    }

    #[test]
    fn junit_testsuite_counts_match() {
        let reporter = JunitReporter::new();
        let xml = reporter.generate(&mixed_report()).unwrap();
        assert!(xml.contains(r#"tests="4""#));
        assert!(xml.contains(r#"failures="1""#));
        assert!(xml.contains(r#"skipped="1""#));
    }

    #[test]
    fn junit_non_compliant_has_failure() {
        let reporter = JunitReporter::new();
        let xml = reporter.generate(&mixed_report()).unwrap();
        assert!(xml.contains("<failure"));
        assert!(xml.contains("fix me"));
    }

    #[test]
    fn junit_skipped_has_skipped() {
        let reporter = JunitReporter::new();
        let xml = reporter.generate(&mixed_report()).unwrap();
        assert!(xml.contains("<skipped"));
    }

    #[test]
    fn junit_escapes_special_chars() {
        let reporter = JunitReporter::new();
        let xml = reporter.generate(&mixed_report()).unwrap();
        assert!(!xml.contains("D & E <test>")); // raw should be escaped
        assert!(xml.contains("D &amp; E &lt;test&gt;"));
    }

    #[test]
    fn junit_reporter_snapshot() {
        let reporter = JunitReporter::new();
        let output = reporter.generate(&mixed_report()).unwrap();
        insta::assert_snapshot!(output);
    }
}
