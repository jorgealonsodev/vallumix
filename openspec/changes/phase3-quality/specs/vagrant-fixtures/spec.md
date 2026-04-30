# vagrant-fixtures Specification

## Purpose

Provide reproducible, SSH-able virtual machine fixtures for Debian 12, Ubuntu 24.04, and Rocky 9 to enable manual and integration testing of vallumix controls on real Linux systems.

## Requirements

### Requirement: Vagrantfile Multi-VM Configuration

The project MUST include a `Vagrantfile` at the repository root defining three VMs: `debian12` (generic/debian12), `ubuntu2404` (generic/ubuntu2404), and `rocky9` (generic/rocky9). Each VM MUST have 512MB RAM, 1 vCPU, and a private network IP on the 192.168.56.0/24 subnet.

#### Scenario: Vagrant up provisions all three VMs

- GIVEN the Vagrantfile at the repository root
- WHEN `vagrant up` is executed
- THEN three VMs are created: debian12 (192.168.56.10), ubuntu2404 (192.168.56.11), rocky9 (192.168.56.12)

#### Scenario: VMs are SSH-accessible

- GIVEN all three VMs are running
- WHEN `vagrant ssh debian12` is executed
- THEN the user gains shell access to the Debian 12 VM without password prompts

### Requirement: Idempotent Provisioning Scripts

Each VM MUST have a provision script (`provision/debian12.sh`, `provision/ubuntu2404.sh`, `provision/rocky9.sh`) that installs the Rust toolchain, builds vallumix, and creates a baseline snapshot. Provisioning MUST be idempotent — running `vagrant provision` twice MUST NOT fail or produce side effects.

#### Scenario: Provision script is idempotent

- GIVEN debian12 VM is running and already provisioned
- WHEN `vagrant provision debian12` is executed a second time
- THEN the script completes with exit code 0 and no duplicate packages or entries are created

#### Scenario: Provision installs Rust and builds vallumix

- GIVEN a freshly created VM
- WHEN the provision script runs
- THEN `rustc --version` outputs 1.75.x, and `cargo build --release` succeeds

### Requirement: Baseline Audit Execution

The provision script MUST run `vallumix audit --profile web --report json` as a post-build step to establish a baseline compliance report. The output MUST be saved to `/vagrant/baseline-{distro}.json` on the host.

#### Scenario: Baseline audit runs on provisioned VM

- GIVEN the provision script has built vallumix
- WHEN it executes `vallumix audit --profile web --report json`
- THEN a baseline JSON report is written to `/vagrant/baseline-debian12.json`

### Requirement: Vagrant Configuration File

The Vagrantfile MUST support a configurable `VALLUMIX_PROFILE` environment variable (default: `web`) that passes the profile to provision scripts. The Vagrantfile MUST also support `VALLUMIX_DRY_RUN=1` to run apply commands in dry-run mode.

#### Scenario: Custom profile via environment variable

- GIVEN `VALLUMIX_PROFILE=database` is set in the environment
- WHEN `vagrant up` is executed
- THEN provision scripts use `--profile database` instead of `--profile web`

#### Scenario: Dry-run mode prevents system changes

- GIVEN `VALLUMIX_DRY_RUN=1` is set in the environment
- WHEN the provision script runs vallumix apply
- THEN the `--dry-run` flag is passed and no system changes occur

### Requirement: Vagrant Cleanup and Reprovision

The Vagrantfile MUST support `vagrant destroy -f` to remove all VMs and `vagrant snapshot save` / `vagrant snapshot restore` for saving and restoring VM state after testing.

#### Scenario: Destroy and reprovision from clean state

- GIVEN VMs have been modified by vallumix apply
- WHEN `vagrant destroy -f && vagrant up` is executed
- THEN fresh VMs are provisioned without artifacts from previous runs

#### Scenario: Snapshot and restore VM state

- GIVEN a VM is in its baseline provisioned state
- WHEN `vagrant snapshot save debian12 baseline` is executed
- THEN the VM state is saved and can be restored with `vagrant snapshot restore debian12 baseline`