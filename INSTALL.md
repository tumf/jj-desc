# Installation Guide

This guide provides detailed installation instructions for `jj-desc` on various platforms.

## Table of Contents

- [Quick Install](#quick-install)
- [macOS](#macos)
- [Linux](#linux)
- [Windows](#windows)
- [Building from Source](#building-from-source)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

## Quick Install

### Homebrew (macOS/Linux)

```bash
brew install tumf/tap/jj-desc
```

### Shell Installer (macOS/Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
```

### PowerShell Installer (Windows)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.ps1 | iex"
```

## macOS

### Option 1: Homebrew (Recommended)

**Prerequisites:** [Homebrew](https://brew.sh/) installed

```bash
# Add the tap (first time only)
brew tap tumf/tap

# Install jj-desc
brew install jj-desc
```

**Update to latest version:**

```bash
brew update
brew upgrade jj-desc
```

**Uninstall:**

```bash
brew uninstall jj-desc
```

### Option 2: Shell Installer

This script automatically detects your architecture and installs to `~/.cargo/bin`:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
```

Make sure `~/.cargo/bin` is in your PATH:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc  # or ~/.bashrc
source ~/.zshrc
```

### Option 3: Manual Download

**For Apple Silicon (M1/M2/M3/M4):**

```bash
cd /tmp
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-apple-darwin.tar.xz
tar xf jj-desc-aarch64-apple-darwin.tar.xz
sudo mv jj-desc /usr/local/bin/
sudo chmod +x /usr/local/bin/jj-desc
```

**For Intel Macs:**

```bash
cd /tmp
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-apple-darwin.tar.xz
tar xf jj-desc-x86_64-apple-darwin.tar.xz
sudo mv jj-desc /usr/local/bin/
sudo chmod +x /usr/local/bin/jj-desc
```

**Verify architecture:**

```bash
uname -m
# Apple Silicon: arm64
# Intel: x86_64
```

## Linux

### Option 1: Shell Installer (Recommended)

This script automatically detects your architecture and installs to `~/.cargo/bin`:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.sh | sh
```

Make sure `~/.cargo/bin` is in your PATH:

```bash
# For bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Option 2: Homebrew on Linux

If you have [Homebrew on Linux](https://docs.brew.sh/Homebrew-on-Linux) installed:

```bash
brew install tumf/tap/jj-desc
```

### Option 3: Manual Download

**For x86_64 (AMD/Intel 64-bit):**

```bash
cd /tmp
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-unknown-linux-gnu.tar.xz
tar xf jj-desc-x86_64-unknown-linux-gnu.tar.xz
sudo mv jj-desc /usr/local/bin/
sudo chmod +x /usr/local/bin/jj-desc
```

**For ARM64 (aarch64):**

```bash
cd /tmp
curl -LO https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-aarch64-unknown-linux-gnu.tar.xz
tar xf jj-desc-aarch64-unknown-linux-gnu.tar.xz
sudo mv jj-desc /usr/local/bin/
sudo chmod +x /usr/local/bin/jj-desc
```

**Verify architecture:**

```bash
uname -m
# x86_64: x86_64
# ARM64: aarch64
```

### Distribution-Specific Notes

**Ubuntu/Debian:**

No additional dependencies required. The binaries are statically linked.

**Arch Linux:**

Consider creating an AUR package or use one of the methods above.

**Fedora/RHEL/CentOS:**

No additional dependencies required.

## Windows

### Option 1: PowerShell Installer (Recommended)

Open PowerShell as Administrator and run:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-installer.ps1 | iex"
```

This installs to `%USERPROFILE%\.cargo\bin\jj-desc.exe` and updates your PATH automatically.

### Option 2: Manual Download

1. Download [jj-desc-x86_64-pc-windows-msvc.zip](https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-pc-windows-msvc.zip)

2. Extract the ZIP file (right-click → Extract All)

3. Move `jj-desc.exe` to a directory in your PATH, such as:
   - `C:\Program Files\jj-desc\` (recommended)
   - `%USERPROFILE%\bin\`
   - `%USERPROFILE%\.cargo\bin\`

4. Add the directory to your PATH:
   - Press `Win + X` → System
   - Click "Advanced system settings"
   - Click "Environment Variables"
   - Under "User variables", select "Path" → Edit
   - Click "New" and add the directory path
   - Click OK to save

### Option 3: Scoop Package Manager

If you use [Scoop](https://scoop.sh/):

```powershell
# Coming soon - package submission pending
scoop bucket add tumf https://github.com/tumf/scoop-bucket
scoop install jj-desc
```

### Option 4: Chocolatey

```powershell
# Coming soon - package submission pending
choco install jj-desc
```

## Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.85 or later (Edition 2024 support)
- Git (optional, for cloning)

### Install via Cargo

Install directly from the Git repository:

```bash
cargo install --git https://github.com/tumf/jj-desc
```

Or clone and build locally:

```bash
# Clone the repository
git clone https://github.com/tumf/jj-desc
cd jj-desc

# Build and install
cargo install --path .

# Or build for development
cargo build --release
# Binary will be at: target/release/jj-desc
```

### Cross-Compilation

To build for a different platform:

```bash
# Install cross-compilation tools
cargo install cross

# Build for Linux ARM64 from macOS/Windows
cross build --release --target aarch64-unknown-linux-gnu

# Build for Windows from macOS/Linux
cross build --release --target x86_64-pc-windows-msvc
```

## Verification

After installation, verify that `jj-desc` is installed correctly:

```bash
# Check version
jj-desc --version

# Should output: jj-desc 0.2.0 (or latest version)

# Check help
jj-desc --help

# Test with dry-run (requires jj and LLM API key configured)
jj-desc --dry-run
```

## Troubleshooting

### Command not found

**Problem:** `jj-desc: command not found` or `'jj-desc' is not recognized`

**Solution:**

1. **Check if installed:**
   ```bash
   # macOS/Linux
   which jj-desc
   ls ~/.cargo/bin/jj-desc
   ls /usr/local/bin/jj-desc

   # Windows
   where jj-desc
   ```

2. **Update PATH:**
   ```bash
   # macOS/Linux (add to ~/.zshrc or ~/.bashrc)
   export PATH="$HOME/.cargo/bin:$PATH"
   export PATH="/usr/local/bin:$PATH"

   # Windows PowerShell
   $env:Path += ";$env:USERPROFILE\.cargo\bin"
   ```

3. **Restart your shell** or run:
   ```bash
   source ~/.zshrc  # or ~/.bashrc
   ```

### Permission denied

**Problem:** `Permission denied` when trying to run

**Solution:**

```bash
# macOS/Linux
chmod +x /usr/local/bin/jj-desc
# or
chmod +x ~/.cargo/bin/jj-desc
```

### SSL/TLS errors

**Problem:** Certificate verification errors on some systems

**Solution:** The binaries use `rustls` instead of OpenSSL, which should work on all platforms. If you still encounter issues, try:

```bash
# Update system certificates
# Ubuntu/Debian
sudo apt-get update && sudo apt-get install ca-certificates

# macOS
brew install ca-certificates

# Or use a custom base URL without SSL verification (not recommended for production)
export OPENROUTER_BASE_URL="http://..."  # Only for testing
```

### Homebrew installation fails

**Problem:** Homebrew can't find the formula

**Solution:**

```bash
# Update Homebrew
brew update

# Add the tap explicitly
brew tap tumf/tap

# Try installing again
brew install tumf/tap/jj-desc

# If still fails, check tap status
brew tap-info tumf/tap
```

### Windows Defender blocks installer

**Problem:** Windows Defender SmartScreen blocks the installer

**Solution:**

1. Click "More info" in the SmartScreen warning
2. Click "Run anyway"
3. This is expected for new/unsigned executables. The binary is safe (you can verify checksums)

**Alternative:** Use manual download and verify SHA256 checksum:

```powershell
# Download checksum file
Invoke-WebRequest -Uri "https://github.com/tumf/jj-desc/releases/latest/download/jj-desc-x86_64-pc-windows-msvc.zip.sha256" -OutFile checksum.txt

# Verify (compare with actual file hash)
Get-FileHash jj-desc-x86_64-pc-windows-msvc.zip -Algorithm SHA256
```

### Wrong architecture downloaded

**Problem:** Binary doesn't run or shows architecture mismatch

**Solution:**

Check your system architecture and download the correct binary:

```bash
# macOS/Linux
uname -m

# Windows PowerShell
$env:PROCESSOR_ARCHITECTURE
```

Then download the matching binary:
- `aarch64` / `arm64` → ARM64 binary
- `x86_64` / `AMD64` → x86_64 binary

## Getting Help

If you encounter issues not covered here:

1. Check [GitHub Issues](https://github.com/tumf/jj-desc/issues)
2. Open a new issue with:
   - Your OS and version
   - Installation method used
   - Complete error message
   - Output of `jj-desc --version` (if installed)

## Next Steps

After installation:

1. [Configure your LLM provider](README.md#configuration)
2. [Learn basic usage](README.md#usage)
3. [Explore examples](README.md#examples)
