# Network and Kernel Controls

The network domain (CIS section 3.1.x – 3.3.x) contains 9 controls that harden the Linux kernel's networking stack. These parameters are enforced via `sysctl` and the firewall subsystem. Misconfigured network defaults can turn a server into a traffic relay, expose it to spoofing, or make it vulnerable to connection-flood attacks.

## `3.1.1` — Disable IP forwarding

**What apply does:**
Sets `net.ipv4.ip_forward = 0` and `net.ipv6.conf.all.forwarding = 0` in `/etc/sysctl.conf` (or a drop-in under `/etc/sysctl.d/`) and applies the change with `sysctl -p`.

**Manual verification:**

```bash
sysctl net.ipv4.ip_forward
sysctl net.ipv6.conf.all.forwarding
```

Both should return `0`.

**Security justification:**
IP forwarding allows the kernel to route traffic between interfaces. Unless the server is explicitly a router or VPN gateway, enabling forwarding can turn it into an unintended traffic relay for attackers. This is one of the highest-severity network controls.

## `3.1.2` — Disable ICMP send redirects

**What apply does:**
Sets `net.ipv4.conf.all.send_redirects = 0` and `net.ipv4.conf.default.send_redirects = 0`.

**Manual verification:**

```bash
sysctl net.ipv4.conf.all.send_redirects
sysctl net.ipv4.conf.default.send_redirects
```

**Security justification:**
ICMP redirect messages are used by routers to inform hosts of better routes. A malicious actor on the local network can forge redirects to hijack traffic (man-in-the-middle). Disabling them prevents this attack vector.

## `3.2.1` — Disable source routing

**What apply does:**
Sets `net.ipv4.conf.all.accept_source_route = 0` and `net.ipv4.conf.default.accept_source_route = 0`.

**Manual verification:**

```bash
sysctl net.ipv4.conf.all.accept_source_route
sysctl net.ipv4.conf.default.accept_source_route
```

**Security justification:**
Source routing allows the sender to specify the exact network path that packets should follow. Attackers can use this to bypass firewall rules and routing policies by dictating their own path through the network.

## `3.2.2` — Disable ICMP accept redirects

**What apply does:**
Sets `net.ipv4.conf.all.accept_redirects = 0` and `net.ipv4.conf.default.accept_redirects = 0`.

**Manual verification:**

```bash
sysctl net.ipv4.conf.all.accept_redirects
sysctl net.ipv4.conf.default.accept_redirects
```

**Security justification:**
Accepting ICMP redirects allows an attacker on the same network segment to redirect the host's traffic through a compromised machine. This control complements `3.1.2` by disabling redirects in both directions.

## `3.2.3` — Disable ICMP redirect sending

**What apply does:**
Ensures `net.ipv4.conf.all.send_redirects` is `0` (overlaps with 3.1.2 but validated independently).

**Manual verification:**

```bash
sysctl net.ipv4.conf.all.send_redirects
```

**Security justification:**
Prevents the server itself from sending ICMP redirects, which could be forged or misused to influence the routing tables of peer hosts.

## `3.2.6` — Enable reverse path filtering (rp_filter)

**What apply does:**
Sets `net.ipv4.conf.all.rp_filter = 1` and `net.ipv4.conf.default.rp_filter = 1`.

**Manual verification:**

```bash
sysctl net.ipv4.conf.all.rp_filter
sysctl net.ipv4.conf.default.rp_filter
```

**Security justification:**
Reverse path filtering drops packets that arrive on an interface different from the one the kernel would use to reach the source address. This is a primary defense against IP spoofing and source-address forgery attacks.

## `3.2.7` — Enable TCP SYN cookies

**What apply does:**
Sets `net.ipv4.tcp_syncookies = 1`.

**Manual verification:**

```bash
sysctl net.ipv4.tcp_syncookies
```

**Security justification:**
SYN cookies allow the kernel to maintain TCP connections during a SYN flood attack without allocating resources for half-open connections. When the backlog fills, the kernel sends cryptographic cookies instead of keeping state, allowing legitimate clients to complete the handshake while dropping attackers.

## `3.3.1` — Ensure firewalld is configured

**What apply does:**
Verifies that `firewalld` is installed, running, and has a default deny policy. If the service is not active, Vallumix enables and starts it. It does not define specific rules; the administrator must configure ports according to the application needs.

**Manual verification:**

```bash
systemctl is-active firewalld
firewall-cmd --state
firewall-cmd --get-default-zone
```

**Security justification:**
A host-level firewall is the last line of defense against network attacks. Even if the server sits behind a perimeter firewall, local filtering protects against lateral movement and misconfigured upstream rules. firewalld's zone-based model integrates well with dynamic cloud environments.

```tip
Vallumix does not open or close specific ports because that depends on the application. After applying this control, configure the required services with `firewall-cmd --permanent --add-service=...`.
```
