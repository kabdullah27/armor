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

1.  **Bun** (recommended) - Install via [bun.sh](https://bun.sh) — OR — **Node.js** (v18+)
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
    bun install
    ```

    > ⚠️ Gunakan **bun** (bukan npm) karena project ini menggunakan native binding yang hanya compatible dengan bun.

3.  **Run Development Mode**
    This will start the frontend dev server and the Tauri backend simultaneously.

    ```bash
    cargo tauri dev
    ```

    Or alternatively via bun:
    ```bash
    bun run tauri:dev
    ```

    The application window should open automatically.

## 📦 Building & Deployment

### 🚀 Automated Build via GitHub Actions (Recommended)

This project includes a GitHub Actions CI/CD workflow. The easiest way to automatically build the application for **Windows, macOS, and Linux** is by creating a new release version (Tagging).

1.  Ensure all code changes are committed and pushed to GitHub.
2.  Create and push a new tag starting with the letter `v` (e.g., `v1.0.13`). Pushing this tag will **automatically trigger GitHub Actions to build the application** (`.exe`, `.dmg`, `.deb`, `.AppImage`).
    ```bash
    git tag v1.0.13
    git push origin --tags
    ```
3.  Open the **Actions** tab in your GitHub repository to monitor the build process.
4.  Once completed, all installer files (`.exe`, `.dmg`, `.deb`, etc.) will automatically appear and be available for download on the GitHub **Releases** page.

---

### 🐳 Building with Docker (Linux Only)

If you are on macOS or Windows and want to build the **Linux** version locally without installing system dependencies:

1.  **Build the Image**:
    ```bash
    docker build -t armor-builder .
    ```
2.  **Run & Extract**:
    ```bash
    # Runs the build and puts artifacts in the 'output' folder in your current directory
    docker run --rm -v "$(pwd)/output:/app/src-tauri/target/release/bundle" armor-builder
    ```
    _Note: On Apple Silicon, this produces an ARM64 Linux build. For x86_64, add `--platform linux/amd64` (slow)._

---

### 💻 Native Building (Manual)

To build manually, you generally must be on the OS you are building for.

#### 🍎 macOS

```bash
bun run tauri build
# Output: src-tauri/target/release/bundle/macos/
```

#### 🐧 Linux

**Requirements:** `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`.

```bash
bun run tauri build
# Output: src-tauri/target/release/bundle/deb/
```

#### 🪟 Windows

**Requirements:** [C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

```powershell
bun run tauri build
# Output: src-tauri/target/release/bundle/nsis/
```

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

```

```
