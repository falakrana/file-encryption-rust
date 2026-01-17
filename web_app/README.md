# Encryption Suite - Web Application 🌐

This is the frontend component of the **Rust Encryption/Decryption Suite**. It provides a user-friendly interface for securing your files directly in the browser using WebAssembly.

## 🚀 Built With

-   **React + TypeScript**: For a robust and type-safe UI.
-   **Vite**: For lightning-fast development and building.
-   **Tailwind CSS**: For modern, responsive styling.
-   **Rust (WASM)**: For high-performance, client-side cryptographic operations.

## 🛠️ Setup & Running

This application depends on the WebAssembly module located in the root `wasm/` directory.

### 1. Build the WASM module
From the root directory:
```bash
cd wasm
wasm-pack build --target web --out-dir ../web_app/src/pkg
```

### 2. Install Dependencies
From this directory (`web_app/`):
```bash
npm install
```

### 3. Start Development Server
```bash
npm run dev
```

## 🔐 Security

-   **Browser-Based**: All encryption happens within your browser's memory.
-   **No Background Uploads**: Files never leave your machine; the "upload" is strictly local to the WASM module.
-   **Authenticated Encryption**: Uses AES-256-GCM to ensure file integrity and secrecy.

---

For more information on the core logic and CLI, please refer to the [Root README](../README.md).
