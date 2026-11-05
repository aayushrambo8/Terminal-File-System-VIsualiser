# TFSV

Terminal File System Visualiser is a terminal-native Linux file manager written in Rust.

## Overview

TFSV is designed to run inside an SSH session and provide a terminal-native file browsing workflow. It uses Ratatui and Crossterm for keyboard and mouse interaction, with a text-only layout, Unicode filenames and type icons, and no image backgrounds.

## Install on Linux

Requirements: Git and Rust/Cargo 1.85 or newer. Install Rust with rustup if it is not already available:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Clone, install, and launch TFSV:

```bash
git clone https://github.com/aayushrambo8/Terminal-File-System-VIsualiser.git
cd Terminal-File-System-VIsualiser
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"
tfsv
```

To make the PATH change permanent, add `export PATH="$HOME/.cargo/bin:$PATH"` to your shell profile (`~/.profile`, `~/.bashrc`, or `~/.zshrc`). After installation, run `tfsv` from any directory. It starts in the launch directory if it is within your home directory; normal launches cannot navigate outside home.

To allow traversal outside home, run `sudo "$(command -v tfsv)"`. Exiting returns to the shell's original directory. Directory navigation happens inside TFSV.

## First use

- Click a folder in **Places**, or select a directory and press `Enter`/`l` to enter it. Press `h` or Left to go to its parent.
- Use the header ribbon or right-click for New, Copy, Cut, Paste, Rename, Delete, Refresh, and Sort.
- `Ctrl+C`, `Ctrl+X`, and `Ctrl+V` copy, cut, and paste the selected item. Press `q` to quit.
- Normal use is confined to your home directory. To allow traversal outside home, use `sudo "$(command -v tfsv)"`.
- If a text editor, image viewer, or another opener runs, TFSV leaves the alternate screen and restores its interface when the program exits. On errors, it also attempts to restore the normal terminal before printing diagnostics.

To update, enter the cloned repository, pull changes, and reinstall:

```bash
cd ~/Terminal-File-System-VIsualiser
git pull
cargo install --path . --force
```

To uninstall, run `cargo uninstall tfsv`.

The interface works in a regular text terminal. Select a Nerd Font (such as MesloLGS NF or FiraCode Nerd Font) to see file-type logos correctly; without one, some icons may appear as empty boxes.

## Build from source

```bash
cargo build
```

## Run

```bash
cargo run --bin tfsv
```

TFSV starts in the directory where it was launched when that directory is inside your home. Otherwise it uses the configured start directory if it is inside your home, falling back to your home directory. Normal launches cannot traverse outside your home. When launched with `sudo`, traversal is unrestricted. Exiting TFSV leaves the shell in its original working directory.

File-type badges use Nerd Fonts Devicons for language logos and Codicons for common file categories, not emoji. Set the terminal font to a Nerd Font (for example, MesloLGS NF or FiraCode Nerd Font) to render the glyphs correctly.

## Features

- Plain-text terminal rendering with Unicode filenames and icons
- Keyboard navigation
- Clickable Places sidebar with standard home folders
- Copy, cut, and paste from the context menu or `Ctrl+C`, `Ctrl+X`, and `Ctrl+V`
- Header quick-action ribbon for common file operations
- Safe path handling
- File type detection
- Terminal-native file browsing

## Configuration

Settings are stored in the XDG config directory under `tfsv/settings.toml`.

For multi-user or hosted environments, override the config path with:

```bash
export TFSV_CONFIG_DIR="$HOME/.config/tfsv-prod"
```

This keeps per-user or per-deployment settings isolated and avoids shared-state collisions on a shared server.

## SSH / multi-user usage

TFSV is designed for authenticated Linux sessions over SSH. Launch it under the user account that should own the file system session, and keep configuration isolated per user with `TFSV_CONFIG_DIR` when needed.
# Terminal-File-System-VIsualiser
