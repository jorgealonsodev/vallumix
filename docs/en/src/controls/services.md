# Service Disable Controls

The services domain (CIS section 2.2.x) contains 12 controls that stop, disable, and mask network-facing services that are not required for the server's role. Every running daemon is a potential source of vulnerabilities; removing unnecessary services is one of the most effective ways to reduce the attack surface.

## `2.2.2` — Disable CUPS

**What apply does:**
Stops the `cups` service, disables it from starting at boot, and masks the unit so it cannot be started manually or by another service.

**Manual verification:**

```bash
systemctl is-enabled cups
systemctl status cups
```

Expected: `masked` or `disabled`, and the service should be inactive.

**Security justification:**
CUPS (Common UNIX Printing System) exposes network ports for printer discovery and job submission. Servers rarely need printing capability, and CUPS has a history of remotely exploitable vulnerabilities.

## `2.2.3` — Disable Avahi

**What apply does:**
Stops, disables, and masks the `avahi-daemon` service.

**Manual verification:**

```bash
systemctl is-enabled avahi-daemon
systemctl status avahi-daemon
```

**Security justification:**
Avahi implements mDNS/DNS-SD (Bonjour) for local network service discovery. It multicasts host and service information, which leaks the attack surface to anyone on the local network segment and has been the target of amplification attacks.

## `2.2.4` — Disable DHCP server

**What apply does:**
Stops, disables, and masks the DHCP server service (e.g., `isc-dhcp-server`, `dhcpd`).

**Manual verification:**

```bash
systemctl is-enabled dhcpd 2>/dev/null || systemctl is-enabled isc-dhcp-server
```

**Security justification:**
A DHCP server should only run on dedicated network infrastructure. Accidentally leaving it enabled on a server can lead to rogue DHCP attacks, network hijacking, and unauthorized device onboarding.

## `2.2.5` — Disable LDAP server

**What apply does:**
Stops, disables, and masks the LDAP server service (e.g., `slapd`).

**Manual verification:**

```bash
systemctl is-enabled slapd
```

**Security justification:**
Unless the server is explicitly an identity provider, an LDAP server exposes directory information and authentication traffic that should be isolated to dedicated directory services.

## `2.2.6` — Disable NFS server

**What apply does:**
Stops, disables, and masks NFS server services (`nfs-server`, `rpc-nfsd`).

**Manual verification:**

```bash
systemctl is-enabled nfs-server
```

**Security justification:**
NFS exports can leak sensitive data if misconfigured. The NFS protocol has historically suffered from UID-spoofing and weak authentication issues. Only dedicated file servers should run NFS.

## `2.2.7` — Disable rpcbind

**What apply does:**
Stops, disables, and masks `rpcbind`, the RPC portmapper required by NFS and other legacy RPC services.

**Manual verification:**

```bash
systemctl is-enabled rpcbind
```

**Security justification:**
`rpcbind` maps RPC program numbers to network ports. It is essential for NFSv3 but unnecessary on modern servers. It has been used in amplification DDoS attacks and provides information about running RPC services to remote scanners.

## `2.2.8` — Disable DNS server (BIND)

**What apply does:**
Stops, disables, and masks `named` (BIND) or the distribution-specific DNS server package.

**Manual verification:**

```bash
systemctl is-enabled named
```

**Security justification:**
Running a recursive or authoritative DNS server on a general-purpose server increases the attack surface and the risk of DNS cache poisoning or amplification abuse. DNS should be delegated to dedicated resolvers.

## `2.2.9` — Disable vsftpd

**What apply does:**
Stops, disables, and masks the `vsftpd` FTP server.

**Manual verification:**

```bash
systemctl is-enabled vsftpd
```

**Security justification:**
FTP transmits credentials and data in plaintext. Modern workflows should use SFTP (over SSH) or HTTPS. An accidentally enabled FTP server is a frequent source of data leaks.

## `2.2.10` — Disable HTTP server

**What apply does:**
Stops, disables, and masks the `httpd` or `apache2` service.

**Manual verification:**

```bash
systemctl is-enabled httpd 2>/dev/null || systemctl is-enabled apache2
```

**Security justification:**
This control is included in the `database` and `bastion` profiles because those roles should not serve web content. Only the `web` profile omits this control, assuming the administrator explicitly wants a web server.

## `2.2.11` — Disable Dovecot

**What apply does:**
Stops, disables, and masks the `dovecot` IMAP/POP3 mail server.

**Manual verification:**

```bash
systemctl is-enabled dovecot
```

**Security justification:**
Mail access protocols should be centralized on dedicated mail servers. Running Dovecot on a web or database server unnecessarily exposes mail credentials and storage.

## `2.2.14` — Disable SNMP daemon

**What apply does:**
Stops, disables, and masks `snmpd`.

**Manual verification:**

```bash
systemctl is-enabled snmpd
```

**Security justification:**
SNMP (especially v1 and v2c) uses community strings that are often left at default values like `public`. Even SNMPv3 requires careful key management. If monitoring is needed, use agentless metrics (Prometheus node_exporter, Telegraf) instead.

## `2.2.15` — Disable rsync daemon

**What apply does:**
Stops, disables, and masks the `rsyncd` service.

**Manual verification:**

```bash
systemctl is-enabled rsync
```

**Security justification:**
The rsync daemon exposes a TCP port (873) without encryption by default. While rsync over SSH is secure and widely used, the standalone daemon should not run unless explicitly required for a public mirror.
