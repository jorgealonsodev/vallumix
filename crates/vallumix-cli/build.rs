use clap_complete::Shell;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // The repository root, two levels above `crates/vallumix-cli`.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project_root = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    generate_man_page(&out_dir, &project_root);
    generate_completions(&out_dir, &project_root);
}

fn generate_man_page(out_dir: &Path, project_root: &Path) {
    let man_path = out_dir.join("vallumix.1");

    let cmd = <cli::Cli as clap::CommandFactory>::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer).expect("Failed to render man page");
    fs::write(&man_path, buffer).expect("Failed to write man page");

    // Copy to project root man/ directory for packaging
    let target_dir = project_root.join("man");
    fs::create_dir_all(&target_dir).ok();
    fs::copy(&man_path, target_dir.join("vallumix.1")).ok();
}

fn generate_completions(out_dir: &Path, project_root: &Path) {
    // File names are fixed by the packaging assets in Cargo.toml, so they are
    // spelled out here instead of relying on clap_complete's defaults.
    let shells = [(Shell::Bash, "vallumix.bash"), (Shell::Zsh, "_vallumix")];

    // Copy to project root completions/ directory for packaging
    let target_dir = project_root.join("completions");
    fs::create_dir_all(&target_dir).ok();

    for (shell, file_name) in shells {
        let mut cmd = <cli::Cli as clap::CommandFactory>::command();
        let mut buffer: Vec<u8> = Vec::new();
        clap_complete::generate(shell, &mut cmd, "vallumix", &mut buffer);

        let completion_path = out_dir.join(file_name);
        fs::write(&completion_path, buffer).expect("Failed to write shell completion");
        fs::copy(&completion_path, target_dir.join(file_name)).ok();
    }
}
