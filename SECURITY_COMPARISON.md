# Why Aliaser is Secure

## Security Comparison

### Aliaser vs Other Password Managers

| Feature | Aliaser | Cloud-based Managers | Browser Built-in |
|---------|---------|---------------------|------------------|
| **Encryption** | AES-256-GCM | AES-256 (varies) | AES-256 (varies) |
| **Key Derivation** | Argon2id | PBKDF2/Argon2 | PBKDF2 |
| **Data Storage** | Local only | Cloud servers | Local + Sync |
| **Zero Knowledge** | ✅ Yes | ✅ Yes (some) | ❌ No |
| **Open Source** | ✅ Yes | ❌ No (most) | ⚠️ Partial |
| **Network Access** | ❌ None | ✅ Required | ✅ Required |
| **Audit Trail** | Full source code | Limited/NDA | Partial |
| **Telemetry** | ❌ None | ✅ Yes (most) | ✅ Yes |
| **Data Breaches Risk** | Local only | Cloud breach risk | Sync breach risk |
| **Master Password Recovery** | ❌ No | ⚠️ Sometimes | ✅ Yes |

## Security Architecture

### 1. Encryption Layer

**Algorithm: AES-256-GCM**
- **AES-256**: Advanced Encryption Standard with 256-bit keys
  - US Government approved for TOP SECRET data
  - 2^256 possible keys (unbreakable by brute force)
  - No known practical attacks
  
- **GCM Mode**: Galois/Counter Mode
  - Authenticated Encryption with Associated Data (AEAD)
  - Provides both confidentiality AND integrity
  - Detects any tampering with encrypted data
  - Each encryption uses unique random nonce

**Why it matters:**
- Even if someone steals your vault file, they cannot decrypt it
- Any tampering with the file is immediately detected
- No known vulnerabilities in this encryption scheme

### 2. Key Derivation Function

**Algorithm: Argon2id**
- **Winner** of Password Hashing Competition (2015)
- **Resistant to:**
  - GPU attacks
  - ASIC attacks
  - Side-channel attacks
  - Time-memory trade-off attacks

**Parameters:**
- Memory-hard function
- Configurable time and memory costs
- Random salt per vault (256 bits)
- Output: 256-bit encryption key

**Why it matters:**
- Attackers cannot use powerful GPUs to crack passwords quickly
- Each password guess takes significant time and memory
- Even weak passwords get strong protection
- Rainbow tables are useless (random salt)

### 3. Master Password Handling

**Security measures:**
1. **Never Stored**: Only hash stored, not the password
2. **Separate Hashing**: 
   - One hash for verification (Argon2id)
   - Different derivation for encryption key
3. **Memory Safety**: Password zeroized after use
4. **Confirmation Required**: Prevents typos during setup

**Why it matters:**
- No password in config file to steal
- Separate hashes prevent key extraction
- Memory dumps won't reveal password
- User error protection

### 4. Data Protection at Rest

**Vault file structure:**
```
[Nonce (12 bytes)][Encrypted Data][Auth Tag (16 bytes)]
```

**What's encrypted:**
- All usernames and passwords
- Personal information (birthdate, address, etc.)
- Custom fields
- Notes
- Service names
- Timestamps

**What's NOT encrypted (config file):**
- Master password hash (needed for verification)
- Salt (needed for key derivation)
- Version number

**Why it matters:**
- Even metadata is protected
- Config file alone reveals nothing useful
- Both files needed to access vault

### 5. Memory Safety

**Rust Language Benefits:**
- No buffer overflows
- No use-after-free bugs
- No null pointer dereferences
- Memory safety guaranteed at compile time

**Zeroization:**
```rust
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    // ...
}
```

**Why it matters:**
- Traditional security issues prevented by language
- Sensitive data cleared from RAM when done
- Memory dumps are less useful to attackers
- No memory leaks of credentials

### 6. No Network Attack Surface

**Features:**
- Zero network code
- No HTTP/HTTPS requests
- No DNS queries
- No telemetry or analytics
- No auto-updates over network

**Why it matters:**
- Cannot be phished (no server to fake)
- Cannot be MitM attacked (no connections)
- No server breaches (no server)
- No tracking or profiling
- Air-gapped security possible

## Attack Scenarios & Defenses

### Scenario 1: Attacker Steals Vault File

**Attack:** Physical access to `~/.aliaser.vault`

**Defense:**
- File is encrypted with AES-256-GCM
- Cannot decrypt without master password
- Cannot modify without detection (authentication tag)

**Result:** ❌ Attack fails (need master password)

---

### Scenario 2: Attacker Steals Both Vault and Config

**Attack:** Both `.aliaser.vault` and `.aliaser.config` stolen

**Defense:**
- Config contains only hash and salt
- Must brute force master password
- Argon2id makes each guess expensive
- Strong passwords = years to crack

**Result:** ⚠️ Depends on master password strength
- Weak password: vulnerable in days/weeks
- Strong password: effectively impossible

---

### Scenario 3: Keylogger on System

**Attack:** Malware records master password when typed

**Defense:**
- No defense at application level
- Requires clean, trusted system

**Result:** ❌ Attack succeeds (OS compromise)

**Mitigation:** Use full disk encryption, anti-malware, trusted boot

---

### Scenario 4: Memory Dump Attack

**Attack:** Attacker dumps RAM while vault is unlocked

**Defense:**
- Zeroization of sensitive data structures
- Encryption key cleared when no longer needed
- Limited time window for attack

**Result:** ⚠️ Partially successful
- Keys cleared quickly after use
- Some data may be in memory temporarily
- Much harder than persistent storage attack

---

### Scenario 5: Rubber Hose Cryptanalysis

**Attack:** Physical coercion to reveal password

**Defense:**
- No plausible deniability features
- No hidden volumes
- No technical defense

**Result:** ✅ Attack succeeds (by design)

**Note:** This is intentional - security should not encourage users to resist physical threats

---

### Scenario 6: Quantum Computer Attack

**Attack:** Future quantum computer breaks AES-256

**Defense:**
- AES-256 requires ~2^128 quantum operations (Grover's algorithm)
- Still considered secure against quantum computers
- Post-quantum migration path exists

**Result:** ✅ Currently secure
- Future: May need algorithm update
- Plenty of warning time before quantum threat

---

### Scenario 7: Side-Channel Attack

**Attack:** Timing attacks, power analysis, EM radiation

**Defense:**
- Argon2id designed to resist timing attacks
- AES-GCM implementations use constant-time operations
- Rust prevents many timing side-channels

**Result:** ⚠️ Difficult but theoretically possible
- Requires physical proximity
- Requires sophisticated equipment
- Much harder than other attacks

---

## Security Best Practices for Users

### 1. Master Password Strength

**Minimum:** 8 characters (enforced)
**Recommended:** 16+ characters

**Strong password examples:**
- ✅ `correct-horse-battery-staple-2024`
- ✅ `MyC@t!Loves#Tuna$2024`
- ✅ `Tr0pic@l*Sunse7!Ocean`

**Weak passwords to avoid:**
- ❌ `password123`
- ❌ `qwerty`
- ❌ `12345678`

### 2. System Security

**Essential:**
- ✅ Full disk encryption (LUKS/FileVault/BitLocker)
- ✅ Lock screen when away
- ✅ Anti-malware software
- ✅ OS security updates

**Recommended:**
- ✅ Secure boot enabled
- ✅ BIOS/UEFI password
- ✅ Firewall enabled
- ✅ Minimal installed software

### 3. Backup Strategy

**Backup vault regularly:**
```bash
aliaser export ~/backups/vault-$(date +%Y-%m-%d).vault
```

**Backup storage:**
- ✅ Encrypted external drive
- ✅ Encrypted cloud storage (Tresorit, Sync.com)
- ✅ Safe deposit box
- ❌ Unencrypted USB drive
- ❌ Email to yourself
- ❌ Public cloud (Dropbox, Google Drive) without encryption

### 4. Physical Security

- Lock your computer when away
- Don't leave vault file on shared computers
- Securely erase when decommissioning drives
- Keep backups physically secure

### 5. Operational Security

- Don't share master password
- Don't write down master password insecurely
- Don't type master password on untrusted computers
- Don't store vault in shared folders
- Clear terminal history after use

## Comparing to Specific Products

### vs LastPass
- **Advantage:** No cloud, no breaches, open source
- **Disadvantage:** No sync, no mobile app

### vs 1Password
- **Advantage:** No subscription, open source, local only
- **Disadvantage:** No team features, no UI

### vs Bitwarden
- **Advantage:** No network code, simpler architecture
- **Disadvantage:** No browser integration, no sync

### vs KeePass
- **Similar:** Both local, encrypted, open source
- **Advantage:** Modern Rust implementation, simpler
- **Disadvantage:** Less mature, fewer features

### vs Browser Built-in
- **Advantage:** More secure, no sync vulnerabilities
- **Disadvantage:** Not integrated, less convenient

## When NOT to Use Aliaser

❌ **Need cloud sync across devices**
- Use: Bitwarden (self-hosted) or 1Password

❌ **Need browser integration**
- Use: Bitwarden or browser extension

❌ **Need team/family sharing**
- Use: 1Password Teams or Bitwarden Organizations

❌ **Need mobile access**
- Use: Full-featured password manager with apps

❌ **Need GUI interface**
- Wait for future Aliaser GUI or use KeePassXC

## When TO Use Aliaser

✅ **Maximum security is priority**
✅ **Trust no cloud providers**
✅ **Want open source and auditable**
✅ **Comfortable with command line**
✅ **Single device/manual sync acceptable**
✅ **Want minimal attack surface**
✅ **Need identity information storage**
✅ **Value privacy above convenience**

## Conclusion

Aliaser provides **military-grade security** for users who:
- Prioritize security over convenience
- Distrust cloud storage
- Want auditable open source
- Need local-only storage
- Are comfortable with CLI tools

**Security Rating:** ⭐⭐⭐⭐⭐ (5/5)
- Excellent encryption (AES-256-GCM)
- Strong key derivation (Argon2id)
- Zero network attack surface
- Open source for audit
- Memory-safe implementation

**Convenience Rating:** ⭐⭐⭐☆☆ (3/5)
- CLI only (no GUI)
- Manual sync required
- No browser integration
- No mobile apps

**Best for:** Security professionals, privacy advocates, paranoid users, air-gapped systems

---

*Remember: The most secure password manager is worthless if you use weak passwords or run on compromised systems. Security is a holistic process!* 🔐
