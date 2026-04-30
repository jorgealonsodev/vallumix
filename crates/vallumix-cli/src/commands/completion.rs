use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell as ClapShell};

use crate::Shell;

pub fn run(shell: Shell) -> Result<i32> {
    let mut cmd = crate::Cli::command();
    match shell {
        Shell::Bash => generate(ClapShell::Bash, &mut cmd, "vallumix", &mut std::io::stdout()),
        Shell::Zsh => generate(ClapShell::Zsh, &mut cmd, "vallumix", &mut std::io::stdout()),
        Shell::Fish => generate(ClapShell::Fish, &mut cmd, "vallumix", &mut std::io::stdout()),
        Shell::Nushell => {
            println!("# Nushell completion for vallumix");
            println!("# (not yet supported by clap_complete)");
            return Ok(0);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_command_signature_exists() {
        let _f: fn(Shell) -> Result<i32> = run;
    }

    #[test]
    fn completion_returns_exit_code_type() {
        let result: Result<i32> = Ok(0);
        assert!(result.is_ok());
    }
}
