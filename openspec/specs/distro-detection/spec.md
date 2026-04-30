# distro-detection Specification

## Purpose

Enable runtime detection of the Linux distribution by parsing `/etc/os-release`, returning a typed `Distro` enum variant for supported distributions and a clear error for unsupported ones.

## Requirements

### Requirement: Parse /etc/os-release

The system MUST parse `/etc/os-release` to extract `ID` and `VERSION_ID` fields. The parser MUST handle case-insensitive keys, quoted values (single and double quotes), and optional whitespace around the `=` delimiter.

#### Scenario: Standard os-release is parsed correctly

- GIVEN a file at `/etc/os-release` containing `ID=debian` and `VERSION_ID="12"`
- WHEN `detect()` is called
- THEN it returns `Ok(Distro::Debian12)`

#### Scenario: Quoted values are unquoted

- GIVEN `VERSION_ID="22.04"` in `/etc/os-release`
- WHEN the parser reads the `VERSION_ID` key
- THEN the value is `"22.04"` (unquoted, not `'"22.04"'`)

#### Scenario: Missing os-release file

- GIVEN `/etc/os-release` does not exist
- WHEN `detect()` is called
- THEN it returns `Err(VallumixError::UnsupportedDistro)` with a message indicating the file was not found

### Requirement: Detect Supported Distributions

The system MUST map parsed `ID` and `VERSION_ID` combinations to `Distro` enum variants: `ID=debian` + `VERSION_ID=12` → `Debian12`, `ID=ubuntu` + `VERSION_ID=22.04` → `Ubuntu2204`, `ID=ubuntu` + `VERSION_ID=24.04` → `Ubuntu2404`, `ID=rhel` / `ID=rocky` / `ID=almalinux` + `VERSION_ID` starting with `9` → `Rocky9`.

#### Scenario: Ubuntu 24.04 is detected

- GIVEN `/etc/os-release` with `ID=ubuntu` and `VERSION_ID="24.04"`
- WHEN `detect()` is called
- THEN it returns `Ok(Distro::Ubuntu2404)`

#### Scenario: Rocky Linux 9 maps to Rocky9

- GIVEN `/etc/os-release` with `ID=rocky` and `VERSION_ID="9.3"`
- WHEN `detect()` is called
- THEN it returns `Ok(Distro::Rocky9)`

#### Scenario: AlmaLinux 9 maps to Rocky9

- GIVEN `/etc/os-release` with `ID=almalinux` and `VERSION_ID="9.2"`
- WHEN `detect()` is called
- THEN it returns `Ok(Distro::Rocky9)`

### Requirement: Reject Unsupported Distributions

The system MUST return `Err(VallumixError::UnsupportedDistro)` when the detected distribution is not in the supported list. The error message MUST include the actual `ID` and `VERSION_ID` values.

#### Scenario: Unsupported distro returns typed error

- GIVEN `/etc/os-release` with `ID=fedora` and `VERSION_ID="40"`
- WHEN `detect()` is called
- THEN it returns `Err(VallumixError::UnsupportedDistro)` and the error display includes "fedora"

#### Scenario: Supported ID with unsupported version

- GIVEN `/etc/os-release` with `ID=ubuntu` and `VERSION_ID="20.04"`
- WHEN `detect()` is called
- THEN it returns `Err(VallumixError::UnsupportedDistro)`

### Requirement: Override Path for Testing

The `detect()` function MUST accept an optional path override to enable unit testing without a real `/etc/os-release` file. The default path MUST be `/etc/os-release`.

#### Scenario: Custom path is used when provided

- GIVEN a test fixture file at `/tmp/test-os-release` with `ID=debian` and `VERSION_ID="12"`
- WHEN `detect_from_path("/tmp/test-os-release")` is called
- THEN it returns `Ok(Distro::Debian12)`
