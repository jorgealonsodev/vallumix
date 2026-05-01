# Filesystem Hardening Controls

The filesystem domain (CIS section 1.1.x) contains 9 controls that reduce the attack surface by disabling unused filesystem kernel modules and hardening temporary mount options. Unused filesystem drivers can be exploited to load malicious media or escape containers; disabling them removes a class of kernel-level attacks.

## `1.1.1.1` — Disable cramfs

**What apply does:**
Vallumix checks whether the `cramfs` kernel module is loadable. If present, it creates or updates `/etc/modprobe.d/cramfs.conf` with `install cramfs /bin/true` and runs `rmmod cramfs` if the module is currently loaded.

**Manual verification:**

```bash
modprobe -n -v cramfs
lsmod | grep cramfs
```

Expected: the first command should show `/bin/true`, and the second should return nothing.

**Security justification:**
cramfs is a compressed read-only filesystem rarely used on modern servers. Keeping the module available allows an attacker with local access to mount crafted images that could contain malicious payloads. Disabling it follows the principle of least functionality.

## `1.1.1.2` — Disable freevxfs

**What apply does:**
Creates `/etc/modprobe.d/freevxfs.conf` with `install freevxfs /bin/true` and unloads the module if active.

**Manual verification:**

```bash
modprobe -n -v freevxfs
lsmod | grep freevxfs
```

**Security justification:**
freevxfs is the FreeBSD VxFS compatibility driver. It has no legitimate use on Linux servers and represents an unnecessary kernel entry point.

## `1.1.1.3` — Disable jffs2

**What apply does:**
Installs a modprobe blacklist for `jffs2` (Journalling Flash File System v2) and removes it from memory.

**Manual verification:**

```bash
modprobe -n -v jffs2
lsmod | grep jffs2
```

**Security justification:**
jffs2 is designed for raw flash devices such as embedded systems. On a server with block storage, this module serves no purpose and could be abused to interact with physical media.

## `1.1.1.4` — Disable hfs

**What apply does:**
Blacklists the `hfs` module (Apple Hierarchical File System) via modprobe and unloads it.

**Manual verification:**

```bash
modprobe -n -v hfs
lsmod | grep hfs
```

**Security justification:**
hfs enables mounting legacy Apple-formatted volumes. Servers do not need this capability; removing it prevents an attacker from introducing or extracting data via HFS-formatted removable media.

## `1.1.1.5` — Disable hfsplus

**What apply does:**
Blacklists the `hfsplus` module and unloads it if active.

**Manual verification:**

```bash
modprobe -n -v hfsplus
lsmod | grep hfsplus
```

**Security justification:**
hfsplus is the modern Apple filesystem driver. The same reasoning applies as for hfs: no server workload requires native Apple filesystem support.

## `1.1.1.6` — Disable squashfs

**What apply does:**
Blacklists the `squashfs` module. Note: some container runtimes use squashfs internally; verify your workload before applying this control.

**Manual verification:**

```bash
modprobe -n -v squashfs
lsmod | grep squashfs
```

**Security justification:**
squashfs is a compressed read-only filesystem used in live CDs and some container layers. If your server does not depend on snap packages or similar technologies, disabling it removes another mount vector.

```warning
Some distributions use squashfs for snap packages. Verify that `snap list` is empty or not needed before applying this control.
```

## `1.1.1.7` — Disable udf

**What apply does:**
Blacklists the `udf` module (Universal Disk Format, used by DVD-ROMs).

**Manual verification:**

```bash
modprobe -n -v udf
lsmod | grep udf
```

**Security justification:**
udf enables reading optical media. Servers without optical drives have no use for this module, and it could be exploited to mount malicious UDF images.

## `1.1.10` — Disable USB storage

**What apply does:**
Creates `/etc/modprobe.d/usb-storage.conf` with `install usb-storage /bin/true` and removes the module from memory.

**Manual verification:**

```bash
modprobe -n -v usb-storage
lsmod | grep usb-storage
```

**Security justification:**
This is one of the highest-impact filesystem controls. Disabling `usb-storage` prevents the kernel from recognizing USB mass-storage devices, blocking a common physical attack vector (e.g., malicious USB drives inserted into a server room or virtualized host with USB passthrough).

## `1.1.2.1` — Harden tmpfs mount options

**What apply does:**
Ensures that `/tmp`, `/var/tmp`, and `/dev/shm` are mounted with `noexec`, `nosuid`, and `nodev` options. If they are not already mounted via fstab or systemd mount units, Vallumix updates `/etc/fstab` or creates an override mount unit.

**Manual verification:**

```bash
findmnt -n -o OPTIONS /tmp
findmnt -n -o OPTIONS /var/tmp
findmnt -n -o OPTIONS /dev/shm
```

Each should contain `noexec`, `nosuid`, and `nodev`.

**Security justification:**
Temporary directories are frequently used by attackers to drop and execute payloads. The `noexec` flag prevents execution of binaries from these paths, `nosuid` blocks setuid escalation, and `nodev` prevents device file abuse. Together they significantly constrain post-exploitation activity.
