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

pub fn detect_from_path(path: impl AsRef<Path>) -> Result<Distro, VallumixError> {
    let content = std::fs::read_to_string(path.as_ref())?;
    let mut id = None;
    let mut version_id = None;

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
                _ => {}
            }
        }
    }

    let id = id.ok_or_else(|| VallumixError::UnsupportedDistro("missing ID".into()))?;
    let version_id = version_id.ok_or_else(|| VallumixError::UnsupportedDistro("missing VERSION_ID".into()))?;

    match (id.as_str(), version_id.as_str()) {
        ("debian", "12") => Ok(Distro::Debian12),
        ("ubuntu", "22.04") => Ok(Distro::Ubuntu2204),
        ("ubuntu", "24.04") => Ok(Distro::Ubuntu2404),
        ("rocky", v) | ("almalinux", v) | ("rhel", v) if v.starts_with("9.") || v == "9" => Ok(Distro::Rocky9),
        _ => Err(VallumixError::UnsupportedDistro(format!("{}/{}", id, version_id))),
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
