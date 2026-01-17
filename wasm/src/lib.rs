use wasm_bindgen::prelude::*;
use encryptor_core::{Encryptor, create_encrypted_file, parse_encrypted_file};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn encrypt_file(password: &str, data: &[u8]) -> Result<Vec<u8>, JsError> {
    let salt = Encryptor::generate_salt();
    let encryptor = Encryptor::new_with_password(password, &salt)
        .map_err(|e| JsError::new(&e.to_string()))?;
    
    let encrypted_data = encryptor.encrypt(data)
        .map_err(|e| JsError::new(&e.to_string()))?;
        
    Ok(create_encrypted_file(&salt, &encrypted_data))
}

#[wasm_bindgen]
pub fn decrypt_file(password: &str, file_data: &[u8]) -> Result<Vec<u8>, JsError> {
    let (salt, encrypted_data) = parse_encrypted_file(file_data)
        .map_err(|e| JsError::new(&e.to_string()))?;
        
    let encryptor = Encryptor::new_with_password(password, &salt)
        .map_err(|e| JsError::new(&e.to_string()))?;
        
    let plaintext = encryptor.decrypt(&encrypted_data)
        .map_err(|e| JsError::new(&e.to_string()))?;
        
    Ok(plaintext)
}
