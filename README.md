# ARMOR - Offline PGP Desktop Application

Armor is a secure, easy-to-use, offline-first PGP desktop application built with **Rust** (Tauri) and **SvelteKit**. It allows users to manage keys, encrypt, decrypt, sign, and verify files with a modern, user-friendly interface.

## 🚀 distinct Features

- **Secure**: Built on `sequoia-openpgp` and Rust. Keys are stored locally in an encrypted SQLite vault (or custom location).
- **Offline-First**: Designed to work entirely without an internet connection.
- **Modern UI**: Clean, responsive interface built with Svelte and customized Tailwind CSS.
- **Cross-Platform**: Runs on macOS, Windows, and Linux.
- **Key Management**: Generate (RSA/Ed25519), Import, Export, and Delete keys.
- **File Operations**: Encrypt, Decrypt, Sign, and Verify files easily.
- **Database Management**: Backup and Restore your key vault.

## 🛠️ Tech Stack

- **Frontend**: SvelteKit, TypeScript, Tailwind CSS
- **Backend**: Rust, Tauri v2
- **Database**: SQLite (via `rusqlite`)
- **Cryptography**: Sequoia OpenPGP (`sequoia-openpgp`)

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

1.  **Node.js** (v18 or v20 recommended) & **npm** (or pnpm/bun/yarn)
2.  **Rust** (v1.75+) - Install via [rustup.rs](https://rustup.rs/)

### Platform-Specific Requirements

#### 🍎 macOS

- **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```

#### 🪟 Windows

- **Microsoft Visual Studio C++ Build Tools**:
  - Download from [Visual Studio Downloads](https://visualstudio.microsoft.com/downloads/).
  - Select "Desktop development with C++" workload during installation.

#### 🐧 Linux

- **System Dependencies** (Ubuntu/Debian example):
  ```bash
  sudo apt update
  sudo apt install libwebkit2gtk-4.0-dev \
      build-essential \
      curl \
      wget \
      file \
      libssl-dev \
      libgtk-3-dev \
      libayatana-appindicator3-dev \
      librsvg2-dev
  ```
  _(Note: Package names may vary for other distributions like Fedora or Arch.)_

## 💻 Installation & Development

1.  **Clone the Repository**

    ```bash
    git clone https://github.com/yourusername/armor.git
    cd armor
    ```

2.  **Install Frontend Dependencies**

    ```bash
    npm install
    # or
    bun install
    ```

3.  **Run Development Mode**
    This will start the frontend dev server and the Tauri backend simultaneously.

    ```bash
    npm run tauri:dev
    # or
    bun run tauri:dev
    # or
    cargo tauri dev
    ```

    The application window should open automatically.

## 📦 Building for Production

To build the distributable application (installer/executable):

```bash
npm run tauri:build
```

The output files will be located in:

- `src-tauri/target/release/bundle/macos/` (macOS .app, .dmg)
- `src-tauri/target/release/bundle/msi/` (Windows .msi)
- `src-tauri/target/release/bundle/deb/` (Linux .deb)

## 🗃️ Project Structure

├── src/ # SvelteKit Frontend
│ ├── routes/ # Pages (Keys, Encrypt, Settings, etc.)
│ ├── lib/ # Components, Stores, API wrappers
│ └── app.html
├── src-tauri/ # Rust Backend
│ ├── src/
│ │ ├── commands/ # Tauri Commands (API implementations)
│ │ ├── core/ # Business logic (Crypto, Database)
│ │ ├── lib.rs # App entry point & registration
│ │ └── main.rs
│ ├── Cargo.toml # Rust dependencies
│ └── tauri.conf.json # Tauri configuration
└── package.json

```

## 🤝 Contributing

1.  Fork the repository
2.  Create your feature branch (`git checkout -b feature/amazing-feature`)
3.  Commit your changes (`git commit -m 'Add some amazing feature'`)
4.  Push to the branch (`git push origin feature/amazing-feature`)
5.  Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
```
