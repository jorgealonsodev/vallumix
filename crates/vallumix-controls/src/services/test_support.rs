//! Shared test harness for the service controls.
//!
//! WHY THIS EXISTS — do not "simplify" it back into writing one executable
//! script per test:
//!
//! Every test used to write its own executable fake `systemctl` into its own
//! tempdir and exec it immediately. Rust runs tests as threads of a SINGLE
//! process, so while thread A still holds the write fd to the script it just
//! created, thread B's `fork`/`posix_spawn` transiently inherits that fd in the
//! child. Thread A's `exec` of its script then fails with ETXTBSY
//! ("Text file busy", errno 26). `O_CLOEXEC` does not help: it only closes the
//! fd at exec time, and the racy window is between fork and exec in the OTHER
//! process. Under parallel `cargo test` this failed roughly half the time.
//!
//! The race is removed, not retried:
//!
//! 1. The executable is written EXACTLY ONCE per test binary (`OnceLock`).
//! 2. That one-time init drains the fork/exec window before returning, so once
//!    `shared_dir()` hands the path out no process anywhere holds a write fd to
//!    that inode, and nothing ever opens it for writing again.
//! 3. Tests no longer vary the script: each test uses a UNIQUE service name and
//!    only writes plain, non-executable data files (a `<service>.state` file and
//!    a `<service>.service` unit file). Plain data files can never cause
//!    ETXTBSY, because ETXTBSY only applies to files being executed.

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use tempfile::TempDir;

/// `ETXTBSY` on Linux. Spelled as a raw errno instead of
/// `ErrorKind::ExecutableFileBusy` because that variant is newer than the
/// workspace MSRV.
const ETXTBSY: i32 = 26;

/// The fake `systemctl` reads the desired answers from `<dirname $0>/$2.state`,
/// whose first line is `active`/`inactive` and second line is
/// `enabled`/`disabled`. `$2` is the service name, so ONE script serves every
/// test. Lifecycle verbs just succeed.
const FAKE_SYSTEMCTL: &str = r#"#!/bin/bash
state="$(dirname "$0")/$2.state"
case "$1" in
  is-active) sed -n '1p' "$state" ;;
  is-enabled) sed -n '2p' "$state" ;;
esac
exit 0
"#;

static SHARED: OnceLock<TempDir> = OnceLock::new();
static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// The one directory holding the single fake `systemctl` plus every test's
/// plain state and unit files.
fn shared_dir() -> &'static Path {
    SHARED
        .get_or_init(|| {
            let dir = tempfile::tempdir().expect("create shared test dir");
            let script = dir.path().join("systemctl");
            {
                let mut f = std::fs::File::create(&script).expect("create fake systemctl");
                f.write_all(FAKE_SYSTEMCTL.as_bytes())
                    .expect("write fake systemctl");
                f.flush().expect("flush fake systemctl");
            }
            std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))
                .expect("chmod fake systemctl");

            // Drain the fork/exec window ONCE. Another test thread may have
            // forked while the write fd above was still open; its child keeps
            // the inherited fd until it execs. Retrying here — inside the
            // one-time init, never around an assertion — means every later exec
            // of this script is deterministic, because after this point nothing
            // opens the file for writing again.
            let deadline = Instant::now() + Duration::from_secs(10);
            loop {
                match Command::new(&script).args(["is-active", "probe"]).output() {
                    Ok(_) => break,
                    Err(e) if e.raw_os_error() == Some(ETXTBSY) && Instant::now() < deadline => {
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(e) => panic!("fake systemctl is not executable: {e}"),
                }
            }
            dir
        })
        .path()
}

/// A fake service: a unique name, the shared fake `systemctl`, and a unit file.
pub struct FakeService {
    pub name: String,
    pub systemctl_path: PathBuf,
    pub unit_file: PathBuf,
}

/// Registers a fake service whose `systemctl is-active` / `is-enabled` answers
/// are `active` and `enabled` (pass `"inactive"` / `"disabled"` for the
/// compliant case). Only plain data files are written here.
pub fn fake_service(active: &str, enabled: &str) -> FakeService {
    let dir = shared_dir();
    let name = format!("test-svc-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed));
    std::fs::write(
        dir.join(format!("{name}.state")),
        format!("{active}\n{enabled}\n"),
    )
    .expect("write service state file");
    let unit_file = dir.join(format!("{name}.service"));
    std::fs::write(&unit_file, "[Service]\n").expect("write unit file");
    FakeService {
        name,
        systemctl_path: dir.join("systemctl"),
        unit_file,
    }
}

/// Registers a fake service whose unit file exists but whose `systemctl` path
/// points at a file that does not exist, so every probe fails to execute.
pub fn unrunnable_systemctl_service() -> FakeService {
    let mut svc = fake_service("inactive", "disabled");
    svc.systemctl_path = shared_dir().join(format!("{}-missing-systemctl", svc.name));
    svc
}
