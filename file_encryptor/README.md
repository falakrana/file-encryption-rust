# File Encryptor

A secure, command-line file and directory encryption tool built with Rust. Encrypts individual files or entire directory structures using AES-256-GCM encryption with Argon2id key derivation.

## Description

File Encryptor provides a simple, secure way to encrypt and decrypt files and folders. It uses password-based encryption with cryptographically secure algorithms, suitable for protecting sensitive data. The tool preserves directory structure when encrypting folders and includes progress indicators for large files.

## Problem Statement

Files and directories often need encryption for secure storage, backup, or transmission. This tool addresses the need for a fast, secure, and easy-to-use encryption solution that works at both file and directory levels while maintaining the original folder hierarchy.

## Features

- **File Encryption/Decryption**: Encrypt or decrypt individual files
- **Directory Encryption/Decryption**: Recursively encrypt/decrypt entire directory structures while preserving hierarchy
- **Secure Algorithms**: AES-256-GCM encryption with Argon2id password-based key derivation
- **Progress Indicators**: Visual progress bars for large files (>1MB) and batch operations
- **Custom File Format**: Self-contained encrypted files with embedded salt and metadata
- **Password Security**: Password prompts with hidden input via `rpassword`
- **Memory Safety**: Built with Rust's type system and zeroize for secure key handling

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Core Dependencies**:
  - `aes-gcm` (0.10) - AES-256-GCM authenticated encryption
  - `argon2` (0.5) - Argon2id key derivation function
  - `clap` (4.4) - Command-line argument parsing
  - `anyhow` (1.0) - Error handling
  - `rpassword` (7.2) - Secure password input
  - `zeroize` (1.6) - Secure memory zeroing
  - `indicatif` (0.17) - Progress bars
  - `walkdir` (2) - Recursive directory traversal
  - `rand` (0.8) - Cryptographically secure random number generation
  - `hex` (0.4) - Hexadecimal encoding utilities

## Encryption/Decryption Overview

### Encryption Process

1. **Password Input**: User enters password via secure prompt
2. **Salt Generation**: Random 32-byte salt generated for each encryption operation
3. **Key Derivation**: Password is derived to 32-byte key using Argon2id:
   - Memory: 65536 KB
   - Iterations: 3
   - Parallelism: 4
   - Output length: 32 bytes
4. **Encryption**: Plaintext encrypted with AES-256-GCM:
   - Random 12-byte nonce generated per file
   - Nonce prepended to ciphertext
5. **File Format**: Encrypted data stored in custom format:
   - Magic bytes: `ENCR` (4 bytes)
   - Version: `1` (1 byte)
   - Salt: 32 bytes
   - Encrypted payload: 12-byte nonce + ciphertext

### Decryption Process

1. **Password Input**: User enters password via secure prompt
2. **File Parsing**: Encrypted file parsed to extract salt and encrypted payload
3. **Key Derivation**: Same Argon2id parameters used to derive key from password + salt
4. **Decryption**: Nonce extracted and AES-256-GCM decryption performed
5. **Output**: Decrypted plaintext written to destination file

### Directory Encryption

- Uses single salt for all files in directory (shared key derivation)
- Preserves relative directory structure
- Files renamed with `.encrypted` extension appended (e.g., `file.txt` → `file.txt.encrypted`)
- Only processes regular files (no symlinks)

## Folder Structure

```
file_encryptor/
├── Cargo.toml              # Project metadata and dependencies
├── Cargo.lock              # Dependency version lock file
├── src/
│   ├── main.rs             # Entry point, CLI orchestration
│   ├── cli.rs              # Command-line argument definitions
│   ├── crypto.rs           # Encryption/decryption logic (AES-GCM, Argon2)
│   └── file_handler.rs     # File I/O, format handling, progress tracking
├── demoFile.txt            # Example file for testing
├── demoFolder/             # Example directory for testing
│   └── indexFile.txt       # Example nested file
├── steps.txt               # Usage documentation
└── target/                 # Build artifacts (generated)
```

## Installation & Build

### Prerequisites

- Rust 1.70+ (with Cargo)
- Windows, Linux, or macOS

### Build Instructions

1. Clone or navigate to the project directory:
   ```bash
   cd file_encryptor
   ```

2. Build in release mode (recommended):
   ```bash
   cargo build --release
   ```

3. The binary will be located at:
   - Windows: `target/release/file_encryptor.exe`
   - Unix: `target/release/file_encryptor`

4. (Optional) Install globally:
   ```bash
   cargo install --path .
   ```

## How to Run

### File Encryption

Encrypt a single file with default output (appends `.encrypted`):
```bash
file_encryptor encrypt --input demoFile.txt
```

Encrypt with explicit output path:
```bash
file_encryptor encrypt --input demoFile.txt --output demoFile.encrypted
```

### File Decryption

Decrypt with default output (removes `.encrypted` or appends `.decrypted`):
```bash
file_encryptor decrypt --input demoFile.encrypted
```

Decrypt with explicit output:
```bash
file_encryptor decrypt --input demoFile.encrypted --output demoFile.txt
```

### Directory Encryption

Encrypt entire directory with default output (`<dir>.encrypted`):
```bash
file_encryptor encrypt-dir --input ./demoFolder
```

Encrypt with explicit output directory:
```bash
file_encryptor encrypt-dir --input ./demoFolder --output ./demoFolder.encrypted
```

### Directory Decryption

Decrypt directory with default output (strips `.encrypted` or appends `_decrypted`):
```bash
file_encryptor decrypt-dir --input ./demoFolder.encrypted
```

Decrypt with explicit output:
```bash
file_encryptor decrypt-dir --input ./demoFolder.encrypted --output ./demoFolder
```

## Example Commands

```bash
# Encrypt a single file
file_encryptor encrypt -i document.pdf

# Decrypt a file
file_encryptor decrypt -i document.pdf.encrypted

# Encrypt a folder
file_encryptor encrypt-dir -i ./backup

# Decrypt a folder
file_encryptor decrypt-dir -i ./backup.encrypted
```

**Note**: All operations prompt for a password interactively. The password is hidden during input.

## Demo Explanation

The repository includes example files for testing:

- **`demoFile.txt`**: Single file containing "This is demoFile that should be encrypted."
- **`demoFolder/indexFile.txt`**: Nested file for directory encryption testing

### Testing Workflow

1. Encrypt `demoFile.txt`:
   ```bash
   file_encryptor encrypt --input demoFile.txt
   ```
   Creates `demoFile.encrypted` (password required).

2. Decrypt the encrypted file:
   ```bash
   file_encryptor decrypt --input demoFile.encrypted
   ```

3. Encrypt `demoFolder`:
   ```bash
   file_encryptor encrypt-dir --input ./demoFolder
   ```
   Creates `demoFolder.encrypted/` with `indexFile.txt.encrypted` inside.

4. Decrypt the encrypted folder:
   ```bash
   file_encryptor decrypt-dir --input ./demoFolder.encrypted
   ```

## Security Notes

### Strengths

- **AES-256-GCM**: Authenticated encryption prevents tampering
- **Argon2id**: Memory-hard KDF resistant to GPU/ASIC attacks
- **Unique Nonces**: Each file encrypted with random nonce (prevents IV reuse)
- **Secure Memory**: Uses `zeroize` for key material cleanup
- **Salt Storage**: Unique salt per encryption prevents rainbow table attacks

### Limitations & Assumptions

- **Password Security**: Strength depends on user's password choice. Weak passwords are vulnerable to brute-force attacks.
- **Key Derivation**: Argon2id parameters (65536 KB, 3 iterations) are moderate. For higher security, consider increasing iterations (may slow encryption).
- **No Key Management**: No built-in key management system. Password must be remembered or stored separately.
- **Single Salt Per Directory**: All files in a directory share the same salt. If one file is compromised, others using the same salt may be at risk (though different nonces mitigate this).
- **No File Metadata**: Original file permissions, timestamps, and attributes are not preserved.
- **Memory Limits**: Entire file loaded into memory. Very large files (>RAM) may cause issues.
- **No Integrity Verification**: While GCM provides authentication, there's no separate checksum for the file format itself.
- **Version Field**: Current version is `1`. Future format changes may break compatibility.

### Recommendations

- Use strong, unique passwords (preferably from a password manager)
- Consider increasing Argon2id iterations for long-term storage
- Test encryption/decryption before deleting originals
- Keep backups of encrypted files in multiple locations
- For production use, consider key management integration

## Known Limitations

1. **File Size**: Files must fit in available RAM (no streaming encryption)
2. **Symlinks**: Symbolic links are not followed or encrypted
3. **Metadata Loss**: Original file timestamps and permissions not preserved
4. **Single Password**: One password per encryption operation (no multi-factor)
5. **No Compression**: Files are encrypted as-is (no size optimization)
6. **Platform-Specific**: Path handling may differ slightly across Windows/Unix
7. **Error Recovery**: Partial encryption/decryption failures may leave inconsistent state

## Future Improvements

- Streaming encryption for large files (chunk-based processing)
- Compression before encryption (optional)
- Preserve file metadata (timestamps, permissions)
- Key derivation parameter configuration (CLI flags)
- Key management integration (hardware tokens, keyring)
- Multi-factor authentication support
- Encrypted archive format (single file for directories)
- Progress persistence for interrupted operations
- Benchmarking and performance optimization
- Unit and integration test suite