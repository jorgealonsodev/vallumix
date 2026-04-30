#![allow(missing_docs)]

pub mod auth;
pub mod filesystem;
pub mod logging;
pub mod maintenance;
pub mod network;
pub mod services;
pub mod ssh;

use std::collections::HashMap;

use vallumix_core::control::Control;

pub fn registry() -> HashMap<String, Box<dyn Control>> {
    let mut m: HashMap<String, Box<dyn Control>> = HashMap::new();

    // Filesystem controls
    m.insert("1.1.1.1".into(), Box::new(filesystem::disable_cramfs::DisableCramfs::new()));
    m.insert("1.1.1.2".into(), Box::new(filesystem::disable_freevxfs::DisableFreevxfs::new()));
    m.insert("1.1.1.3".into(), Box::new(filesystem::disable_jffs2::DisableJffs2::new()));
    m.insert("1.1.1.4".into(), Box::new(filesystem::disable_hfs::DisableHfs::new()));
    m.insert("1.1.1.5".into(), Box::new(filesystem::disable_hfsplus::DisableHfsplus::new()));
    m.insert("1.1.1.6".into(), Box::new(filesystem::disable_squashfs::DisableSquashfs::new()));
    m.insert("1.1.1.7".into(), Box::new(filesystem::disable_udf::DisableUdf::new()));
    m.insert("1.1.10".into(), Box::new(filesystem::disable_usb_storage::DisableUsbStorage::new()));
    m.insert("1.1.2.1".into(), Box::new(filesystem::harden_tmpfs::HardenTmpfs::new()));

    // Service controls
    m.insert("2.2.2".into(), Box::new(services::disable_cups::DisableCups::new()));
    m.insert("2.2.3".into(), Box::new(services::disable_avahi::DisableAvahi::new()));
    m.insert("2.2.4".into(), Box::new(services::disable_dhcp::DisableDhcp::new()));
    m.insert("2.2.5".into(), Box::new(services::disable_ldap::DisableLdap::new()));
    m.insert("2.2.6".into(), Box::new(services::disable_nfs::DisableNfs::new()));
    m.insert("2.2.7".into(), Box::new(services::disable_rpcbind::DisableRpcbind::new()));
    m.insert("2.2.8".into(), Box::new(services::disable_bind::DisableBind::new()));
    m.insert("2.2.9".into(), Box::new(services::disable_vsftpd::DisableVsftpd::new()));
    m.insert("2.2.10".into(), Box::new(services::disable_httpd::DisableHttpd::new()));
    m.insert("2.2.11".into(), Box::new(services::disable_dovecot::DisableDovecot::new()));
    m.insert("2.2.12".into(), Box::new(services::disable_xinetd::DisableXinetd::new()));
    m.insert("2.2.14".into(), Box::new(services::disable_snmpd::DisableSnmpd::new()));
    m.insert("2.2.15".into(), Box::new(services::disable_rsync::DisableRsync::new()));

    // Network controls
    m.insert("3.1.1".into(), Box::new(network::sysctl_ip_forwarding::SysctlIpForwarding::new()));
    m.insert("3.1.2".into(), Box::new(network::sysctl_disable_send_redirects::SysctlDisableSendRedirects::new()));
    m.insert("3.2.1".into(), Box::new(network::sysctl_disable_source_route::SysctlDisableSourceRoute::new()));
    m.insert("3.2.2".into(), Box::new(network::sysctl_disable_accept_redirects::SysctlDisableAcceptRedirects::new()));
    m.insert("3.2.3".into(), Box::new(network::sysctl_disable_icmp_redirects::SysctlDisableIcmpRedirects::new()));
    m.insert("3.2.6".into(), Box::new(network::sysctl_enable_rp_filter::SysctlEnableRpFilter::new()));
    m.insert("3.2.7".into(), Box::new(network::sysctl_enable_syncookies::SysctlEnableSyncookies::new()));
    m.insert("3.3.1".into(), Box::new(network::configure_firewalld::ConfigureFirewalld::new()));

    // Logging controls
    m.insert("4.1.1.1".into(), Box::new(logging::EnsureRsyslogInstalled::new()));
    m.insert("4.1.1.2".into(), Box::new(logging::EnsureRsyslogConfigured::new()));
    m.insert("4.1.1.3".into(), Box::new(logging::EnsureRsyslogPerms::new()));
    m.insert("4.1.2.1".into(), Box::new(logging::EnsureJournaldConfigured::new()));
    m.insert("4.1.2.2".into(), Box::new(logging::EnsureJournaldOverride::new()));
    m.insert("4.1.3.1".into(), Box::new(logging::EnsureAuditdInstalled::new()));
    m.insert("4.1.3.2".into(), Box::new(logging::EnsureAuditdConfigured::new()));
    m.insert("4.1.4.1".into(), Box::new(logging::EnsureAuditIdentityRules::new()));
    m.insert("4.1.4.2".into(), Box::new(logging::EnsureAuditLoginEvents::new()));
    m.insert("4.1.4.3".into(), Box::new(logging::EnsureAuditSessionEvents::new()));
    m.insert("4.1.7".into(), Box::new(logging::EnsureLogrotate::new()));

    // SSH controls
    m.insert("5.2.1".into(), Box::new(ssh::ssh_ensure_protocol_2::SshEnsureProtocol2::new()));
    m.insert("5.2.2".into(), Box::new(ssh::ssh_set_loglevel::SshSetLoglevel::new()));
    m.insert("5.2.3".into(), Box::new(ssh::ssh_disable_empty_passwords::SshDisableEmptyPasswords::new()));
    m.insert("5.2.4".into(), Box::new(ssh::disable_root_login::SshDisableRootLogin::new()));
    m.insert("5.2.4b".into(), Box::new(ssh::ssh_max_auth_tries::SshMaxAuthTries::new()));
    m.insert("5.2.6".into(), Box::new(ssh::ssh_client_alive_interval::SshClientAliveInterval::new()));
    m.insert("5.2.7".into(), Box::new(ssh::ssh_login_grace_time::SshLoginGraceTime::new()));
    m.insert("5.2.8".into(), Box::new(ssh::ssh_limit_access::SshLimitAccess::new()));
    m.insert("5.2.9".into(), Box::new(ssh::ssh_set_banner::SshSetBanner::new()));
    m.insert("5.2.10".into(), Box::new(ssh::ssh_disable_x11_forwarding::SshDisableX11Forwarding::new()));
    m.insert("5.2.11".into(), Box::new(ssh::ssh_set_crypto_policy::SshSetCryptoPolicy::new()));

    // Auth controls
    m.insert("5.1.1".into(), Box::new(auth::EnsureCronDaemon::new()));
    m.insert("5.3.1".into(), Box::new(auth::EnsurePamPasswordQuality::new()));
    m.insert("5.3.2".into(), Box::new(auth::EnsurePamMinlen::new()));
    m.insert("5.3.3".into(), Box::new(auth::EnsurePamCredit::new()));
    m.insert("5.3.4".into(), Box::new(auth::EnsurePamFaillock::new()));
    m.insert("5.3.5".into(), Box::new(auth::EnsurePamRemember::new()));
    m.insert("5.4.1".into(), Box::new(auth::EnsurePasswordHashing::new()));
    m.insert("5.5.1".into(), Box::new(auth::EnsureUmask::new()));
    m.insert("5.5.2".into(), Box::new(auth::EnsureShellTimeout::new()));

    // Maintenance controls
    m.insert("6.1.1".into(), Box::new(maintenance::ensure_perms_passwd::EnsurePermsPasswd::new()));
    m.insert("6.1.2".into(), Box::new(maintenance::EnsurePermsShadow::new()));
    m.insert("6.1.3".into(), Box::new(maintenance::EnsurePermsGroup::new()));
    m.insert("6.1.4".into(), Box::new(maintenance::EnsurePermsGshadow::new()));
    m.insert("6.1.5".into(), Box::new(maintenance::AuditWorldWritable));
    m.insert("6.1.6".into(), Box::new(maintenance::AuditSuidSgid));
    m.insert("6.1.7".into(), Box::new(maintenance::AuditUnownedFiles));
    m.insert("6.1.8".into(), Box::new(maintenance::AuditDuplicateIds));
    m.insert("6.1.9".into(), Box::new(maintenance::EnsureCronPerms::new()));

    m
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn registry_has_all_pilot_controls() {
        let reg = registry();
        assert!(reg.contains_key("1.1.1.1"));
        assert!(reg.contains_key("2.2.3"));
        assert!(reg.contains_key("3.1.1"));
        assert!(reg.contains_key("5.2.4"));
        assert!(reg.contains_key("6.1.1"));
    }

    #[test]
    fn registry_has_all_filesystem_controls() {
        let reg = registry();
        assert!(reg.contains_key("1.1.1.2"));
        assert!(reg.contains_key("1.1.1.3"));
        assert!(reg.contains_key("1.1.1.4"));
        assert!(reg.contains_key("1.1.1.5"));
        assert!(reg.contains_key("1.1.1.6"));
        assert!(reg.contains_key("1.1.1.7"));
        assert!(reg.contains_key("1.1.10"));
        assert!(reg.contains_key("1.1.2.1"));
    }

    #[test]
    fn registry_has_all_service_controls() {
        let reg = registry();
        assert!(reg.contains_key("2.2.2"));
        assert!(reg.contains_key("2.2.4"));
        assert!(reg.contains_key("2.2.5"));
        assert!(reg.contains_key("2.2.6"));
        assert!(reg.contains_key("2.2.7"));
        assert!(reg.contains_key("2.2.8"));
        assert!(reg.contains_key("2.2.9"));
        assert!(reg.contains_key("2.2.10"));
        assert!(reg.contains_key("2.2.11"));
        assert!(reg.contains_key("2.2.12"));
        assert!(reg.contains_key("2.2.14"));
        assert!(reg.contains_key("2.2.15"));
    }

    #[test]
    fn registry_has_all_network_controls() {
        let reg = registry();
        assert!(reg.contains_key("3.1.1"));
        assert!(reg.contains_key("3.1.2"));
        assert!(reg.contains_key("3.2.1"));
        assert!(reg.contains_key("3.2.2"));
        assert!(reg.contains_key("3.2.3"));
        assert!(reg.contains_key("3.2.6"));
        assert!(reg.contains_key("3.2.7"));
        assert!(reg.contains_key("3.3.1"));
    }

    #[test]
    fn registry_has_all_logging_controls() {
        let reg = registry();
        assert!(reg.contains_key("4.1.1.1"));
        assert!(reg.contains_key("4.1.1.2"));
        assert!(reg.contains_key("4.1.1.3"));
        assert!(reg.contains_key("4.1.2.1"));
        assert!(reg.contains_key("4.1.2.2"));
        assert!(reg.contains_key("4.1.3.1"));
        assert!(reg.contains_key("4.1.3.2"));
        assert!(reg.contains_key("4.1.4.1"));
        assert!(reg.contains_key("4.1.4.2"));
        assert!(reg.contains_key("4.1.4.3"));
        assert!(reg.contains_key("4.1.7"));
    }

    #[test]
    fn registry_has_all_ssh_controls() {
        let reg = registry();
        assert!(reg.contains_key("5.2.1"));
        assert!(reg.contains_key("5.2.2"));
        assert!(reg.contains_key("5.2.3"));
        assert!(reg.contains_key("5.2.4"));
        assert!(reg.contains_key("5.2.4b"));
        assert!(reg.contains_key("5.2.6"));
        assert!(reg.contains_key("5.2.7"));
        assert!(reg.contains_key("5.2.8"));
        assert!(reg.contains_key("5.2.9"));
        assert!(reg.contains_key("5.2.10"));
        assert!(reg.contains_key("5.2.11"));
    }

    #[test]
    fn registry_has_all_auth_controls() {
        let reg = registry();
        assert!(reg.contains_key("5.1.1"));
        assert!(reg.contains_key("5.3.1"));
        assert!(reg.contains_key("5.3.2"));
        assert!(reg.contains_key("5.3.3"));
        assert!(reg.contains_key("5.3.4"));
        assert!(reg.contains_key("5.3.5"));
        assert!(reg.contains_key("5.4.1"));
        assert!(reg.contains_key("5.5.1"));
        assert!(reg.contains_key("5.5.2"));
    }

    #[test]
    fn registry_has_all_maintenance_controls() {
        let reg = registry();
        assert!(reg.contains_key("6.1.1"));
        assert!(reg.contains_key("6.1.2"));
        assert!(reg.contains_key("6.1.3"));
        assert!(reg.contains_key("6.1.4"));
        assert!(reg.contains_key("6.1.5"));
        assert!(reg.contains_key("6.1.6"));
        assert!(reg.contains_key("6.1.7"));
        assert!(reg.contains_key("6.1.8"));
        assert!(reg.contains_key("6.1.9"));
    }

    #[test]
    fn registry_controls_are_clonable() {
        let reg = registry();
        for (id, ctrl) in &reg {
            let cloned = ctrl.clone_box();
            assert_eq!(cloned.id(), *id);
        }
    }

    #[test]
    fn profile_web_toml_contains_pilot_controls() {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let path = workspace_root.join("profiles").join("web.toml");
        let profile = vallumix_core::profile::Profile::from_file(&path).expect("web.toml should parse");
        assert!(profile.controls.contains(&"1.1.1.1".into()));
        assert!(profile.controls.contains(&"3.1.1".into()));
        assert!(profile.controls.contains(&"2.2.3".into()));
        assert!(profile.controls.contains(&"5.2.4".into()));
        assert!(profile.controls.contains(&"6.1.1".into()));
    }

    #[test]
    fn pilot_controls_resolve_from_registry() {
        let profile = vallumix_core::profile::Profile {
            name: "test".into(),
            description: "test".into(),
            controls: vec![
                "1.1.1.1".into(),
                "3.1.1".into(),
                "2.2.3".into(),
                "5.2.4".into(),
                "6.1.1".into(),
            ],
        };
        let reg = registry();
        let resolved = profile.resolve_controls(&reg).expect("should resolve pilot controls");
        assert_eq!(resolved.len(), 5);
    }

    #[test]
    fn pilot_controls_return_category() {
        use vallumix_core::control::Category;
        let reg = registry();
        assert_eq!(reg.get("1.1.1.1").unwrap().category(), Category::Filesystem);
        assert_eq!(reg.get("2.2.3").unwrap().category(), Category::Services);
        assert_eq!(reg.get("3.1.1").unwrap().category(), Category::Network);
        assert_eq!(reg.get("5.2.4").unwrap().category(), Category::Ssh);
        assert_eq!(reg.get("6.1.1").unwrap().category(), Category::Maintenance);
    }

    #[test]
    fn profile_web_resolves_all_controls() {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let path = workspace_root.join("profiles").join("web.toml");
        let profile = vallumix_core::profile::Profile::from_file(&path).expect("web.toml should parse");
        let reg = registry();
        let resolved = profile.resolve_controls(&reg).expect("all web controls should resolve");
        assert_eq!(resolved.len(), profile.controls.len());
        assert!(profile.controls.len() >= 20);
    }

    #[test]
    fn profile_database_resolves_all_controls() {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let path = workspace_root.join("profiles").join("database.toml");
        let profile = vallumix_core::profile::Profile::from_file(&path).expect("database.toml should parse");
        let reg = registry();
        let resolved = profile.resolve_controls(&reg).expect("all database controls should resolve");
        assert_eq!(resolved.len(), profile.controls.len());
        assert!(profile.controls.len() >= 20);
    }

    #[test]
    fn profile_bastion_resolves_all_controls() {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let path = workspace_root.join("profiles").join("bastion.toml");
        let profile = vallumix_core::profile::Profile::from_file(&path).expect("bastion.toml should parse");
        let reg = registry();
        let resolved = profile.resolve_controls(&reg).expect("all bastion controls should resolve");
        assert_eq!(resolved.len(), profile.controls.len());
        assert!(profile.controls.len() >= 25);
    }

    #[test]
    fn profile_counts_differ() {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let web = vallumix_core::profile::Profile::from_file(workspace_root.join("profiles/web.toml")).unwrap();
        let db = vallumix_core::profile::Profile::from_file(workspace_root.join("profiles/database.toml")).unwrap();
        let bastion = vallumix_core::profile::Profile::from_file(workspace_root.join("profiles/bastion.toml")).unwrap();
        assert_ne!(web.controls.len(), db.controls.len());
        assert_ne!(db.controls.len(), bastion.controls.len());
    }

    #[test]
    fn profiles_have_unique_controls() {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let web = vallumix_core::profile::Profile::from_file(workspace_root.join("profiles/web.toml")).unwrap();
        let db = vallumix_core::profile::Profile::from_file(workspace_root.join("profiles/database.toml")).unwrap();
        // Database has dhcp (2.2.4) which web does not
        assert!(db.controls.contains(&"2.2.4".into()));
        assert!(!web.controls.contains(&"2.2.4".into()));
        // Web has cups? No, web does not have cups either. Bastion does.
        let bastion = vallumix_core::profile::Profile::from_file(workspace_root.join("profiles/bastion.toml")).unwrap();
        assert!(bastion.controls.contains(&"2.2.2".into()));
        assert!(!web.controls.contains(&"2.2.2".into()));
    }
}
