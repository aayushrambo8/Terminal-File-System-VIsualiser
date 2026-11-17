<div align="center">

# 🗂️ TFSV

### Terminal File System Visualiser

**A blazingly fast, keyboard & mouse friendly terminal file manager for Linux and SSH sessions, written in Rust.**

[![Crates.io Version](https://img.shields.io/badge/version-0.1.2-blue?style=for-the-badge&logo=rust)](Cargo.toml)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-orange?style=for-the-badge&logo=rust)](Cargo.toml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-green.svg?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux-lightgrey?style=for-the-badge&logo=linux)](https://www.kernel.org)
[![Built with Ratatui](https://img.shields.io/badge/TUI-Ratatui%20%26%20Crossterm-red?style=for-the-badge)](https://ratatui.rs)

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-keyboard--mouse-shortcuts">Controls</a> •
  <a href="#-configuration">Configuration</a> •
  <a href="#-security--sandboxing">Security</a> •
  <a href="#-architecture">Architecture</a>
</p>

```text
┌── TFSV ────────────────────────────────────────────────────────────────────────┐
│ [New] [Copy] [Cut] [Paste] [Rename] [Delete] [Refresh] [Sort: Name] [Hidden]   │
├─────────────────┬──────────────────────────────────────────────────────────────┤
│  PLACES         │  📁 /home/user/workspace/tsfv                                │
│                 ├──────────────────────────────────────────────────────────────┤
│   Home         │   ..                                                        │
│   Desktop      │   docs/                                                     │
│   Documents    │   src/                                                      │
│   Downloads    │   tests/                                                    │
│   Music        │   Cargo.toml                                         747 B  │
│   Pictures     │   Cargo.lock                                       23.1 KB  │
│   Videos       │   README.md                                         4.4 KB  │
│   Trash        │   install.sh                                        3.3 KB  │
│                 │   LICENSE                                          35.1 KB  │
├─────────────────┴──────────────────────────────────────────────────────────────┤
│  [Normal Mode] 9 items | Selected: README.md (4.4 KB)           Press '?' / 'q'│
└────────────────────────────────────────────────────────────────────────────────┘
```

</div>

---

## ✨ Features

- **⚡ Blazingly Fast & Lightweight**  
  Engineered in pure Rust with minimal memory footprint and instant startup times. No electron bloat, no heavy dependencies.

- **🌐 Remote & SSH Optimized**  
  Built specifically for local environments and remote SSH terminal sessions. Operates reliably without requiring X11 forwarding or GPU acceleration.

- **🖱️ Hybrid Keyboard & Mouse Navigation**  
  Seamlessly navigate with full Vim-style keys (`h`/`j`/`k`/`l`), arrow keys, or full point-and-click mouse control (sidebar places, breadcrumbs, header buttons, and right-click context menu).

- **🛡️ Built-in Path Sandboxing**  
  Restricts regular sessions within `$HOME` to protect critical system directories from accidental modifications. Full unrestricted system access is safely unlocked via `sudo`.

- **🎨 Nerd Font & Unicode Icons**  
  Automatic file-type detection with rich Devicons and Codicons glyphs for programming languages, documents, media, and archives.

- **📋 Clipboard & Batch File Operations**  
  Full support for Copy, Cut, and Paste across directories with visual prompts for creating directories, renaming items, and deletion confirmations.

- **🚀 Smart External Openers**  
  Seamlessly launches your configured `$EDITOR` (e.g. `nvim`, `nano`, `vim`) or system opener (`xdg-open`) by cleanly switching out of the alternate screen and restoring the TUI state when exiting.

- **⚙️ Compliant XDG Configuration**  
  Reads and persists custom settings in `~/.config/tfsv/settings.toml` with environment overrides for isolated multi-user setups.

---

## 🚀 Quick Start

### Automated Installation (Linux)

> [!NOTE]  
> **Requirements**: Linux with `git` and either `curl` or `wget`.  
> The installer automatically fetches and configures the minimal stable Rust toolchain via Rustup if Cargo is not present.

Clone and run the interactive installer:

```bash
git clone https://github.com/aayushrambo8/Terminal-File-System-VIsualiser.git
cd Terminal-File-System-VIsualiser
./install.sh
```

- **User Install (Default)**: Installs the standalone executable to `~/.local/bin/tfsv` and adds it to your shell configuration (`.bashrc`, `.zshrc`, or `.profile`).
- **System-Wide Install (Root)**: Run with `sudo` to install to `/usr/local/bin/tfsv` for all system users:

  ```bash
  sudo ./install.sh
  ```

- **Custom Directory Install**:

  ```bash
  sudo env TFSV_INSTALL_DIR=/home/opt ./install.sh
  ```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/aayushrambo8/Terminal-File-System-VIsualiser.git
cd Terminal-File-System-VIsualiser

# Build release binary
cargo build --release

# Run directly
cargo run --bin tfsv
```

---

## ⌨️ Keyboard & Mouse Shortcuts

### 🧭 Navigation

| Shortcut | Action |
|:---|:---|
| <kbd>j</kbd> / <kbd>↓</kbd> | Move cursor down |
| <kbd>k</kbd> / <kbd>↑</kbd> | Move cursor up |
| <kbd>l</kbd> / <kbd>→</kbd> | Enter selected directory |
| <kbd>h</kbd> / <kbd>←</kbd> | Move to parent directory |
| <kbd>Enter</kbd> | Open file in editor/viewer or enter directory |
| <kbd>Home</kbd> / <kbd>End</kbd> | Jump to first / last item in list |
| <kbd>Page Up</kbd> / <kbd>Page Down</kbd> | Jump cursor by 10 items |

### 🛠️ File Operations

| Shortcut | Action |
|:---|:---|
| <kbd>n</kbd> | Create new directory (interactive input dialog) |
| <kbd>m</kbd> | Rename selected file or directory |
| <kbd>d</kbd> | Delete selected file or directory (with confirmation) |
| <kbd>Ctrl</kbd> + <kbd>c</kbd> | Copy selected item to clipboard |
| <kbd>Ctrl</kbd> + <kbd>x</kbd> | Cut selected item to clipboard |
| <kbd>Ctrl</kbd> + <kbd>v</kbd> | Paste item from clipboard into current directory |
| <kbd>r</kbd> | Refresh current directory view |
| <kbd>s</kbd> | Cycle sorting mode (`Name` ➔ `Type` ➔ `Size`) |
| <kbd>.</kbd> | Toggle visibility of hidden files (`.*`) |
| <kbd>c</kbd> | Toggle right-click context menu |
| <kbd>q</kbd> / <kbd>Esc</kbd> | Close dialog / Context menu / Exit TFSV |

### 🖱️ Mouse Interactions

- **Left-Click Ribbon**: Trigger toolbar quick actions (`[New]`, `[Copy]`, `[Paste]`, `[Sort]`, etc.).
- **Left-Click Sidebar**: Instant jump to Places (`Home`, `Desktop`, `Documents`, `Downloads`, `Trash`, etc.).
- **Left-Click File**: Select item or enter directory.
- **Right-Click**: Open context menu at cursor position.
- **Scroll Wheel**: Smooth vertical list scrolling.

---

## ⚙️ Configuration

TFSV conforms to the XDG Base Directory specification. Configuration files reside at:

```
~/.config/tfsv/settings.toml
```

### Example `settings.toml`

```toml
# Default startup directory (defaults to user home)
start_dir = "/home/user"

# Display dotfiles and hidden entries on startup
show_hidden = false

# UI color theme
theme_name = "default"

[openers]
# Custom opener commands for specific file categories
text = "nvim"          # Defaults to $EDITOR or 'nano'
image = "xdg-open"     # Image viewer handler
pdf = "xdg-open"       # PDF document reader
default = "xdg-open"   # Catch-all file opener
```

### Environment Overrides

| Variable | Description |
|:---|:---|
| `TFSV_CONFIG_DIR` | Custom directory containing `settings.toml` (useful for production or shared hosts) |
| `TFSV_INSTALL_DIR` | Target installation directory when running `install.sh` (e.g. `/opt/bin`) |
| `TFSV_SHARED_BIN_DIR` | Symlink target directory for multi-user root installations |

```bash
# Example: Use a dedicated configuration for a deployment
export TFSV_CONFIG_DIR="$HOME/.config/tfsv-prod"
tfsv
```

---

## 🔒 Security & Sandboxing

TFSV is engineered with safe defaults for multi-tenant and remote systems:

> [!IMPORTANT]  
> - **Default Home Confinement**: Under standard execution, TFSV prevents traversing outside `$HOME`. Attempts to escape via symlinks or path traversal outside your home tree are denied.  
> - **Unrestricted Mode**: To inspect or administer root file systems, run with administrative privileges:
>   ```bash
>   sudo "$(command -v tfsv)"
>   ```
> - **Shell State Isolation**: Exiting TFSV leaves your shell in its original working directory without unintended side effects.

---

## 🔤 Font Recommendations

TFSV renders file badges and indicators using **Nerd Fonts** (Devicons and Codicons). For the best visual experience, configure your terminal to use any Nerd Font:

- [FiraCode Nerd Font](https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts/FiraCode)
- [JetBrainsMono Nerd Font](https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts/JetBrainsMono)
- [MesloLGS NF](https://github.com/romkatv/powerlevel10k#meslo-nerd-font-patched-for-powerlevel10k)

---

## 🔄 Updating & Uninstallation

<details>
<summary><b>Click to expand update and uninstall instructions</b></summary>

### Update

Pull the latest commits and run the installer:

```bash
cd ~/Terminal-File-System-VIsualiser
git pull
./install.sh
```

### Uninstall

Remove the installed executable:

```bash
# For standard user install:
rm -f "$HOME/.local/bin/tfsv"

# For root install:
sudo rm -f /usr/local/bin/tfsv
```

*(Optional)* Remove the `export PATH=...` line added to your `~/.bashrc`, `~/.zshrc`, or `~/.profile`.

</details>

---

## 🏗️ Architecture

```
src/
├── app.rs            # Application state machine, TUI rendering & mouse handling
├── components/       # Reusable UI widgets (dialogs, context menus)
├── config/           # TOML serialization, default settings & XDG resolution
├── events.rs         # Crossterm keyboard & mouse event dispatcher
├── filesystem/       # Metadata inspection, directory traversal, safety checks & sorting
├── opener/           # External file opener detection and process execution
├── lib.rs            # Library interface
└── main.rs           # Terminal initialization, event loop & clean shutdown
```

---

## 📄 License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](LICENSE) file for complete details.

---

<div align="center">
  <sub>Crafted with ❤️ using Rust & Ratatui.</sub>
</div>
