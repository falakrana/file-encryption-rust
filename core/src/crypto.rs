use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use argon2::Argon2;
use anyhow::{Result, anyhow};
use rand::RngCore;

pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    /// Derive a key from password using Argon2
    pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
        use argon2::{Algorithm, Params, Version};
        
        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| anyhow!("Failed to create Argon2 parameters: {:?}", e))?;
        
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            params
        );
        
        let mut key = [0u8; 32];
        argon2.hash_password_into(
            password.as_bytes(),
            salt,
            &mut key
        )
        .map_err(|e| anyhow!("Failed to derive key from password: {:?}", e))?;
        
        Ok(key)
    }
    
    /// Create a new encryptor with a password
    pub fn new_with_password(password: &str, salt: &[u8]) -> Result<Self> {
        let key = Self::derive_key_from_password(password, salt)?;
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        
        Ok(Self { cipher })
    }
    
    /// Encrypt data
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self.cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }
    
    /// Decrypt data
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(anyhow!("Invalid ciphertext: too short"));
        }
        
        // Extract nonce and actual ciphertext
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        
        Ok(plaintext)
    }
    
    /// Generate a random salt
    pub fn generate_salt() -> [u8; 32] {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        salt
    }
}
