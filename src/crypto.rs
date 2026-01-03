use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use rand::RngCore;
use anyhow::{Result, Context};
use crate::yubikey::{YubiKeyAuth, combine_keys};

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 32;

/// Derives a 256-bit key from a password using Argon2id
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let argon2 = Argon2::default();
    let mut output_key = [0u8; 32];
    
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut output_key)
        .map_err(|e| anyhow::anyhow!("Failed to derive key from password: {}", e))?;
    
    Ok(output_key)
}

/// Derives encryption key with optional YubiKey
pub fn derive_key_with_yubikey(
    password: &str,
    salt: &[u8],
    use_yubikey: bool,
) -> Result<[u8; 32]> {
    // derive the key from password
    let password_key = derive_key(password, salt)?;

    if use_yubikey {
        // get yubikey component
        let mut yubikey = YubiKeyAuth::new()
            .context("Failed to initialize Yubikey")?;

        let yubikey_key = yubikey.derive_key_component(salt)
            .context("Failed to derive key from YubiKey")?;

        // combine both keys
        Ok(combine_keys(&password_key, &yubikey_key))
    } else {
        Ok(password_key)
    }
}


/// Generates a random salt for key derivation
pub fn generate_salt() -> [u8; SALT_SIZE] {
    let mut salt = [0u8; SALT_SIZE];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Encrypts data using AES-256-GCM
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());
    
    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
    
    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Decrypts data using AES-256-GCM
pub fn decrypt(encrypted_data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
    if encrypted_data.len() < NONCE_SIZE {
        anyhow::bail!("Invalid encrypted data: too short");
    }
    
    let cipher = Aes256Gcm::new(key.into());
    
    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
    
    Ok(plaintext)
}

/// Hashes a password for verification (not for encryption key derivation)
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?
        .to_string();
    
    Ok(password_hash)
}

/// Verifies a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = [0u8; 32];
        let data = b"Hello, World!";
        
        let encrypted = encrypt(data, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        
        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_password_hashing() {
        let password = "super_secret_password";
        let hash = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_yubikey_detection() {
        match YubiKeyAuth::is_available() {
            true => println!("YubiKey detected"),
            false => println!("No YubiKey found"),
        }
    }

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

}
