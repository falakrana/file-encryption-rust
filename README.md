# Rust Encryption/Decryption Suite 🛡️

A powerful, high-performance, and secure encryption/decryption suite built with **Rust**, **WebAssembly**, and **React**. This project provides both a robust Command Line Interface (CLI) and a modern Web Application to handle your sensitive data with industry-standard cryptographic practices.

## 🚀 Overview

This suite is designed to provide seamless encryption across different platforms while maintaining maximum security. By leveraging Rust's performance and safety, and WebAssembly's portability, you can encrypt files locally on your machine or directly in your browser without ever sending your data to a server.

### Key Components

-   **`core/`**: The heart of the suite. A pure Rust crate containing the cryptographic logic.
-   **`file_encryptor/`**: A high-speed CLI tool for file and directory encryption.
-   **`wasm/`**: WebAssembly bindings that bring Rust's crypto power to the web.
-   **`web_app/`**: A modern, responsive React frontend built with Vite and Tailwind CSS.

---

## 🔒 Security Specifications

We prioritize security by using proven, modern algorithms:

-   **Encryption**: `AES-256-GCM` (Advanced Encryption Standard with Galois/Counter Mode). Provides both confidentiality and authenticity.
-   **Key Derivation**: `Argon2id`. A memory-hard function resistant to GPU-based brute-force attacks.
-   **Zero-Knowledge**: All encryption and decryption happen locally. Your passwords and unencrypted data never leave your device.

---

## ✨ Features

### 🖥️ CLI (Command Line Interface)
-   **High Performance**: Efficiently handles large files with progress indicators.
-   **Directory Encryption**: Encrypt/Decrypt entire folders recursively.
-   **Cross-Platform**: Works on Windows, macOS, and Linux.

### 🌐 Web Application
-   **Client-Side Only**: Uses WASM to perform encryption in the browser.
-   **User-Friendly**: Drag-and-drop interface for files.
-   **Modern UI**: Sleek, responsive design with dark mode support.

---

## 🛠️ Getting Started

### Prerequisites

-   **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (latest stable)
-   **Node.js**: [Install Node.js](https://nodejs.org/) (v18+)
-   **wasm-pack**: [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for building the WASM bridge)

### Installation

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/your-username/rust-project-encryption-decryption.git
    cd rust-project-encryption-decryption
    ```

2.  **Build the CLI**:
    ```bash
    cargo build --release -p file_encryptor
    ```

3.  **Setup the Web App**:
    ```bash
    # Build WASM first
    cd wasm
    wasm-pack build --target web --out-dir ../web_app/src/pkg
    
    # Install web dependencies
    cd ../web_app
    npm install
    ```

---

## 📖 Usage

### Using the CLI

The CLI is available through the `file_encryptor` crate.

```bash
# Encrypt a single file
cargo run -p file_encryptor -- encrypt --input path/to/file.txt

# Decrypt a file
cargo run -p file_encryptor -- decrypt --input path/to/file.txt.encrypted

# Encrypt a whole directory
cargo run -p file_encryptor -- encrypt-dir --input path/to/folder
```

### Using the Web App

1.  Start the development server:
    ```bash
    cd web_app
    npm run dev
    ```
2.  Open your browser to the local URL (usually `http://localhost:5173`).
3.  Upload your file, enter a password, and download the secured version!

---

## 📂 Project Structure

```text
.
├── core/                # Core cryptographic logic (AES-GCM, Argon2)
├── file_encryptor/      # CLI application source code
├── wasm/                # Rust to WASM bridge
├── web_app/             # React + Vite + Tailwind frontend
├── Cargo.toml           # Workspace configuration
└── .gitignore           # Project-wide ignore rules
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an issue for bugs and feature requests.

