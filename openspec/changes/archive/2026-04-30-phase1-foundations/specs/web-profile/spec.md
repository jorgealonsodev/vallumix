# web-profile Specification

## Purpose

Populate `profiles/web.toml` with approximately 20 control IDs relevant to a web server hardening profile, ensuring it is parseable by the `Profile` struct's TOML deserialization.

## Requirements

### Requirement: web.toml Structure

The `profiles/web.toml` file MUST contain three fields: `name` set to `"web"`, `description` set to a non-empty string describing the web server profile, and `controls` set to a non-empty array of CIS control ID strings. The file MUST be valid TOML parseable by `toml::from_str::<Profile>()`.

#### Scenario: web.toml parses into Profile struct

- GIVEN the file `profiles/web.toml`
- WHEN `Profile::from_file("profiles/web.toml")` is called
- THEN it returns `Ok(Profile)` with `name` equal to `"web"` and `controls` non-empty

#### Scenario: web.toml is valid TOML

- GIVEN the content of `profiles/web.toml`
- WHEN parsed by a TOML validator
- THEN it produces no parse errors

### Requirement: Control IDs Span CIS Categories

The `controls` array in `web.toml` MUST include IDs from at least 5 of the 6 CIS categories defined in PRD §5.6: initial configuration (1.x), services (2.x), network configuration (3.x), logging/auditing (4.x), access/authentication (5.x), and system maintenance (6.x). Phase 1 pilot controls (`1.1.1.1`, `3.1.1`, `5.2.4`, `6.1.1`) MUST be included.

#### Scenario: Profile includes pilot control IDs

- GIVEN `profiles/web.toml` is loaded
- WHEN the controls list is inspected
- THEN it contains `"1.1.1.1"`, a CIS 3.x ID, `"5.2.4"`, and a CIS 6.x ID

#### Scenario: Profile spans multiple CIS categories

- GIVEN the controls list from `web.toml`
- WHEN control IDs are grouped by their CIS category prefix (first digit)
- THEN at least 5 distinct categories are represented

### Requirement: Profile Integrity with Registry

When `Profile::resolve_controls(&registry)` is called with the pilot control registry containing the 5 implemented controls, all pilot control IDs in `web.toml` MUST resolve successfully. Non-pilot IDs (not yet implemented) MUST NOT cause a panic — they are simply skipped or reported as missing.

#### Scenario: All pilot controls resolve

- GIVEN `web.toml` contains `"1.1.1.1"` and `"5.2.4"`
- WHEN `profile.resolve_controls(&pilot_registry)` is called with a registry containing those controls
- THEN the resolved list includes `Box<dyn Control>` instances for those IDs

#### Scenario: Missing control IDs are reported

- GIVEN `web.toml` contains a CIS ID not yet implemented (e.g., `"2.2.1"`)
- WHEN `profile.resolve_controls(&pilot_registry)` is called
- THEN the result reports the missing ID without panicking