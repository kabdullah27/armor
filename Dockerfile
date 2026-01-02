# Use Ubuntu 22.04 as base image for broad compatibility
FROM ubuntu:22.04

# Avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# 1. Install System Dependencies
# Tauri and sequoia-openpgp require these libraries
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libwebkit2gtk-4.0-dev \
    nettle-dev \
    llvm \
    clang \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 2. Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 3. Install Bun
ENV BUN_INSTALL="/root/.bun"
ENV PATH="${BUN_INSTALL}/bin:${PATH}"
RUN curl -fsSL https://bun.sh/install | bash

# 4. Set Working Directory
WORKDIR /app

# 5. Copy Dependencies Files first (for caching)
COPY package.json bun.lockb ./
COPY src-tauri/Cargo.toml src-tauri/tauri.conf.json ./src-tauri/

# Create dummy src-tauri/src/main.rs to build dependencies
RUN mkdir -p src-tauri/src && echo "fn main() {}" > src-tauri/src/main.rs

# 6. Install Frontend Dependencies
RUN bun install

# 7. Copy Source Code
COPY . .

# 8. Build
# We use a script to ensure the output directory exists and build
CMD ["bun", "run", "tauri", "build"]
