use crate::crypto::{decrypt, derive_key, derive_key_with_yubikey, encrypt, generate_salt, hash_password, verify_password};
use crate::yubikey::YubiKeyAuth;
use crate::identity::Identity;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const VAULT_FILE: &str = ".aliaser.vault";
const CONFIG_FILE: &str = ".aliaser.config";

/// Vault configuration with YubiKey support
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub master_password_hash: String,
    pub salt: Vec<u8>,
    pub version: String,
    pub yubikey_enabled: bool,
}

/// Encrypted vault data
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultData {
    pub identities: HashMap<String, Identity>,
}

pub struct Vault {
    vault_path: PathBuf,
    config_path: PathBuf,
    key: Option<[u8; 32]>,
}

impl Vault {
    /// Creates a new vault instance
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().context("Failed to get home directory")?;
        let vault_path = home.join(VAULT_FILE);
        let config_path = home.join(CONFIG_FILE);

        Ok(Self {
            vault_path,
            config_path,
            key: None,
        })
    }

    /// Checks if vault is initialized
    pub fn is_initialized(&self) -> bool {
        self.config_path.exists() && self.vault_path.exists()
    }

    /// Initializes a new vault with a master password
    pub fn initialize(&mut self, master_password: &str, use_yubikey: bool) -> Result<()> {
        if self.is_initialized() {
            anyhow::bail!("Vault already initialized");
        }

        // check YubiKey if requested
        if use_yubikey && !YubiKeyAuth::is_available() {
            anyhow::bail!("YubiKey not found. Please plug it in");
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

    /// Unlocks the vault with the master password or optional YubiKey
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

        // check YubiKey if enabled
        if config.yubikey_enabled && !YubiKeyAuth::is_available() {
            anyhow::bail!("YubiKey required but not found. Please plug it in");
        }


        // Derive key
        let key = derive_key_with_yubikey(master_password, &config.salt, config.yubikey_enabled)?;
        self.key = Some(key);

        Ok(())
    }

    /// Adds a new identity to the vault
    pub fn add_identity(&self, identity: Identity) -> Result<()> {
        let mut data = self.load_vault_data()?;

        if data.identities.contains_key(&identity.service) {
            anyhow::bail!("Identity for service '{}' already exists", identity.service);
        }

        data.identities.insert(identity.service.clone(), identity);
        self.save_vault_data(&data)?;

        Ok(())
    }

    /// Gets an identity by service name
    pub fn get_identity(&self, service: &str) -> Result<Identity> {
        let data = self.load_vault_data()?;
        data.identities
            .get(service)
            .cloned()
            .context(format!("Identity for service '{}' not found", service))
    }

    /// Lists all service names
    pub fn list_services(&self) -> Result<Vec<String>> {
        let data = self.load_vault_data()?;
        let mut services: Vec<String> = data.identities.keys().cloned().collect();
        services.sort();
        Ok(services)
    }

    /// Updates an existing identity
    pub fn update_identity(&self, service: &str, mut identity: Identity) -> Result<()> {
        let mut data = self.load_vault_data()?;

        if !data.identities.contains_key(service) {
            anyhow::bail!("Identity for service '{}' not found", service);
        }

        identity.update_timestamp();
        data.identities.insert(service.to_string(), identity);
        self.save_vault_data(&data)?;

        Ok(())
    }

    /// Deletes an identity
    pub fn delete_identity(&self, service: &str) -> Result<()> {
        let mut data = self.load_vault_data()?;

        if data.identities.remove(service).is_none() {
            anyhow::bail!("Identity for service '{}' not found", service);
        }

        self.save_vault_data(&data)?;
        Ok(())
    }

    /// Changes the master password
    pub fn change_master_password(&mut self, old_password: &str, new_password: &str) -> Result<()> {
        // Verify old password and load data
        self.unlock(old_password)?;
        let data = self.load_vault_data()?;

        // Generate new salt and hash
        let new_salt = generate_salt();
        let new_hash = hash_password(new_password)?;

        // Derive new key
        let new_key = derive_key(new_password, &new_salt)?;

        // Update config
        let config = VaultConfig {
            master_password_hash: new_hash,
            salt: new_salt.to_vec(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            yubikey_enabled: false, // TODO: Preserve yubikey setting when changing password
        };

        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(&self.config_path, config_json)?;

        // Re-encrypt vault with new key
        self.key = Some(new_key);
        self.save_vault_data(&data)?;

        Ok(())
    }

    /// Exports vault data to a file (encrypted)
    pub fn export(&self, path: &Path) -> Result<()> {
        let encrypted_data = fs::read(&self.vault_path)?;
        fs::write(path, encrypted_data)?;
        Ok(())
    }

    /// Imports vault data from a file
    pub fn import(&self, path: &Path) -> Result<()> {
        let encrypted_data = fs::read(path)?;
        
        // Verify it can be decrypted
        let key = self.key.as_ref().context("Vault not unlocked")?;
        let decrypted = decrypt(&encrypted_data, key)?;
        let _: VaultData = serde_json::from_slice(&decrypted)?;

        // Save to vault
        fs::write(&self.vault_path, encrypted_data)?;
        Ok(())
    }

    // Private helper methods

    fn load_config(&self) -> Result<VaultConfig> {
        let config_json = fs::read_to_string(&self.config_path)
            .context("Failed to read vault config")?;
        let config: VaultConfig = serde_json::from_str(&config_json)
            .context("Failed to parse vault config")?;
        Ok(config)
    }

    fn load_vault_data(&self) -> Result<VaultData> {
        let key = self.key.as_ref().context("Vault not unlocked")?;

        let encrypted_data = fs::read(&self.vault_path)
            .context("Failed to read vault file")?;

        let decrypted = decrypt(&encrypted_data, key)
            .context("Failed to decrypt vault")?;

        let vault_data: VaultData = serde_json::from_slice(&decrypted)
            .context("Failed to parse vault data")?;

        Ok(vault_data)
    }

    fn save_vault_data(&self, data: &VaultData) -> Result<()> {
        let key = self.key.as_ref().context("Vault not unlocked")?;

        let json = serde_json::to_string(data)
            .context("Failed to serialize vault data")?;

        let encrypted = encrypt(json.as_bytes(), key)
            .context("Failed to encrypt vault data")?;

        fs::write(&self.vault_path, encrypted)
            .context("Failed to write vault file")?;

        Ok(())
    }
}
