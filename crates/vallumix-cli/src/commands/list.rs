use anyhow::Result;
use vallumix_core::control::Control;
use vallumix_controls::registry;

pub fn run() -> Result<i32> {
    let reg = registry();
    for (id, ctrl) in &reg {
        println!("{} - {} ({:?})", id, ctrl.description(), ctrl.severity());
    }
    Ok(0)
}
