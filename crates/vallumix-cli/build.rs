use std::env;
use std::fs;
use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let man_path = PathBuf::from(&out_dir).join("vallumix.1");

    let cmd = <cli::Cli as clap::CommandFactory>::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Vec::new();
    man.render(&mut buffer).expect("Failed to render man page");
    fs::write(&man_path, buffer).expect("Failed to write man page");

    // Copy to project root man/ directory for packaging
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project_root = manifest_dir.parent().unwrap().parent().unwrap().to_path_buf();
    let target_dir = project_root.join("man");
    fs::create_dir_all(&target_dir).ok();
    fs::copy(&man_path, target_dir.join("vallumix.1")).ok();
}
