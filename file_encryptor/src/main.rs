mod cli;
mod file_handler;

use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use rpassword::prompt_password;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

use cli::Cli;
use encryptor_core::Encryptor;
use file_handler::FileHandler;

const LARGE_FILE_THRESHOLD: u64 = 1024 * 1024; // 1 MB

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        cli::Commands::Encrypt { input, output } => {
            encrypt_file(&input, output.as_deref())?;
        }
        cli::Commands::Decrypt { input, output } => {
            decrypt_file(&input, output.as_deref())?;
        }
        cli::Commands::EncryptDir { input, output } => {
            encrypt_dir(&input, output.as_deref())?;
        }
        cli::Commands::DecryptDir { input, output } => {
            decrypt_dir(&input, output.as_deref())?;
        }
    }
    
    Ok(())
}

fn create_progress_bar(len: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(msg.to_string());
    pb
}

fn encrypt_file(input_path: &str, output_path: Option<&str>) -> Result<()> {
    // Prompt for password
    let password = prompt_password("Enter password: ")
        .context("Failed to read password")?;
    
    let input_path = Path::new(input_path);
    let file_size = std::fs::metadata(input_path)
        .with_context(|| format!("Failed to read input file metadata: {}", input_path.display()))?
        .len();
    
    let use_progress = file_size >= LARGE_FILE_THRESHOLD;
    
    // Read input file (with optional progress bar)
    let plaintext = if use_progress {
        let pb = create_progress_bar(file_size, "Reading");
        let data = FileHandler::read_file_with_progress(input_path, |read, _| {
            pb.set_position(read);
        })
        .with_context(|| format!("Failed to read input file: {}", input_path.display()))?;
        pb.finish_and_clear();
        data
    } else {
        FileHandler::read_file(input_path)
            .with_context(|| format!("Failed to read input file: {}", input_path.display()))?
    };
    
    // Generate salt and create encryptor
    let salt = Encryptor::generate_salt();
    let encryptor = Encryptor::new_with_password(&password, &salt)
        .context("Failed to initialize encryptor")?;
    
    // Encrypt data
    let encrypted_data = encryptor.encrypt(&plaintext)
        .context("Encryption failed")?;
    
    // Create encrypted file format
    let encrypted_file = FileHandler::create_encrypted_file(&salt, &encrypted_data);
    
    // Determine output path
    let output_path: PathBuf = output_path
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let mut path = input_path.to_path_buf();
            path.set_extension("encrypted");
            path
        });
    
    // Write encrypted file (with optional progress bar)
    if use_progress {
        let pb = create_progress_bar(encrypted_file.len() as u64, "Writing");
        FileHandler::write_file_with_progress(&output_path, &encrypted_file, |written, _| {
            pb.set_position(written);
        })
        .with_context(|| format!("Failed to write encrypted file: {}", output_path.display()))?;
        pb.finish_with_message("Done");
    } else {
        FileHandler::write_file(&output_path, &encrypted_file)
            .with_context(|| format!("Failed to write encrypted file: {}", output_path.display()))?;
    }
    
    println!("File encrypted successfully: {}", output_path.display());
    Ok(())
}

fn decrypt_file(input_path: &str, output_path: Option<&str>) -> Result<()> {
    // Prompt for password
    let password = prompt_password("Enter password: ")
        .context("Failed to read password")?;
    
    let input_path = Path::new(input_path);
    let file_size = std::fs::metadata(input_path)
        .with_context(|| format!("Failed to read encrypted file metadata: {}", input_path.display()))?
        .len();
    
    let use_progress = file_size >= LARGE_FILE_THRESHOLD;
    
    // Read encrypted file (with optional progress bar)
    let encrypted_file_data = if use_progress {
        let pb = create_progress_bar(file_size, "Reading");
        let data = FileHandler::read_file_with_progress(input_path, |read, _| {
            pb.set_position(read);
        })
        .with_context(|| format!("Failed to read encrypted file: {}", input_path.display()))?;
        pb.finish_and_clear();
        data
    } else {
        FileHandler::read_file(input_path)
            .with_context(|| format!("Failed to read encrypted file: {}", input_path.display()))?
    };
    
    // Parse encrypted file format
    let (salt, encrypted_data) = FileHandler::parse_encrypted_file(&encrypted_file_data)
        .context("Failed to parse encrypted file format")?;
    
    // Create decryptor with extracted salt
    let encryptor = Encryptor::new_with_password(&password, &salt)
        .context("Failed to initialize decryptor")?;
    
    // Decrypt data
    let plaintext = encryptor.decrypt(&encrypted_data)
        .context("Decryption failed - wrong password or corrupted file")?;
    
    // Determine output path
    let output_path: PathBuf = output_path.map(PathBuf::from).unwrap_or_else(|| {
        let mut path = input_path.to_path_buf();
        if path.extension().and_then(|s| s.to_str()) == Some("encrypted") {
            path.set_extension("");
        } else {
            path.set_extension("decrypted");
        }
        path
    });
    
    // Write decrypted file (with optional progress bar)
    if use_progress {
        let pb = create_progress_bar(plaintext.len() as u64, "Writing");
        FileHandler::write_file_with_progress(&output_path, &plaintext, |written, _| {
            pb.set_position(written);
        })
        .with_context(|| format!("Failed to write decrypted file: {}", output_path.display()))?;
        pb.finish_with_message("Done");
    } else {
        FileHandler::write_file(&output_path, &plaintext)
            .with_context(|| format!("Failed to write decrypted file: {}", output_path.display()))?;
    }
    
    println!("File decrypted successfully: {}", output_path.display());
    Ok(())
}

/// Collect all file paths in a directory recursively (files only, no symlinks)
fn collect_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let files: Vec<PathBuf> = WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();
    Ok(files)
}

fn encrypt_dir(input_dir: &str, output_dir: Option<&str>) -> Result<()> {
    let password = prompt_password("Enter password: ")
        .context("Failed to read password")?;
    
    let input_path = Path::new(input_dir);
    if !input_path.is_dir() {
        anyhow::bail!("Input is not a directory: {}", input_path.display());
    }
    
    let output_path: PathBuf = output_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("{}.encrypted", input_dir)));
    
    std::fs::create_dir_all(&output_path)
        .with_context(|| format!("Failed to create output directory: {}", output_path.display()))?;
    
    let files = collect_files(input_path)?;
    if files.is_empty() {
        println!("No files found in directory: {}", input_path.display());
        return Ok(());
    }
    
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Encrypting directory");
    
    let salt = Encryptor::generate_salt();
    let encryptor = Encryptor::new_with_password(&password, &salt)
        .context("Failed to initialize encryptor")?;
    
    for file_path in &files {
        let relative = file_path
            .strip_prefix(input_path)
            .with_context(|| format!("Failed to get relative path: {}", file_path.display()))?;
        
        let plaintext = FileHandler::read_file(file_path)
            .with_context(|| format!("Failed to read: {}", file_path.display()))?;
        
        let encrypted_data = encryptor.encrypt(&plaintext)
            .context("Encryption failed")?;
        let encrypted_file = FileHandler::create_encrypted_file(&salt, &encrypted_data);
        
        let mut out_file = output_path.join(relative);
        let ext = out_file
            .extension()
            .map(|e| format!("{}.encrypted", e.to_string_lossy()))
            .unwrap_or_else(|| "encrypted".to_string());
        out_file.set_extension(ext);
        
        FileHandler::create_parent_dirs(&out_file)?;
        FileHandler::write_file(&out_file, &encrypted_file)
            .with_context(|| format!("Failed to write: {}", out_file.display()))?;
        
        pb.inc(1);
    }
    
    pb.finish_with_message("Done");
    println!("Directory encrypted successfully: {} ({} files)", output_path.display(), files.len());
    Ok(())
}

fn decrypt_dir(input_dir: &str, output_dir: Option<&str>) -> Result<()> {
    let password = prompt_password("Enter password: ")
        .context("Failed to read password")?;
    
    let input_path = Path::new(input_dir);
    if !input_path.is_dir() {
        anyhow::bail!("Input is not a directory: {}", input_path.display());
    }
    
    let default_output = {
        let s = input_path.to_string_lossy();
        if s.ends_with(".encrypted") {
            PathBuf::from(s.strip_suffix(".encrypted").unwrap())
        } else {
            PathBuf::from(format!("{}_decrypted", s))
        }
    };
    
    let output_path: PathBuf = output_dir.map(PathBuf::from).unwrap_or(default_output);
    
    std::fs::create_dir_all(&output_path)
        .with_context(|| format!("Failed to create output directory: {}", output_path.display()))?;
    
    let all_files = collect_files(input_path)?;
    let files: Vec<PathBuf> = all_files
        .into_iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("encrypted"))
        .collect();
    
    if files.is_empty() {
        println!("No .encrypted files found in directory: {}", input_path.display());
        return Ok(());
    }
    
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Decrypting directory");
    
    for file_path in &files {
        let relative = file_path
            .strip_prefix(input_path)
            .with_context(|| format!("Failed to get relative path: {}", file_path.display()))?;
        
        let encrypted_file_data = FileHandler::read_file(file_path)
            .with_context(|| format!("Failed to read: {}", file_path.display()))?;
        
        let (salt, encrypted_data) = FileHandler::parse_encrypted_file(&encrypted_file_data)
            .with_context(|| format!("Invalid encrypted file: {}", file_path.display()))?;
        
        let encryptor = Encryptor::new_with_password(&password, &salt)
            .context("Failed to initialize decryptor")?;
        
        let plaintext = encryptor.decrypt(&encrypted_data)
            .with_context(|| format!("Decryption failed (wrong password?): {}", file_path.display()))?;
        
        // Strip .encrypted from extension: e.g. a.txt.encrypted -> a.txt
        let mut out_file = output_path.join(relative);
        if out_file.extension().and_then(|e| e.to_str()) == Some("encrypted") {
            out_file.set_extension("");
        } else {
            out_file.set_extension("decrypted");
        }
        
        FileHandler::create_parent_dirs(&out_file)?;
        FileHandler::write_file(&out_file, &plaintext)
            .with_context(|| format!("Failed to write: {}", out_file.display()))?;
        
        pb.inc(1);
    }
    
    pb.finish_with_message("Done");
    println!("Directory decrypted successfully: {} ({} files)", output_path.display(), files.len());
    Ok(())
}
