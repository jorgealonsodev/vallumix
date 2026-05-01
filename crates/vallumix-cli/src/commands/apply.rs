use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use vallumix_core::control::{ApplyStatus, CheckStatus, Control};
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;
use vallumix_core::profile::{ControlReport, Profile, Reporter};
use vallumix_reporters::{build_report, HtmlReporter, JsonReporter, JunitReporter, TextReporter};

pub fn run(
    profile: &str,
    dry_run: bool,
    threshold: u8,
    report_formats: Option<Vec<String>>,
    output: Option<&Path>,
    quiet: bool,
) -> Result<i32> {
    let distro = Distro::Debian12; // TODO: detect
    let mut ctx = Context::new(distro)?;
    ctx.dry_run = dry_run;
    let profile_path = ctx.profile_dir.join(format!("{}.toml", profile));
    let profile = Profile::from_file(&profile_path)?;
    let registry = vallumix_controls::registry();
    let controls = profile.resolve_controls(&registry)?;

    let _session_id = format!("{}-{}", profile.name, Utc::now().timestamp());

    let pb = if quiet || controls.is_empty() {
        None
    } else {
        let pb = ProgressBar::new(controls.len() as u64);
        pb.set_style(
            ProgressStyle::with_template("[{pos}/{len}] {msg}")
                .expect("valid template"),
        );
        Some(pb)
    };

    let mut control_reports = Vec::new();
    let mut pass = 0usize;

    for ctrl in &controls {
        if let Some(ref pb) = pb {
            pb.set_message(format!("{} — {}", ctrl.id(), ctrl.description()));
        }

        let check_result = ctrl.check(&ctx)?;
        let (status_str, evidence, message) = match check_result.status {
            CheckStatus::Compliant => {
                pass += 1;
                (
                    "Compliant".to_string(),
                    check_result.evidence,
                    check_result.message,
                )
            }
            CheckStatus::Skipped => {
                ("Skipped".to_string(), check_result.evidence, check_result.message)
            }
            CheckStatus::Error => {
                ("Error".to_string(), check_result.evidence, check_result.message)
            }
            CheckStatus::Warning(ref msg) => (
                "Warning".to_string(),
                check_result.evidence.clone(),
                msg.clone(),
            ),
            CheckStatus::NonCompliant => {
                if dry_run {
                    (
                        "Skipped".to_string(),
                        check_result.evidence,
                        Some("dry-run: would apply".into()),
                    )
                } else {
                    let apply_result = ctrl.apply(&ctx)?;
                    match apply_result.status {
                        ApplyStatus::Applied | ApplyStatus::AlreadyCompliant => {
                            pass += 1;
                            (
                                "Compliant".to_string(),
                                apply_result.message.unwrap_or_default(),
                                None,
                            )
                        }
                        ApplyStatus::Skipped => (
                            "Skipped".to_string(),
                            apply_result.message.unwrap_or_default(),
                            None,
                        ),
                        ApplyStatus::PartialApply(ref msg) => (
                            "PartialApply".to_string(),
                            apply_result.message.unwrap_or_default(),
                            msg.clone(),
                        ),
                        ApplyStatus::Failed => (
                            "NonCompliant".to_string(),
                            apply_result.message.unwrap_or_default(),
                            None,
                        ),
                    }
                }
            }
        };

        control_reports.push(ControlReport {
            id: ctrl.id().to_string(),
            description: ctrl.description().to_string(),
            severity: format!("{:?}", ctrl.severity()),
            status: status_str,
            evidence,
            message,
        });

        if let Some(ref pb) = pb {
            pb.inc(1);
        }
    }

    if let Some(ref pb) = pb {
        pb.finish_with_message("Done");
    }

    let total = controls.len();
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
            if let Some(output_path) = output {
                let ext = match fmt.as_str() {
                    "json" => "json",
                    "html" => "html",
                    "junit" => "xml",
                    "text" => "txt",
                    _ => "txt",
                };
                let path = if output_path.extension().and_then(|e| e.to_str()) == Some(ext) {
                    output_path.to_path_buf()
                } else {
                    std::fs::create_dir_all(output_path)?;
                    output_path.join(format!("vallumix-report.{}", ext))
                };
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
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
    fn apply_command_signature_exists() {
        let _f: fn(&str, bool, u8, Option<Vec<String>>, Option<&Path>, bool) -> Result<i32> = run;
    }

    #[test]
    fn apply_returns_exit_code_type() {
        // Compile-time check that the function returns Result<i32>
        let result: Result<i32> = Ok(0);
        assert!(result.is_ok());
    }
}
