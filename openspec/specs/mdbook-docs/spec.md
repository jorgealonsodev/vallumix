# mdbook-docs Specification

## Purpose

Establish bilingual (English/Spanish) mdBook documentation structure covering Getting Started, CLI Reference, Profiles, CIS Mapping, and Contributing, deployed to GitHub Pages per PRD §3.2 and §12.3 (item 5).

## Requirements

### Requirement: Bilingual Book Structure

The project MUST contain two mdBook instances: `docs/en/` (English) and `docs/es/` (Spanish), each with its own `book.toml` and `src/SUMMARY.md` that build independently.

#### Scenario: English book builds successfully

- GIVEN the `docs/en/` directory exists
- WHEN `mdbook build` is run inside `docs/en/`
- THEN a complete English book is generated in `docs/en/book/`

#### Scenario: Spanish book builds successfully

- GIVEN the `docs/es/` directory exists
- WHEN `mdbook build` is run inside `docs/es/`
- THEN a complete Spanish book is generated in `docs/es/book/`

### Requirement: Content Chapters

Each book MUST contain at minimum: Getting Started, CLI Reference, Profiles Guide, CIS Control Mapping, and Contributing chapters with substantive content (not stubs).

#### Scenario: English SUMMARY.md lists all required chapters

- GIVEN `docs/en/src/SUMMARY.md`
- WHEN the file is read
- THEN it contains entries for Getting Started, CLI Reference, Profiles, CIS Mapping, and Contributing

#### Scenario: Chapter files contain real content

- GIVEN any chapter source file in `docs/en/src/`
- WHEN word count is checked
- THEN each chapter has ≥100 words of content (not placeholder text)

### Requirement: GitHub Pages Deployment

A `.github/workflows/docs.yml` workflow MUST build both books and deploy them to GitHub Pages at `/en/` and `/es/` paths, triggered on push to `develop`.

#### Scenario: Docs workflow deploys on develop push

- GIVEN a push to `develop` branch
- WHEN the docs workflow triggers
- THEN both books are built, and the `gh-pages` branch is updated with `/en/` and `/es/` content

#### Scenario: Docs workflow does not break main CI

- GIVEN both `ci.yml` and `docs.yml` run on the same push
- WHEN docs deployment fails
- THEN the CI workflow is unaffected

### Requirement: API Docs Cross-Reference

Each book MUST link to `cargo doc`-generated API documentation, either embedded or linked from a dedicated chapter.

#### Scenario: API Reference link resolves

- GIVEN the English book is built
- WHEN a user clicks the API Reference link
- THEN they reach the generated `vallumix-core` API documentation