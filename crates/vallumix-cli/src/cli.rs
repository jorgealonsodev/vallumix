use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(name = "vallumix")]
#[command(about = "Modular Linux hardening with CIS profiles")]
#[command(version)]
pub struct Cli {
    #[arg(long, global = true, default_value = "web")]
    pub profile: String,

    #[arg(long, global = true)]
    pub dry_run: bool,

    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[arg(long, global = true, default_value = "80", value_parser = clap::value_parser!(u8).range(0..=100))]
    pub threshold: u8,

    #[arg(long, global = true)]
    pub no_color: bool,

    #[arg(long, global = true, value_name = "FORMATS")]
    pub report: Option<String>,

    #[arg(long, global = true, value_name = "PATH")]
    pub output: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ReportFormat {
    Json,
    Html,
    Junit,
    Text,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Apply,
    Audit,
    Rollback {
        #[arg(long)]
        control_id: Option<String>,
        #[arg(long)]
        session: Option<String>,
    },
    List,
    Completion {
        shell: Shell,
    },
}
