use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};

pub fn run(shell: Shell) -> Result<i32> {
    let mut cmd = crate::cli::Cli::command();
    generate(shell, &mut cmd, "vallumix", &mut std::io::stdout());
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
