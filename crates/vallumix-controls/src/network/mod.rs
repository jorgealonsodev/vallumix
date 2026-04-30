pub mod common;
pub mod sysctl_ip_forwarding;

macro_rules! def_sysctl_control {
    ($name:ident, $id:expr, $desc:expr, $sev:expr, $params:expr, $dropin:expr, $content:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name(super::common::SysctlControl);

        impl $name {
            pub fn new() -> Self {
                $name(super::common::SysctlControl::new($id, $desc, $sev, $params, $dropin, $content))
            }
            pub fn with_paths(proc_prefix: std::path::PathBuf, sysctl_dir: std::path::PathBuf) -> Self {
                $name(super::common::SysctlControl::with_paths($id, $desc, $sev, $params, sysctl_dir, proc_prefix, $dropin, $content))
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

pub mod sysctl_disable_send_redirects {
    def_sysctl_control!(
        SysctlDisableSendRedirects,
        "3.1.2",
        "Ensure ICMP redirects are not accepted",
        vallumix_core::control::Severity::Medium,
        vec![
            ("net/ipv4/conf/all/send_redirects", "0"),
            ("net/ipv4/conf/default/send_redirects", "0"),
        ],
        "99-vallumix-send-redirects.conf",
        "net.ipv4.conf.all.send_redirects = 0\nnet.ipv4.conf.default.send_redirects = 0\n"
    );
}

pub mod sysctl_disable_source_route {
    def_sysctl_control!(
        SysctlDisableSourceRoute,
        "3.2.1",
        "Ensure source routed packets are not accepted",
        vallumix_core::control::Severity::Medium,
        vec![
            ("net/ipv4/conf/all/accept_source_route", "0"),
            ("net/ipv4/conf/default/accept_source_route", "0"),
        ],
        "99-vallumix-source-route.conf",
        "net.ipv4.conf.all.accept_source_route = 0\nnet.ipv4.conf.default.accept_source_route = 0\n"
    );
}

pub mod sysctl_disable_accept_redirects {
    def_sysctl_control!(
        SysctlDisableAcceptRedirects,
        "3.2.2",
        "Ensure ICMP redirects are not accepted",
        vallumix_core::control::Severity::Medium,
        vec![
            ("net/ipv4/conf/all/accept_redirects", "0"),
            ("net/ipv4/conf/default/accept_redirects", "0"),
        ],
        "99-vallumix-accept-redirects.conf",
        "net.ipv4.conf.all.accept_redirects = 0\nnet.ipv4.conf.default.accept_redirects = 0\n"
    );
}

pub mod sysctl_enable_rp_filter {
    def_sysctl_control!(
        SysctlEnableRpFilter,
        "3.2.6",
        "Ensure reverse path filtering is enabled",
        vallumix_core::control::Severity::Medium,
        vec![
            ("net/ipv4/conf/all/rp_filter", "1"),
            ("net/ipv4/conf/default/rp_filter", "1"),
        ],
        "99-vallumix-rp-filter.conf",
        "net.ipv4.conf.all.rp_filter = 1\nnet.ipv4.conf.default.rp_filter = 1\n"
    );
}

pub mod sysctl_enable_syncookies {
    def_sysctl_control!(
        SysctlEnableSyncookies,
        "3.2.7",
        "Ensure TCP SYN Cookies is enabled",
        vallumix_core::control::Severity::Medium,
        vec![
            ("net/ipv4/tcp_syncookies", "1"),
        ],
        "99-vallumix-syncookies.conf",
        "net.ipv4.tcp_syncookies = 1\n"
    );
}

pub mod sysctl_disable_icmp_redirects {
    def_sysctl_control!(
        SysctlDisableIcmpRedirects,
        "3.2.3",
        "Ensure secure ICMP redirects are not accepted",
        vallumix_core::control::Severity::Medium,
        vec![
            ("net/ipv4/conf/all/secure_redirects", "0"),
            ("net/ipv4/conf/default/secure_redirects", "0"),
        ],
        "99-vallumix-icmp-redirects.conf",
        "net.ipv4.conf.all.secure_redirects = 0\nnet.ipv4.conf.default.secure_redirects = 0\n"
    );
}

pub mod configure_firewalld;

#[cfg(test)]
mod tests {
    use super::*;
    use super::common::SysctlControl;
    use vallumix_core::control::Control;
    use vallumix_core::context::Context;
    use vallumix_core::distro::Distro;
    use std::path::PathBuf;

    fn test_ctx(dry_run: bool) -> Context {
        Context::with_paths(
            "test".into(),
            Distro::Debian12,
            "/tmp".into(),
            "/tmp".into(),
            "/tmp".into(),
            dry_run,
        )
    }

    #[test]
    fn sysctl_disable_send_redirects_check_compliant_when_both_zero() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/all")).unwrap();
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/default")).unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/all/send_redirects"), "0\n").unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/default/send_redirects"), "0\n").unwrap();

        let ctrl = sysctl_disable_send_redirects::SysctlDisableSendRedirects::with_paths(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, vallumix_core::control::CheckStatus::Compliant);
    }

    #[test]
    fn sysctl_disable_send_redirects_check_non_compliant_when_one_is_one() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/all")).unwrap();
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/default")).unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/all/send_redirects"), "1\n").unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/default/send_redirects"), "0\n").unwrap();

        let ctrl = sysctl_disable_send_redirects::SysctlDisableSendRedirects::with_paths(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, vallumix_core::control::CheckStatus::NonCompliant);
    }

    #[test]
    fn sysctl_disable_send_redirects_apply_writes_sysctl_drop_in() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");

        let ctrl = sysctl_disable_send_redirects::SysctlDisableSendRedirects::with_paths(proc_prefix, sysctl_dir.clone());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, vallumix_core::control::ApplyStatus::Applied);

        let dropin = sysctl_dir.join("99-vallumix-send-redirects.conf");
        assert!(dropin.exists());
        let content = std::fs::read_to_string(&dropin).unwrap();
        assert!(content.contains("net.ipv4.conf.all.send_redirects = 0"));
    }

    #[test]
    fn sysctl_disable_source_route_check_compliant_when_all_zero() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/all")).unwrap();
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/default")).unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/all/accept_source_route"), "0\n").unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/default/accept_source_route"), "0\n").unwrap();

        let ctrl = sysctl_disable_source_route::SysctlDisableSourceRoute::with_paths(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, vallumix_core::control::CheckStatus::Compliant);
    }

    #[test]
    fn sysctl_disable_accept_redirects_apply_writes_correct_values() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");

        let ctrl = sysctl_disable_accept_redirects::SysctlDisableAcceptRedirects::with_paths(proc_prefix, sysctl_dir.clone());
        let result = ctrl.apply(&test_ctx(false)).unwrap();
        assert_eq!(result.status, vallumix_core::control::ApplyStatus::Applied);

        let dropin = sysctl_dir.join("99-vallumix-accept-redirects.conf");
        assert!(dropin.exists());
        let content = std::fs::read_to_string(&dropin).unwrap();
        assert!(content.contains("net.ipv4.conf.all.accept_redirects = 0"));
    }

    #[test]
    fn sysctl_enable_rp_filter_check_compliant_when_rp_filter_one() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/all")).unwrap();
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4/conf/default")).unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/all/rp_filter"), "1\n").unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/conf/default/rp_filter"), "1\n").unwrap();

        let ctrl = sysctl_enable_rp_filter::SysctlEnableRpFilter::with_paths(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, vallumix_core::control::CheckStatus::Compliant);
    }

    #[test]
    fn sysctl_enable_syncookies_check_compliant_when_syncookies_one() {
        let tmpdir = tempfile::tempdir().unwrap();
        let proc_prefix = tmpdir.path().join("proc");
        let sysctl_dir = tmpdir.path().join("sysctl.d");
        std::fs::create_dir_all(&proc_prefix.join("net/ipv4")).unwrap();
        std::fs::write(&proc_prefix.join("net/ipv4/tcp_syncookies"), "1\n").unwrap();

        let ctrl = sysctl_enable_syncookies::SysctlEnableSyncookies::with_paths(proc_prefix, sysctl_dir);
        let result = ctrl.check(&test_ctx(false)).unwrap();
        assert_eq!(result.status, vallumix_core::control::CheckStatus::Compliant);
    }
}
