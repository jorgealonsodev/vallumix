# Profiles Guide

Profiles are predefined sets of CIS controls tailored to specific server roles.

## Built-in Profiles

### Web Server Profile (`web`)

Optimized for HTTP/HTTPS servers running Nginx, Apache, or similar.

Key controls:
- Disabling unnecessary services
- Configuring firewall rules for HTTP/HTTPS
- Setting secure file permissions for web roots
- Enabling audit logging for web traffic

### Database Server Profile (`database`)

Designed for PostgreSQL, MariaDB, MongoDB, and similar database servers.

Key controls:
- Restricting network access to database ports
- Enforcing strong authentication
- Configuring audit logging for database operations
- Setting appropriate kernel parameters for database workloads

### Bastion Host Profile (`bastion`)

Hardening for jump hosts and SSH bastions.

Key controls:
- Strict SSH configuration (key-only auth, no root login)
- Session timeout and idle disconnect
- Enhanced logging and monitoring
- Minimal installed packages

## Custom Profiles

You can create custom profiles by adding TOML files to the profiles directory. See the control mapping documentation for the available control IDs and their configuration options.
