# Adding YubiKey Support to Aliaser

This guide explains how to add hardware security key (YubiKey) support for additional authentication.

## 🔑 Overview

YubiKey integration adds a second factor of authentication:
- **Something you know**: Master password
- **Something you have**: YubiKey hardware token

## 🎯 Implementation Options

### Option 1: Challenge-Response (Recommended)

Use YubiKey's HMAC-SHA1 challenge-response mode to derive an additional key component.

**Benefits:**
- Works offline (no internet required)
- No counter/server needed
- Deterministic (same challenge = same response)
- Compatible with existing vault

**How it works:**
1. Master password → Argon2id → Key A (32 bytes)
2. YubiKey challenge-response → Key B (20 bytes)
3. Combine: Final Key = HKDF(Key A || Key B)

### Option 2: FIDO2/WebAuthn

Use YubiKey's FIDO2 capabilities for authentication.

**Benefits:**
- Modern standard
- Phishing-resistant
- Can use any FIDO2 key

**Limitations:**
- More complex implementation
- Requires credential storage
- May need user interaction each time

### Option 3: Static Password

Store an additional password on the YubiKey.

**Limitations:**
- Less secure (password stored on key)
- Limited to password managers
- Not recommended for high security

## 🛠️ Implementation: Challenge-Response

### Step 1: Add Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# Existing dependencies...

# YubiKey support
yubikey = "0.8"  # YubiKey challenge-response

# Or for lower-level control:
yubico_manager = "0.6"

# Key derivation
hkdf = "0.12"
sha2 = "0.10"
```

### Step 2: Create YubiKey Module

Create `src/yubikey.rs`:

```rust
use anyhow::{Context, Result};
use yubikey::{YubiKey, configure::Slot};
use hkdf::Hkdf;
use sha2::Sha256;

pub struct YubiKeyAuth {
    slot: Slot,
}

impl YubiKeyAuth {
    pub fn new() -> Result<Self> {
        Ok(Self {
            slot: Slot::Slot2, // Use slot 2 for HMAC-SHA1
        })
    }

    /// Get challenge-response from YubiKey
    pub fn challenge_response(&self, challenge: &[u8]) -> Result<Vec<u8>> {
        // Open YubiKey
        let mut yubikey = YubiKey::open()
            .context("Failed to open YubiKey. Is it plugged in?")?;

        // Send challenge and get response
        let response = yubikey
            .challenge_response_hmac(challenge, self.slot)
            .context("Failed to get YubiKey response")?;

        Ok(response.to_vec())
    }

    /// Derive additional key material from YubiKey
    pub fn derive_key_component(&self, salt: &[u8]) -> Result<[u8; 32]> {
        // Create challenge from salt
        let challenge = if salt.len() >= 64 {
            &salt[..64]
        } else {
            // Pad if needed
            let mut padded = [0u8; 64];
            padded[..salt.len()].copy_from_slice(salt);
            &padded
        };

        // Get response from YubiKey
        let response = self.challenge_response(challenge)?;

        // Derive 32-byte key from response
        let hk = Hkdf::<Sha256>::new(Some(salt), &response);
        let mut key_component = [0u8; 32];
        hk.expand(b"aliaser-yubikey-v1", &mut key_component)
            .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?;

        Ok(key_component)
    }

    /// Check if YubiKey is present
    pub fn is_available() -> bool {
        YubiKey::open().is_ok()
    }
}

/// Combine password-derived key with YubiKey-derived key
pub fn combine_keys(password_key: &[u8; 32], yubikey_key: &[u8; 32]) -> [u8; 32] {
    let mut combined = [0u8; 64];
    combined[..32].copy_from_slice(password_key);
    combined[32..].copy_from_slice(yubikey_key);

    let hk = Hkdf::<Sha256>::new(None, &combined);
    let mut final_key = [0u8; 32];
    hk.expand(b"aliaser-combined-key-v1", &mut final_key)
        .expect("Key combination failed");

    // Zeroize intermediate keys
    combined.zeroize();

    final_key
}
```

### Step 3: Update Crypto Module

Modify `src/crypto.rs`:

```rust
use crate::yubikey::{YubiKeyAuth, combine_keys};

/// Derives encryption key with optional YubiKey
pub fn derive_key_with_yubikey(
    password: &str,
    salt: &[u8],
    use_yubikey: bool,
) -> Result<[u8; 32]> {
    // Derive key from password
    let password_key = derive_key(password, salt)?;

    if use_yubikey {
        // Get YubiKey component
        let yubikey = YubiKeyAuth::new()
            .context("Failed to initialize YubiKey")?;
        
        let yubikey_key = yubikey.derive_key_component(salt)
            .context("Failed to derive key from YubiKey")?;

        // Combine both keys
        Ok(combine_keys(&password_key, &yubikey_key))
    } else {
        Ok(password_key)
    }
}
```

### Step 4: Update Storage Module

Modify `src/storage.rs`:

```rust
/// Vault configuration with YubiKey support
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub master_password_hash: String,
    pub salt: Vec<u8>,
    pub version: String,
    pub yubikey_enabled: bool, // New field
}

impl Vault {
    /// Initialize vault with optional YubiKey
    pub fn initialize(&mut self, master_password: &str, use_yubikey: bool) -> Result<()> {
        if self.is_initialized() {
            anyhow::bail!("Vault already initialized");
        }

        // Check YubiKey if requested
        if use_yubikey && !YubiKeyAuth::is_available() {
            anyhow::bail!("YubiKey not found. Please plug it in.");
        }

        // Generate salt and hash password
        let salt = generate_salt();
        let password_hash = hash_password(master_password)?;

        // Create config
        let config = VaultConfig {
            master_password_hash: password_hash,
            salt: salt.to_vec(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            yubikey_enabled: use_yubikey,
        };

        // Save config
        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(&self.config_path, config_json)?;

        // Derive encryption key (with YubiKey if enabled)
        let key = derive_key_with_yubikey(master_password, &salt, use_yubikey)?;
        self.key = Some(key);

        // Create empty vault
        let vault_data = VaultData {
            identities: HashMap::new(),
        };
        self.save_vault_data(&vault_data)?;

        Ok(())
    }

    /// Unlock vault with optional YubiKey
    pub fn unlock(&mut self, master_password: &str) -> Result<()> {
        if !self.is_initialized() {
            anyhow::bail!("Vault not initialized. Run 'init' first.");
        }

        // Load config
        let config = self.load_config()?;

        // Verify password
        if !verify_password(master_password, &config.master_password_hash)? {
            anyhow::bail!("Invalid master password");
        }

        // Check YubiKey if enabled
        if config.yubikey_enabled && !YubiKeyAuth::is_available() {
            anyhow::bail!("YubiKey required but not found. Please plug it in.");
        }

        // Derive key (with YubiKey if enabled)
        let key = derive_key_with_yubikey(
            master_password,
            &config.salt,
            config.yubikey_enabled,
        )?;
        self.key = Some(key);

        Ok(())
    }
}
```

### Step 5: Update CLI

Modify `src/cli.rs`:

```rust
pub fn init() -> Result<()> {
    let mut vault = Vault::new()?;

    if vault.is_initialized() {
        println!("{}", "Vault already initialized!".yellow());
        return Ok(());
    }

    println!("{}", "Initializing new vault...".cyan().bold());
    println!();

    // Ask about YubiKey
    let use_yubikey = if YubiKeyAuth::is_available() {
        println!("{}", "YubiKey detected!".green());
        prompt_yes_no("Enable YubiKey authentication? (y/n): ")?
    } else {
        println!("{}", "No YubiKey detected (optional)".dim());
        false
    };

    let master_password = prompt_new_password("Enter master password: ")?;

    if use_yubikey {
        println!();
        println!("{}", "Please touch your YubiKey...".cyan());
    }

    vault.initialize(&master_password, use_yubikey)?;

    println!();
    println!("{}", "✓ Vault initialized successfully!".green().bold());
    
    if use_yubikey {
        println!(
            "{}",
            "⚠ YubiKey required: Keep your YubiKey safe!".yellow()
        );
        println!(
            "{}",
            "  You'll need both your password AND YubiKey to unlock.".yellow()
        );
    }

    Ok(())
}
```

### Step 6: Update Main Module

Add to `src/main.rs`:

```rust
mod yubikey;
```

## 🔧 YubiKey Configuration

### Configuring Your YubiKey

Before using with Aliaser, configure slot 2 for HMAC-SHA1:

```bash
# Install YubiKey Manager
# Linux:
sudo apt install yubikey-manager

# macOS:
brew install ykman

# Windows: Download from yubico.com

# Configure slot 2 for HMAC-SHA1 challenge-response
ykman otp chalresp --touch --generate 2

# The --touch flag requires physical touch for each use (recommended)
```

## 📋 Usage Examples

### Initialize with YubiKey

```bash
aliaser init
# YubiKey detected!
# Enable YubiKey authentication? (y/n): y
# Enter master password: ********
# Please touch your YubiKey...
# ✓ Vault initialized successfully!
```

### Unlock with YubiKey

```bash
aliaser list
# Master password: ********
# Please touch your YubiKey...
# [Lists identities]
```

### Without YubiKey

If YubiKey is not available:
```bash
aliaser list
# Master password: ********
# Error: YubiKey required but not found. Please plug it in.
```

## 🔒 Security Considerations

### Advantages

✅ **Two-factor security**: Requires both password and hardware key  
✅ **Offline operation**: Works without internet  
✅ **Phishing resistant**: Hardware key cannot be copied  
✅ **Portable**: Works across devices with the same YubiKey  
✅ **Deterministic**: Same inputs always give same outputs  

### Limitations

⚠️ **Physical requirement**: Must have YubiKey plugged in  
⚠️ **Single point of failure**: Lose YubiKey = lose access  
⚠️ **No backup**: Cannot access vault without the specific YubiKey  
⚠️ **Device-specific**: YubiKey configuration is per-device  

### Best Practices

1. **Backup YubiKey**: Configure a second YubiKey identically
2. **Export backups**: Regularly export vault before YubiKey changes
3. **Test access**: Verify you can unlock with YubiKey after setup
4. **Secure storage**: Keep backup YubiKey in safe location
5. **Document setup**: Write down YubiKey configuration steps

## 🆘 Recovery Options

### Option 1: Backup YubiKey

Configure multiple YubiKeys with identical secrets:

```bash
# Program second YubiKey with same secret as first
ykman otp chalresp --generate 2
# Use same secret as the first key
```

### Option 2: Emergency Backup

Create a password-only vault backup:

```bash
# Before enabling YubiKey, export vault
aliaser export backup-no-yubikey.vault

# This backup can be imported to a non-YubiKey vault
```

### Option 3: Disable YubiKey (Advanced)

Add a command to disable YubiKey requirement:

```bash
aliaser disable-yubikey
# Requires both master password and YubiKey
# Converts vault to password-only
```

## 🧪 Testing

### Test YubiKey Detection

```rust
#[test]
fn test_yubikey_detection() {
    match YubiKeyAuth::is_available() {
        true => println!("YubiKey detected"),
        false => println!("No YubiKey found"),
    }
}
```

### Test Challenge-Response

```rust
#[test]
fn test_challenge_response() {
    if !YubiKeyAuth::is_available() {
        return; // Skip if no YubiKey
    }

    let yubikey = YubiKeyAuth::new().unwrap();
    let challenge = b"test challenge";
    let response1 = yubikey.challenge_response(challenge).unwrap();
    let response2 = yubikey.challenge_response(challenge).unwrap();
    
    // Same challenge should give same response
    assert_eq!(response1, response2);
}
```

## 📚 Additional Resources

- [YubiKey Developer Guide](https://developers.yubico.com/)
- [Challenge-Response Documentation](https://developers.yubico.com/OTP/OTPs_Explained.html)
- [yubikey-rs Crate](https://docs.rs/yubikey/)
- [FIDO2 Specification](https://fidoalliance.org/specifications/)

## 🎯 Future Enhancements

- [ ] Support multiple YubiKeys
- [ ] FIDO2/WebAuthn support
- [ ] U2F support
- [ ] NFC support (for NFC-enabled YubiKeys)
- [ ] Biometric support (YubiKey Bio)
- [ ] Backup/restore YubiKey configuration
- [ ] Smart card (PIV) support

## ⚠️ Important Notes

1. **YubiKey configuration is permanent**: Once programmed, slot configuration cannot be easily changed
2. **Touch requirement recommended**: Use `--touch` flag for physical confirmation
3. **Test before committing**: Ensure YubiKey works before migrating all data
4. **Keep backup**: Always maintain an unencrypted export as emergency backup
5. **Cross-platform**: YubiKey works on Linux, macOS, and Windows

---

**Ready to implement?** Start with the dependencies and test YubiKey detection first!
