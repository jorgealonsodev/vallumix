# CIS Control Mapping

Vallumix implements controls aligned with the CIS Benchmarks for Linux distributions. This chapter provides a reference mapping between Vallumix controls and CIS recommendations.

## Control Categories

### 1.x Initial Setup

Controls related to filesystem configuration, software updates, and initial system setup.

### 2.x Services

Controls for configuring and securing system services, including disabling unnecessary services and configuring time synchronization.

### 3.x Network Configuration

Controls for firewall configuration, network parameters, and TCP/IP stack hardening.

### 4.x Logging and Auditing

Controls for configuring auditd, rsyslog, and ensuring appropriate logging levels for security events.

### 5.x Access, Authentication and Authorization

Controls for user account management, PAM configuration, SSH hardening, and password policies.

### 6.x System Maintenance

Controls for file permissions, system integrity checking, and ensuring only authorized software is installed.

## Control Status

Each control can report one of the following statuses during an audit:

- **Compliant**: The system meets the CIS recommendation.
- **Non-Compliant**: The system does not meet the recommendation.
- **Not Applicable**: The control does not apply to the current distribution or configuration.

## Severity Levels

Controls are classified by severity:

- **High**: Critical security issues that should be addressed immediately.
- **Medium**: Important issues that should be addressed in the near term.
- **Low**: Recommended improvements with lower immediate risk.
