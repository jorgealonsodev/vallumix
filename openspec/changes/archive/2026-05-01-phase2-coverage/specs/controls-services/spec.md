# controls-services Specification

## Purpose

Implement CIS 2.x service-disable controls that detect and disable unnecessary network services, each implementing the `Control` trait with `category()` returning `Category::Services`.

## Requirements

### Requirement: Service Disable Controls

The system MUST provide controls that detect and disable unnecessary services using the distro-appropriate service manager (`systemctl` on all supported distros). Each control MUST implement `check()`, `apply()`, and `rollback()` using the `with_paths()` testability pattern.

#### Scenario: disable_cups detects running service

- GIVEN `cups` service is active and enabled
- WHEN `DisableCups::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence containing service status

#### Scenario: disable_cups applies and creates backup

- GIVEN `cups` service is active
- WHEN `DisableCups::apply(&ctx)` is called
- THEN the service is stopped, disabled, and a backup records the prior state

#### Scenario: Service already disabled — AlreadyCompliant

- GIVEN `cups` service is already inactive and disabled
- WHEN `DisableCups::apply(&ctx)` is called
- THEN `ApplyStatus::AlreadyCompliant` is returned with no side effects

### Requirement: Network Service Controls

The system MUST implement `disable_dhcp` (CIS 2.2.4), `disable_ldap` (CIS 2.2.5), `disable_nfs` (CIS 2.2.6), `disable_rpcbind` (CIS 2.2.7), `disable_bind` (CIS 2.2.8). Each MUST have `severity()` returning `Severity::Medium` or appropriate level per CIS.

#### Scenario: disable_nfs checks service and package

- GIVEN NFS server package is installed or `nfs-server` is active
- WHEN `DisableNfs::check(&ctx)` is called
- THEN it returns `NonCompliant` with evidence about the NFS service status

#### Scenario: disable_rpcbind rollback restores service

- GIVEN `DisableRpcbind::apply()` has been executed
- WHEN `rollback(&ctx, &backup)` is called
- THEN the `rpcbind` service is re-enabled and started

### Requirement: Insecure Service Controls

The system MUST implement `disable_vsftpd` (CIS 2.2.9), `disable_httpd` (CIS 2.2.10), `disable_dovecot` (CIS 2.2.11), `disable_snmpd` (CIS 2.2.14), `disable_rsync` (CIS 2.2.15). `severity()` MUST return `Severity::Medium` for all.

#### Scenario: disable_vsftpd handles absent package gracefully

- GIVEN `vsftpd` package is not installed
- WHEN `DisableVsftpd::check(&ctx)` is called
- THEN it returns `Compliant` — absent packages are inherently compliant

### Requirement: Legacy Service Controls

The system MUST implement `disable_xinetd` (CIS 2.2.12). If `xinetd` is not installed, `check()` MUST return `Compliant`. `applicable_distros()` for legacy services SHOULD include only distros where the service is commonly found.

#### Scenario: disable_xinetd on system without xinetd

- GIVEN `xinetd` package is not installed
- WHEN `DisableXinetd::check(&ctx)` is called
- THEN it returns `Compliant` with evidence "xinetd not installed"

### Requirement: Avahi Already Implemented

The `disable_avahi` control (CIS 2.2.3) already exists in the pilot. It MUST receive `category()` returning `Category::Services`. No new implementation is needed.

#### Scenario: disable_avahi returns Services category

- GIVEN `DisableAvahi` implements `Control`
- WHEN `category()` is called
- THEN it returns `Category::Services`

### Requirement: Service Controls Distro Awareness

Each service control MUST detect the distro-appropriate service manager. On all supported distros (Debian12, Ubuntu2204, Ubuntu2404, Rocky9), `systemctl` is used. Controls MUST handle packages vs services distinction — a package not installed means the service is compliant.

#### Scenario: Service control with distro-specific paths

- GIVEN `DisableDhcp` constructed with `with_paths(MockPaths { systemctl, services_dir })`
- WHEN `check(&ctx)` is called on a system where `dhcpd` is not installed
- THEN it returns `Compliant`

## Acceptance Criteria

- [ ] 8-10 service controls implemented with CIS IDs
- [ ] All controls implement `Control` trait including `category()`
- [ ] `with_paths()` test pattern for every service control
- [ ] Absent packages/services return `Compliant`
- [ ] Rollback re-enables and starts services