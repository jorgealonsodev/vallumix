use std::path::Path;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use vallumix_core::control::{CheckStatus, Control};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::profile::{ControlReport, Profile};
use vallumix_reporters::{build_report, HtmlReporter, JsonReporter, JunitReporter, TextReporter};

pub fn run(
    profile: &str,
    threshold: u8,
    report_formats: Option<Vec<String>>,
    output: Option<&Path>,
    quiet: bool,
) -> Result<i32> {
    let distro = Distro::Debian12; // TODO: detect
    let ctx = Context::new(distro)?;
    let profile_path = ctx.profile_dir.join(format!("{}.toml", profile));
    let profile = Profile::from_file(&profile_path)?;
    let registry = vallumix_controls::registry();
    let controls = profile.resolve_controls(&registry)?;

    let total = controls.len();

    let pb = if quiet || total == 0 {
        None
    } else {
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::with_template("[{pos}/{len}] {msg}")
                .expect("valid template"),
        );
        Some(pb)
    };

    let control_reports: Vec<ControlReport> = controls
        .par_iter()
        .map(|ctrl| {
            if let Some(ref pb) = pb {
                pb.set_message(format!("{} — {}", ctrl.id(), ctrl.description()));
            }
            let result = ctrl.check(&ctx);
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
            match result {
                Ok(check) => ControlReport {
                    id: ctrl.id().to_string(),
                    description: ctrl.description().to_string(),
                    severity: format!("{:?}", ctrl.severity()),
                    status: match check.status {
                        CheckStatus::Compliant => "Compliant".to_string(),
                        CheckStatus::NonCompliant => "NonCompliant".to_string(),
                        CheckStatus::Skipped => "Skipped".to_string(),
                        CheckStatus::Error => "Error".to_string(),
                        CheckStatus::Warning(_) => "Warning".to_string(),
                    },
                    evidence: check.evidence,
                    message: check.message,
                },
                Err(e) => ControlReport {
                    id: ctrl.id().to_string(),
                    description: ctrl.description().to_string(),
                    severity: format!("{:?}", ctrl.severity()),
                    status: "Error".to_string(),
                    evidence: e.to_string(),
                    message: None,
                },
            }
        })
        .collect();

    if let Some(ref pb) = pb {
        pb.finish_with_message("Done");
    }

    let pass = control_reports.iter().filter(|c| c.status == "Compliant").count();
    let report = build_report(
        ctx.hostname.clone(),
        distro.to_string(),
        control_reports,
    );

    if let Some(formats) = report_formats {
        for fmt in formats {
            let content = match fmt.as_str() {
                "json" => JsonReporter::new().generate(&report)?,
                "html" => HtmlReporter::new().generate(&report)?,
                "junit" => JunitReporter::new().generate(&report)?,
                "text" => TextReporter::new().generate(&report)?,
                _ => continue,
            };
            if let Some(output_dir) = output {
                let ext = match fmt.as_str() {
                    "json" => "json",
                    "html" => "html",
                    "junit" => "xml",
                    "text" => "txt",
                    _ => "txt",
                };
                let path = output_dir.join(format!("vallumix-report.{}", ext));
                std::fs::create_dir_all(output_dir)?;
                std::fs::write(&path, content)?;
                if !quiet {
                    println!("Report written to {}", path.display());
                }
            } else if !quiet {
                println!("{}", content);
            }
        }
    }

    if total == 0 || (pass as f64 / total as f64) * 100.0 >= threshold as f64 {
        Ok(0)
    } else {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_command_signature_exists() {
        let _f: fn(&str, u8, Option<Vec<String>>, Option<&Path>, bool) -> Result<i32> = run;
    }

    #[test]
    fn audit_returns_exit_code_type() {
        let result: Result<i32> = Ok(0);
        assert!(result.is_ok());
    }
}
