# packer-example Specification

## Purpose

Provide a working Packer HCL example that demonstrates using Vallumix in a CI/CD provisioning pipeline, validating PRD use case CU-05 (DevOps integration) and persona 2 (Carlos, DevOps engineer).

## Requirements

### Requirement: Packer HCL Template

`examples/packer/vallumix-hardened.pkr.hcl` MUST define a QEMU builder targeting Debian 12, with a shell provisioner that downloads and executes Vallumix with the web profile using the `apply` subcommand.

#### Scenario: Packer build produces hardened image

- GIVEN Packer and QEMU are installed
- WHEN `packer build vallumix-hardened.pkr.hcl` is run
- THEN a QEMU VM image is produced without errors

#### Scenario: Template uses QEMU builder (no cloud credentials required)

- GIVEN the Packer template is read
- WHEN the `source` block is inspected
- THEN it uses `qemu` builder with Debian 12 image, not a cloud provider

### Requirement: Vallumix Provisioner Script

The template MUST include a shell provisioner script that either downloads the Vallumix binary from a GitHub release or installs via the `.deb` package, then runs `sudo vallumix apply --profile web --report html`.

#### Scenario: Provisioner executes Vallumix

- GIVEN the Packer build reaches the provisioner step
- WHEN the shell provisioner runs
- THEN `vallumix apply --profile web --report html` executes successfully

#### Scenario: Provisioner works with .deb installation

- GIVEN the `.deb` package URL is specified in variables
- WHEN the provisioner installs the package via `dpkg -i`
- THEN `vallumix` is available at `/usr/bin/vallumix`

### Requirement: Example Documentation

An `examples/packer/README.md` MUST explain prerequisites (Packer, QEMU), provide build commands, describe expected output, and link to the main mdBook docs.

#### Scenario: README enables standalone execution

- GIVEN `examples/packer/README.md` is read by a developer
- WHEN they follow the instructions
- THEN they can run `packer build` without consulting external documentation

#### Scenario: README links to project docs

- GIVEN the Packer example README
- WHEN the documentation links are followed
- THEN they resolve to the project mdBook or main README