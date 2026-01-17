use anyhow::{Result, anyhow};

/// Create encrypted file format with metadata
pub fn create_encrypted_file(
    salt: &[u8],
    encrypted_data: &[u8],
) -> Vec<u8> {
    let mut result = Vec::new();
    
    // Magic bytes to identify our encrypted files
    result.extend_from_slice(b"ENCR");
    
    // Version byte
    result.push(1);
    
    // Salt (32 bytes)
    result.extend_from_slice(salt);
    
    // Encrypted data
    result.extend_from_slice(encrypted_data);
    
    result
}

/// Parse encrypted file format
pub fn parse_encrypted_file(data: &[u8]) -> Result<([u8; 32], Vec<u8>)> {
    if data.len() < 37 {  // 4 (magic) + 1 (version) + 32 (salt)
        return Err(anyhow!("Invalid encrypted file: too short"));
    }
    
    // Check magic bytes
    if &data[0..4] != b"ENCR" {
        return Err(anyhow!("Invalid encrypted file: wrong magic bytes"));
    }
    
    // Check version
    if data[4] != 1 {
        return Err(anyhow!("Unsupported file version"));
    }
    
    // Extract salt
    let mut salt = [0u8; 32];
    salt.copy_from_slice(&data[5..37]);
    
    // Extract encrypted data
    let encrypted_data = data[37..].to_vec();
    
    Ok((salt, encrypted_data))
}
