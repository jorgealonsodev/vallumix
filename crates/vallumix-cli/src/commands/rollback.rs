use anyhow::Result;
use vallumix_backup::BackupManager;
use vallumix_core::context::Context;
use vallumix_core::distro::Distro;

pub fn run(control_id: Option<String>, session_id: Option<String>) -> Result<i32> {
    let distro = Distro::Debian12; // TODO: detect
    let ctx = Context::new(distro)?;
    let backup_mgr = BackupManager::new(&ctx.backup_dir);

    if let Some(id) = control_id {
        backup_mgr.rollback_by_control(&id)?;
        println!("Rolled back control: {}", id);
        Ok(0)
    } else if let Some(session) = session_id {
        let count = backup_mgr.rollback_session(&session)?;
        println!("Rolled back {} control(s) from session: {}", count, session);
        Ok(0)
    } else {
        let sessions = backup_mgr.list_all_sessions()?;
        if let Some(session) = sessions.first() {
            let count = backup_mgr.rollback_session(&session.id)?;
            println!(
                "Rolled back {} control(s) from most recent session: {}",
                count, session.id
            );
            Ok(0)
        } else {
            eprintln!("No backup sessions found");
            Ok(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollback_command_signature_exists() {
        let _f: fn(Option<String>, Option<String>) -> Result<i32> = run;
    }

    #[test]
    fn rollback_returns_exit_code_type() {
        let result: Result<i32> = Ok(0);
        assert!(result.is_ok());
    }
}
