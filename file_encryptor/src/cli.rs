use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "file-encryptor")]
#[command(about = "A secure file encryption tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encrypt a file
    Encrypt {
        /// Input file path
        #[arg(short, long)]
        input: String,
        
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Decrypt a file
    Decrypt {
        /// Input file path
        #[arg(short, long)]
        input: String,
        
        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Encrypt all files in a directory (preserves structure)
    EncryptDir {
        /// Input directory path
        #[arg(short, long)]
        input: String,
        
        /// Output directory path (optional, defaults to <input>.encrypted)
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Decrypt all .encrypted files in a directory (preserves structure)
    DecryptDir {
        /// Input directory path (containing .encrypted files)
        #[arg(short, long)]
        input: String,
        
        /// Output directory path (optional, defaults to <input> with .encrypted stripped)
        #[arg(short, long)]
        output: Option<String>,
    },
}