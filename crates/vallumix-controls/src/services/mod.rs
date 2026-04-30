pub mod common;
pub mod disable_avahi;

macro_rules! def_service_control {
    ($name:ident, $id:expr, $desc:expr, $svc:expr, $sev:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name(super::common::ServiceDisable);

        impl $name {
            pub fn new() -> Self {
                $name(super::common::ServiceDisable::new($id, $desc, $svc, $sev))
            }
            pub fn with_paths(service_name: std::string::String, systemctl_path: std::path::PathBuf) -> Self {
                $name(super::common::ServiceDisable::with_paths($id, $desc, service_name, systemctl_path, $sev))
            }
        }

        impl vallumix_core::control::Control for $name {
            fn id(&self) -> &str { self.0.id() }
            fn description(&self) -> &str { self.0.description() }
            fn severity(&self) -> vallumix_core::control::Severity { self.0.severity() }
            fn applicable_distros(&self) -> &[vallumix_core::distro::Distro] { self.0.applicable_distros() }
            fn category(&self) -> vallumix_core::control::Category { self.0.category() }
            fn check(&self, ctx: &vallumix_core::context::Context) -> vallumix_core::control::ControlResult {
                self.0.check(ctx)
            }
            fn apply(&self, ctx: &vallumix_core::context::Context) -> std::result::Result<vallumix_core::control::ApplyResult, vallumix_core::error::ControlError> {
                self.0.apply(ctx)
            }
            fn rollback(&self, ctx: &vallumix_core::context::Context, backup: &vallumix_core::profile::Backup) -> std::result::Result<(), vallumix_core::error::ControlError> {
                self.0.rollback(ctx, backup)
            }
            fn clone_box(&self) -> std::boxed::Box<dyn vallumix_core::control::Control> {
                std::boxed::Box::new(self.clone())
            }
        }
    };
}

pub mod disable_cups { def_service_control!(DisableCups, "2.2.2", "Ensure CUPS is not enabled", "cups", vallumix_core::control::Severity::Low); }
pub mod disable_dhcp { def_service_control!(DisableDhcp, "2.2.4", "Ensure DHCP Server is not enabled", "isc-dhcp-server", vallumix_core::control::Severity::Low); }
pub mod disable_ldap { def_service_control!(DisableLdap, "2.2.5", "Ensure LDAP server is not enabled", "slapd", vallumix_core::control::Severity::Low); }
pub mod disable_nfs { def_service_control!(DisableNfs, "2.2.6", "Ensure NFS is not enabled", "nfs-kernel-server", vallumix_core::control::Severity::Low); }
pub mod disable_rpcbind { def_service_control!(DisableRpcbind, "2.2.7", "Ensure RPCBind is not enabled", "rpcbind", vallumix_core::control::Severity::Low); }
pub mod disable_bind { def_service_control!(DisableBind, "2.2.8", "Ensure DNS Server is not enabled", "named", vallumix_core::control::Severity::Low); }
pub mod disable_vsftpd { def_service_control!(DisableVsftpd, "2.2.9", "Ensure FTP Server is not enabled", "vsftpd", vallumix_core::control::Severity::Low); }
pub mod disable_httpd { def_service_control!(DisableHttpd, "2.2.10", "Ensure HTTP Server is not enabled", "httpd", vallumix_core::control::Severity::Low); }
pub mod disable_dovecot { def_service_control!(DisableDovecot, "2.2.11", "Ensure IMAP and POP3 server is not enabled", "dovecot", vallumix_core::control::Severity::Low); }
pub mod disable_snmpd { def_service_control!(DisableSnmpd, "2.2.14", "Ensure SNMP Server is not enabled", "snmpd", vallumix_core::control::Severity::Low); }
pub mod disable_rsync { def_service_control!(DisableRsync, "2.2.15", "Ensure rsync service is not enabled", "rsync", vallumix_core::control::Severity::Low); }
pub mod disable_xinetd { def_service_control!(DisableXinetd, "2.2.12", "Ensure xinetd is not installed", "xinetd", vallumix_core::control::Severity::Low); }
