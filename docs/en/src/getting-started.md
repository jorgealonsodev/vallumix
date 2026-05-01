# Getting Started

Vallumix is designed to make Linux server hardening straightforward and reproducible. Whether you manage a single web server or a fleet of database instances, Vallumix provides the tools you need to enforce security baselines.

## What is Vallumix?

Vallumix is a command-line tool that applies CIS Benchmark controls to your Linux systems. It supports multiple distributions and provides rollback capabilities, making it safe to experiment with different hardening configurations.

## Key Concepts

- **Profiles**: Predefined sets of controls tailored to specific server roles (web, database, bastion).
- **Controls**: Individual security checks and remediation actions aligned with CIS Benchmarks.
- **Sessions**: A snapshot of applied changes that can be rolled back if needed.
- **Reports**: Detailed compliance output in HTML, JSON, JUnit XML, or plain text.

## Before You Begin

Ensure you have root or sudo access on the target system, as many hardening controls require administrative privileges to modify system configuration files.
