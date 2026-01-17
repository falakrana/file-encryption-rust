use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use anyhow::{Result, Context};

const PROGRESS_CHUNK_SIZE: usize = 64 * 1024; // 64 KB

pub struct FileHandler;

impl FileHandler {
    /// Read entire file into memory
    pub fn read_file(path: &Path) -> Result<Vec<u8>> {
        let mut file = File::open(path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        
        Ok(buffer)
    }
    
    /// Read file in chunks, calling on_progress(bytes_read, total) for progress reporting
    pub fn read_file_with_progress<F>(path: &Path, mut on_progress: F) -> Result<Vec<u8>>
    where
        F: FnMut(u64, u64),
    {
        let total = std::fs::metadata(path)
            .with_context(|| format!("Failed to get file size: {}", path.display()))?
            .len();
        
        let mut file = File::open(path)
            .with_context(|| format!("Failed to open file: {}", path.display()))?;
        
        let mut buffer = Vec::with_capacity(total as usize);
        let mut read = 0u64;
        let mut chunk = vec![0u8; PROGRESS_CHUNK_SIZE];
        
        loop {
            let n = file
                .read(&mut chunk)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;
            if n == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..n]);
            read += n as u64;
            on_progress(read, total);
        }
        
        Ok(buffer)
    }
    
    /// Write data to file
    pub fn write_file(path: &Path, data: &[u8]) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .with_context(|| format!("Failed to create file: {}", path.display()))?;
        
        file.write_all(data)
            .with_context(|| format!("Failed to write file: {}", path.display()))?;
        
        Ok(())
    }
    
    /// Write data in chunks, calling on_progress(bytes_written, total) for progress reporting
    pub fn write_file_with_progress<F>(path: &Path, data: &[u8], mut on_progress: F) -> Result<()>
    where
        F: FnMut(u64, u64),
    {
        let total = data.len() as u64;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .with_context(|| format!("Failed to create file: {}", path.display()))?;
        
        let mut written = 0u64;
        for chunk in data.chunks(PROGRESS_CHUNK_SIZE) {
            file.write_all(chunk)
                .with_context(|| format!("Failed to write file: {}", path.display()))?;
            written += chunk.len() as u64;
            on_progress(written, total);
        }
        
        Ok(())
    }
    
    /// Ensure parent directories exist for the given path
    pub fn create_parent_dirs(path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
        Ok(())
    }
    
    /// Create encrypted file format with metadata (delegates to core)
    pub fn create_encrypted_file(
        salt: &[u8],
        encrypted_data: &[u8],
    ) -> Vec<u8> {
        encryptor_core::create_encrypted_file(salt, encrypted_data)
    }
    
    /// Parse encrypted file format (delegates to core)
    pub fn parse_encrypted_file(data: &[u8]) -> Result<([u8; 32], Vec<u8>)> {
        encryptor_core::parse_encrypted_file(data)
    }
}