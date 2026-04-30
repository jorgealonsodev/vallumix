pub mod common;
pub mod disable_root_login;
pub mod ssh_limit_access;
pub mod ssh_set_banner;

macro_rules! def_sshd_control {
    ($name:ident, $id:expr, $desc:expr, $sev:expr, $directive:expr, $expected:expr, $apply:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name(super::common::SshdConfigControl);

        impl $name {
            pub fn new() -> Self {
                $name(super::common::SshdConfigControl::new($id, $desc, $sev, $directive, $expected, $apply))
            }
            pub fn with_path(sshd_config_path: std::path::PathBuf) -> Self {
                $name(super::common::SshdConfigControl::with_path($id, $desc, $sev, sshd_config_path, $directive, $expected, $apply))
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

pub mod ssh_ensure_protocol_2 {
    def_sshd_control!(SshEnsureProtocol2, "5.2.1", "Ensure SSH Protocol is set to 2", vallumix_core::control::Severity::Medium, "Protocol", vec!["2"], "2");
}
pub mod ssh_set_loglevel {
    def_sshd_control!(SshSetLoglevel, "5.2.2", "Ensure SSH LogLevel is appropriate", vallumix_core::control::Severity::Medium, "LogLevel", vec!["INFO", "VERBOSE"], "INFO");
}
pub mod ssh_disable_empty_passwords {
    def_sshd_control!(SshDisableEmptyPasswords, "5.2.3", "Ensure SSH empty passwords are disabled", vallumix_core::control::Severity::High, "PermitEmptyPasswords", vec!["no"], "no");
}
pub mod ssh_max_auth_tries {
    def_sshd_control!(SshMaxAuthTries, "5.2.4b", "Ensure SSH MaxAuthTries is set to 4 or less", vallumix_core::control::Severity::Medium, "MaxAuthTries", vec!["1", "2", "3", "4"], "4");
}
pub mod ssh_client_alive_interval {
    def_sshd_control!(SshClientAliveInterval, "5.2.6", "Ensure SSH Idle Timeout Interval is configured", vallumix_core::control::Severity::Medium, "ClientAliveInterval", vec!["300"], "300");
}
pub mod ssh_login_grace_time {
    def_sshd_control!(SshLoginGraceTime, "5.2.7", "Ensure SSH LoginGraceTime is set to one minute or less", vallumix_core::control::Severity::Medium, "LoginGraceTime", vec!["60", "1m"], "60");
}
pub mod ssh_disable_x11_forwarding {
    def_sshd_control!(SshDisableX11Forwarding, "5.2.10", "Ensure SSH X11 forwarding is disabled", vallumix_core::control::Severity::Medium, "X11Forwarding", vec!["no"], "no");
}
pub mod ssh_set_crypto_policy {
    def_sshd_control!(SshSetCryptoPolicy, "5.2.11", "Ensure SSH crypto policy is configured", vallumix_core::control::Severity::Medium, "Ciphers", vec![], "aes256-gcm@openssh.com,chacha20-poly1305@openssh.com");
}
