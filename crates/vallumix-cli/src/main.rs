#![allow(missing_docs)]

mod cli;
mod commands;

use cli::{Cli, Commands, ReportFormat};
use clap::Parser;
use owo_colors::OwoColorize;

fn init_tracing(verbose: bool, quiet: bool) {
    let level = if quiet {
        tracing::Level::WARN
    } else if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
}

fn check_privileges() -> bool {
    nix::unistd::geteuid().is_root()
}

#[allow(dead_code)]
fn compute_exit_code(compliance_rate: f64, threshold: u8) -> i32 {
    if compliance_rate >= threshold as f64 {
        0
    } else {
        1
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        owo_colors::set_override(false);
    }

    init_tracing(cli.verbose, cli.quiet);

    let report_formats: Option<Vec<String>> = cli.report.as_ref().map(|s| {
        s.split(',').map(|f| f.trim().to_lowercase()).collect()
    });

    let exit_code = match &cli.command {
        Commands::Apply => {
            if !check_privileges() {
                eprintln!("{} {}", "Error:".red().bold(), "apply requires root privileges");
                std::process::exit(3);
            }
            match commands::apply::run(
                &cli.profile,
                cli.dry_run,
                cli.threshold,
                report_formats.clone(),
                cli.output.as_ref(),
                cli.quiet,
            ) {
                Ok(code) => code,
                Err(_) => 2,
            }
        }
        Commands::Audit => match commands::audit::run(
            &cli.profile,
            cli.threshold,
            report_formats.clone(),
            cli.output.as_ref(),
            cli.quiet,
        ) {
            Ok(code) => code,
            Err(_) => 2,
        },
        Commands::Rollback { control_id, session } => match commands::rollback::run(control_id.clone(), session.clone()) {
            Ok(code) => code,
            Err(_) => 2,
        },
        Commands::List => match commands::list::run() {
            Ok(code) => code,
            Err(_) => 2,
        },
        Commands::Completion { shell } => match commands::completion::run(*shell) {
            Ok(code) => code,
            Err(_) => 2,
        },
    };

    std::process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_parses_defaults() {
        let cli = Cli::parse_from(["vallumix", "audit"]);
        assert_eq!(cli.profile, "web");
        assert!(!cli.dry_run);
        assert_eq!(cli.threshold, 80);
        assert!(!cli.verbose);
        assert!(!cli.quiet);
    }

    #[test]
    fn cli_profile_flag() {
        let cli = Cli::parse_from(["vallumix", "--profile", "database", "audit"]);
        assert_eq!(cli.profile, "database");
    }

    #[test]
    fn cli_verbose_flag() {
        let cli = Cli::parse_from(["vallumix", "-v", "audit"]);
        assert!(cli.verbose);
    }

    #[test]
    fn cli_quiet_flag() {
        let cli = Cli::parse_from(["vallumix", "-q", "audit"]);
        assert!(cli.quiet);
    }

    #[test]
    fn cli_threshold_flag() {
        let cli = Cli::parse_from(["vallumix", "--threshold", "95", "audit"]);
        assert_eq!(cli.threshold, 95);
    }

    #[test]
    fn cli_dry_run_flag() {
        let cli = Cli::parse_from(["vallumix", "--dry-run", "apply"]);
        assert!(cli.dry_run);
    }

    #[test]
    fn cli_report_comma_separated() {
        let cli = Cli::parse_from(["vallumix", "--report", "html,json", "audit"]);
        assert_eq!(cli.report, Some("html,json".into()));
    }

    #[test]
    fn cli_output_flag() {
        let cli = Cli::parse_from(["vallumix", "--output", "/tmp/out", "audit"]);
        assert_eq!(cli.output, Some(std::path::PathBuf::from("/tmp/out")));
    }

    #[test]
    fn cli_rollback_with_control_id() {
        let cli = Cli::parse_from(["vallumix", "rollback", "--control-id", "5.2.4"]);
        match cli.command {
            Commands::Rollback { control_id, session } => {
                assert_eq!(control_id, Some("5.2.4".into()));
                assert_eq!(session, None);
            }
            _ => panic!("expected rollback"),
        }
    }

    #[test]
    fn cli_rollback_without_control_id() {
        let cli = Cli::parse_from(["vallumix", "rollback"]);
        match cli.command {
            Commands::Rollback { control_id, session } => {
                assert_eq!(control_id, None);
                assert_eq!(session, None);
            }
            _ => panic!("expected rollback"),
        }
    }

    #[test]
    fn cli_rollback_with_session() {
        let cli = Cli::parse_from(["vallumix", "rollback", "--session", "web-1234567890"]);
        match cli.command {
            Commands::Rollback { control_id, session } => {
                assert_eq!(control_id, None);
                assert_eq!(session, Some("web-1234567890".into()));
            }
            _ => panic!("expected rollback"),
        }
    }

    #[test]
    fn cli_subcommands_exist() {
        let cmd = Cli::command();
        let subs: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();
        assert!(subs.contains(&"apply"));
        assert!(subs.contains(&"audit"));
        assert!(subs.contains(&"rollback"));
        assert!(subs.contains(&"list"));
        assert!(subs.contains(&"completion"));
    }

    #[test]
    fn verbose_and_quiet_are_mutually_exclusive() {
        let result = Cli::try_parse_from(["vallumix", "-v", "-q", "audit"]);
        assert!(result.is_err());
    }

    #[test]
    fn exit_code_ok_when_above_threshold() {
        assert_eq!(compute_exit_code(90.0, 80), 0);
    }

    #[test]
    fn exit_code_below_threshold() {
        assert_eq!(compute_exit_code(70.0, 80), 1);
    }

    #[test]
    fn exit_code_exact_threshold() {
        assert_eq!(compute_exit_code(80.0, 80), 0);
    }
}
