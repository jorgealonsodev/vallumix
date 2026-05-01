# Access, Authentication, and Authorization Controls

The access and authentication domain covers SSH hardening (CIS section 5.2.x) and system-level authentication policy (PAM, umask, shell timeout — CIS sections 5.1.x, 5.3.x – 5.5.x). With 21 controls in total across SSH and auth, this is the most critical domain for preventing unauthorized access. A single misconfigured SSH parameter can expose the server to brute force, credential stuffing, or complete remote compromise.

## SSH Controls

### `5.2.1` — Ensure SSH Protocol 2 is used

**What apply does:**
Sets `Protocol 2` in `/etc/ssh/sshd_config`. Protocol 1 is obsolete and cryptographically broken.

**Manual verification:**

```bash
grep -E '^Protocol' /etc/ssh/sshd_config
```

Expected: `Protocol 2` or the line absent (modern OpenSSH defaults to 2).

**Security justification:**
SSH Protocol 1 lacks integrity checks, uses weak CRC-32 for packet validation, and is vulnerable to insertion attacks. Protocol 2 introduces strong cryptographic integrity and key exchange.

### `5.2.2` — Set SSH LogLevel to INFO

**What apply does:**
Sets `LogLevel INFO` in `sshd_config` to ensure login attempts, key exchanges, and authentication failures are logged.

**Manual verification:**

```bash
grep '^LogLevel' /etc/ssh/sshd_config
```

**Security justification:**
`INFO` captures enough detail for intrusion detection and forensic investigation without the verbosity of `DEBUG`. `QUIET` or `FATAL` would hide brute-force patterns.

### `5.2.3` — Disable empty passwords

**What apply does:**
Sets `PermitEmptyPasswords no` in `sshd_config`.

**Manual verification:**

```bash
grep '^PermitEmptyPasswords' /etc/ssh/sshd_config
```

**Security justification:**
Empty passwords allow login without any credential. Even on internal networks, this is unacceptable because it bypasses the entire authentication layer.

### `5.2.4` — Disable root login

**What apply does:**
Sets `PermitRootLogin no` (or `prohibit-password`) in `sshd_config` and restarts the SSH service.

**Manual verification:**

```bash
grep '^PermitRootLogin' /etc/ssh/sshd_config
```

**Security justification:**
Root login is the highest-value target for brute-force attacks. Disabling it forces attackers to guess both a username and a password, doubling the search space. It also ensures all administrative actions are attributable to individual users via sudo audit logs.

```danger
This control can lock you out if you do not have key-based access as a non-root user. Verify `ssh -o PasswordAuthentication=no user@host` works before applying.
```

### `5.2.4b` — Set MaxAuthTries to 4 or less

**What apply does:**
Sets `MaxAuthTries 4` in `sshd_config`.

**Manual verification:**

```bash
grep '^MaxAuthTries' /etc/ssh/sshd_config
```

**Security justification:**
Limiting authentication attempts per connection slows down brute-force attacks. After 4 failures, the connection is dropped, forcing the attacker to establish a new TCP session.

### `5.2.6` — Set ClientAliveInterval

**What apply does:**
Sets `ClientAliveInterval 300` and `ClientAliveCountMax 0` to disconnect idle sessions after 5 minutes.

**Manual verification:**

```bash
grep -E '^ClientAlive' /etc/ssh/sshd_config
```

**Security justification:**
Idle sessions left open on shared workstations or forgotten terminal multiplexers provide an open door for anyone with physical access. Automatic disconnection limits this window.

### `5.2.8` — Limit SSH access (AllowUsers / AllowGroups)

**What apply does:**
If the administrator has configured `AllowUsers` or `AllowGroups` in the profile metadata, Vallumix writes those values to `sshd_config`. By default, it only verifies that the directive is present.

**Manual verification:**

```bash
grep -E '^AllowUsers|^AllowGroups' /etc/ssh/sshd_config
```

**Security justification:**
Restricting SSH to specific users or groups prevents disabled, terminated, or compromised accounts from accessing the server. It is especially important on bastion hosts.

### `5.2.9` — Set SSH banner

**What apply does:**
Sets `Banner /etc/ssh/banner` and creates the banner file if it does not exist.

**Manual verification:**

```bash
grep '^Banner' /etc/ssh/sshd_config
```

**Security justification:**
A login banner establishes legal notice and discourages unauthorized access. In some jurisdictions, a banner is required for successful prosecution under computer crime statutes.

## Authentication Controls (PAM)

### `5.3.1` — Ensure PAM password quality is enabled

**What apply does:**
Verifies that `pam_pwquality.so` is referenced in `/etc/pam.d/system-auth` or `/etc/pam.d/common-password`.

**Manual verification:**

```bash
grep pam_pwquality /etc/pam.d/system-auth /etc/pam.d/common-password 2>/dev/null
```

**Security justification:**
pam_pwquality enforces minimum complexity rules (length, character classes, dictionary checks) at the point of password creation or change. Without it, users can set trivial passwords that fall to dictionary attacks in seconds.

### `5.3.2` — Ensure PAM minimum password length

**What apply does:**
Sets `minlen = 14` in `/etc/security/pwquality.conf`.

**Manual verification:**

```bash
grep '^minlen' /etc/security/pwquality.conf
```

**Security justification:**
Longer passwords exponentially increase the brute-force search space. A 14-character passphrase with mixed character classes is currently considered the minimum for resistance against offline hash cracking.

### `5.3.4` — Ensure PAM faillock is configured

**What apply does:**
Adds `pam_faillock.so` lines to the auth stack with `deny=5`, `unlock_time=900`, and `even_deny_root`.

**Manual verification:**

```bash
grep pam_faillock /etc/pam.d/system-auth /etc/pam.d/password-auth 2>/dev/null
```

**Security justification:**
faillock temporarily locks accounts after repeated authentication failures. This stops automated brute-force bots without permanently disabling the account, giving administrators time to react.

### `5.3.5` — Ensure PAM password history

**What apply does:**
Sets `remember = 5` in the PAM password module configuration to prevent reuse of the last 5 passwords.

**Manual verification:**

```bash
grep 'remember' /etc/pam.d/system-auth /etc/pam.d/common-password 2>/dev/null
```

**Security justification:**
Without history enforcement, users cycle between a small set of passwords. If an old password is leaked in a breach, the attacker can often log in because the user has returned to it.

### `5.5.1` — Ensure default umask is restrictive

**What apply does:**
Sets `umask 027` in `/etc/profile`, `/etc/bashrc`, and `/etc/login.defs`.

**Manual verification:**

```bash
grep -E '^umask' /etc/profile /etc/bashrc /etc/login.defs 2>/dev/null
```

**Security justification:**
A umask of `027` ensures new files are created with `640` permissions and directories with `750`, preventing other users from reading newly created data by default.

### `5.5.2` — Ensure shell timeout is configured

**What apply does:**
Adds `TMOUT=900` and `readonly TMOUT` to `/etc/profile` and `/etc/bashrc`.

**Manual verification:**

```bash
grep 'TMOUT' /etc/profile /etc/bashrc 2>/dev/null
```

**Security justification:**
Shell timeouts automatically log out inactive local and SSH sessions after 15 minutes. This is complementary to `ClientAliveInterval` and protects against physical access or abandoned terminal windows.
