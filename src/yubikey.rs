use anyhow::{Context, Result};
use yubico_manager::{Yubico, config::{Config, Slot}};
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroize;

pub struct YubiKeyAuth {
    yubico: Yubico,
}

impl YubiKeyAuth {
    pub fn new() -> Result<Self> {
        let yubico = Yubico::new();
        Ok(Self { yubico })
    }

    /// Get challenge-response from YubiKey
    pub fn challenge_response(&mut self, challenge: &[u8]) -> Result<Vec<u8>> {
        // Challenge must be exactly 64 bytes for HMAC-SHA1
        let mut challenge_buf = [0u8; 64];
        let len = challenge.len().min(64);
        challenge_buf[..len].copy_from_slice(&challenge[..len]);

        // Create config for slot 2
        let config = Config::default()
            .set_slot(Slot::Slot2);

        // Get response from slot 2
        let response = self.yubico
            .challenge_response_hmac(&challenge_buf, config)
            .context("Failed to get YubiKey response. Is it plugged in and configured?")?;

        Ok(response.to_vec())
    }

    /// Derive additional key material from YubiKey
    pub fn derive_key_component(&mut self, salt: &[u8]) -> Result<[u8; 32]> {
        // Create 64-byte challenge from salt
        let mut challenge = [0u8; 64];
        let len = salt.len().min(64);
        challenge[..len].copy_from_slice(&salt[..len]);

        // Get response from YubiKey
        let response = self.challenge_response(&challenge)?;

        // Derive 32-byte key from response
        let hk = Hkdf::<Sha256>::new(Some(salt), &response);
        let mut key_component = [0u8; 32];
        hk.expand(b"aliaser-yubikey-v1", &mut key_component)
            .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?;

        Ok(key_component)
    }

    /// Check if YubiKey is present
    pub fn is_available() -> bool {
        let mut yubico = Yubico::new();
        let device = yubico.find_yubikey();
        device.is_ok()
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
