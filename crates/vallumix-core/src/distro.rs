use std::path::Path;

use crate::error::VallumixError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distro {
    Debian12,
    Ubuntu2204,
    Ubuntu2404,
    Rocky9,
}

impl std::fmt::Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distro::Debian12 => write!(f, "debian/12"),
            Distro::Ubuntu2204 => write!(f, "ubuntu/22.04"),
            Distro::Ubuntu2404 => write!(f, "ubuntu/24.04"),
            Distro::Rocky9 => write!(f, "rocky/9"),
        }
    }
}

/// Maps a derivative distribution onto the upstream release it is built from.
///
/// Derivatives are only mapped when the evidence is unambiguous, because the
/// answer decides which control set runs: `ID_LIKE` must name the upstream
/// family, and the upstream release must be identified either by
/// `UBUNTU_CODENAME` or by an `ID_LIKE`-consistent `VERSION_ID`. An Ubuntu-like
/// system that carries a codename we do not know is rejected rather than
/// guessed at from its own version number.
fn resolve_derivative(
    id_like: &str,
    ubuntu_codename: Option<&str>,
    version_id: &str,
) -> Option<Distro> {
    let families: Vec<&str> = id_like.split_whitespace().collect();

    if families.contains(&"ubuntu") {
        return match ubuntu_codename {
            Some("jammy") => Some(Distro::Ubuntu2204),
            Some("noble") => Some(Distro::Ubuntu2404),
            Some(_) => None,
            None => match version_id {
                "22.04" => Some(Distro::Ubuntu2204),
                "24.04" => Some(Distro::Ubuntu2404),
                _ => None,
            },
        };
    }

    if families.contains(&"debian") && (version_id == "12" || version_id.starts_with("12.")) {
        return Some(Distro::Debian12);
    }

    None
}

pub fn detect_from_path(path: impl AsRef<Path>) -> Result<Distro, VallumixError> {
    let content = std::fs::read_to_string(path.as_ref())?;
    let mut id = None;
    let mut version_id = None;
    let mut id_like = None;
    let mut ubuntu_codename = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_lowercase();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            match key.as_str() {
                "id" => id = Some(value.to_string()),
                "version_id" => version_id = Some(value.to_string()),
                "id_like" => id_like = Some(value.to_string()),
                "ubuntu_codename" => ubuntu_codename = Some(value.to_string()),
                _ => {}
            }
        }
    }

    let id = id.ok_or_else(|| VallumixError::UnsupportedDistro("missing ID".into()))?;
    let version_id =
        version_id.ok_or_else(|| VallumixError::UnsupportedDistro("missing VERSION_ID".into()))?;

    match (id.as_str(), version_id.as_str()) {
        ("debian", "12") => Ok(Distro::Debian12),
        ("ubuntu", "22.04") => Ok(Distro::Ubuntu2204),
        ("ubuntu", "24.04") => Ok(Distro::Ubuntu2404),
        ("rocky", v) | ("almalinux", v) | ("rhel", v) if v.starts_with("9.") || v == "9" => {
            Ok(Distro::Rocky9)
        }
        _ => id_like
            .as_deref()
            .and_then(|like| resolve_derivative(like, ubuntu_codename.as_deref(), &version_id))
            .ok_or_else(|| VallumixError::UnsupportedDistro(format!("{}/{}", id, version_id))),
    }
}

pub fn detect() -> Result<Distro, VallumixError> {
    detect_from_path("/etc/os-release")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn detect_debian_12() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=debian\nVERSION_ID=\"12\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Debian12);
    }

    #[test]
    fn detect_ubuntu_2204() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=ubuntu\nVERSION_ID=\"22.04\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Ubuntu2204);
    }

    #[test]
    fn detect_ubuntu_2404() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=ubuntu\nVERSION_ID=\"24.04\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Ubuntu2404);
    }

    #[test]
    fn detect_rocky_9() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=rocky\nVERSION_ID=\"9.3\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Rocky9);
    }

    #[test]
    fn detect_almalinux_9() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=almalinux\nVERSION_ID=\"9.2\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Rocky9);
    }

    #[test]
    fn detect_rhel_9() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=rhel\nVERSION_ID=\"9.0\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Rocky9);
    }

    #[test]
    fn detect_linux_mint_22_maps_to_ubuntu_2404() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "ID=linuxmint\nID_LIKE=\"ubuntu debian\"\nVERSION_ID=\"22.3\"\nUBUNTU_CODENAME=noble\n"
        )
        .unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Ubuntu2404);
    }

    #[test]
    fn detect_ubuntu_derivative_jammy_maps_to_ubuntu_2204() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "ID=pop\nID_LIKE=\"ubuntu debian\"\nVERSION_ID=\"22.04\"\nUBUNTU_CODENAME=jammy\n"
        )
        .unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Ubuntu2204);
    }

    #[test]
    fn detect_ubuntu_derivative_without_codename_uses_version_id() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=zorin\nID_LIKE=ubuntu\nVERSION_ID=\"24.04\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Ubuntu2404);
    }

    #[test]
    fn detect_debian_derivative_maps_to_debian_12() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=raspbian\nID_LIKE=debian\nVERSION_ID=\"12\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Debian12);
    }

    #[test]
    fn detect_rejects_ubuntu_derivative_on_an_unsupported_codename() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "ID=linuxmint\nID_LIKE=\"ubuntu debian\"\nVERSION_ID=\"21.3\"\nUBUNTU_CODENAME=focal\n"
        )
        .unwrap();
        let result = detect_from_path(tmpfile.path());
        assert!(matches!(result, Err(VallumixError::UnsupportedDistro(_))));
    }

    #[test]
    fn detect_rejects_debian_derivative_on_an_unsupported_release() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=devuan\nID_LIKE=debian\nVERSION_ID=\"5\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert!(matches!(result, Err(VallumixError::UnsupportedDistro(_))));
    }

    #[test]
    fn detect_ignores_the_derivative_own_codename() {
        // Linux Mint sets VERSION_CODENAME to its own release name; only
        // UBUNTU_CODENAME identifies the upstream it is built from.
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "ID=linuxmint\nID_LIKE=\"ubuntu debian\"\nVERSION_ID=\"22.3\"\nVERSION_CODENAME=zena\nUBUNTU_CODENAME=noble\n"
        )
        .unwrap();
        let result = detect_from_path(tmpfile.path());
        assert_eq!(result.unwrap(), Distro::Ubuntu2404);
    }

    #[test]
    fn detect_unsupported_names_the_id_and_version() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=fedora\nVERSION_ID=\"40\"\n").unwrap();
        match detect_from_path(tmpfile.path()) {
            Err(VallumixError::UnsupportedDistro(msg)) => {
                assert!(
                    msg.contains("fedora"),
                    "message should name the id: {}",
                    msg
                );
                assert!(
                    msg.contains("40"),
                    "message should name the version: {}",
                    msg
                );
            }
            other => panic!("expected UnsupportedDistro, got {:?}", other),
        }
    }

    #[test]
    fn detect_unsupported() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "ID=fedora\nVERSION_ID=\"40\"\n").unwrap();
        let result = detect_from_path(tmpfile.path());
        assert!(matches!(result, Err(VallumixError::UnsupportedDistro(_))));
    }

    #[test]
    fn detect_missing_file() {
        let result = detect_from_path("/nonexistent/os-release");
        assert!(matches!(result, Err(VallumixError::Io(_))));
    }
}
