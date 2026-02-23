# Kirei (Clean/Beautiful in Japanese)

A fast, safe, and beautiful macOS system cleaner built in Rust. It helps developers reclaim disk space by identifying and removing cache files, build artifacts, and logs that accumulate over time.

## 🚀 Features

- **Developer Tools**: Deep cleaning for:
    - **Xcode**: DerivedData, Archives, Device Support
    - **Node/Bun**: `node_modules` caches, `.npm`, `.bun`
    - **Cargo**: Registry and git checkouts
    - **Logs**: User logs (`~/Library/Logs`)
- **System**: Used app caches (`~/Library/Caches`)
- **Interactive TUI**: Review every file before deleting. Visual dashboard.
- **Safety First**:
    - **Trash Integration**: Moves files to macOS Trash by default (recoverable).
    - **Non-Destructive**: `scan` is read-only.
    - **Whitelisting**: Doesn't touch system files.

## 📦 Installation

```bash
# From source
git clone https://github.com/varunv/tools.git
cd tools/kirei
cargo install --path .
```

## 🛠 Usage

### Interactive Mode (Recommended)

Simply run `kirei` to start the interactive Terminal User Interface (TUI).

```bash
kirei
```

**Controls:**
- `Arrow Keys` / `j/k`: Navigate lists
- `Space`: Toggle selection for cleaning
- `Tab`: Switch between Dashboard and List view
- `c`: Clean selected items (Moves to Trash)
- `r`: Rescan system
- `q`: Quit

### CLI Mode

Generate a quick report without entering interactive mode.

```bash
kirei scan
```

*Note: CLI deletion is not yet implemented for safety. Use the TUI to delete.*

## 🛡 Safety & Design

Kirei is designed to be **safe by default**.
1. **No `rm -rf`**: Uses the `trash` crate to move files to the system Trash. You can undo any action.
2. **Read-Only Scan**: The default action is always just to look.
3. **Explicit Action**: You must manually select items and press `c` to clean.

## 🏗 Architecture

- **Rust**: For performance and safety.
- **Ratatui**: For the text-based user interface.
- **Tokio**: For asynchronous, parallel scanning of directories.

