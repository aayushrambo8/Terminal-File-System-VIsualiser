# TFSV

Terminal File System Visualiser is a terminal-native Linux file manager written in Rust.

## Overview

TFSV is designed to run inside an SSH session and provide a Nautilus-inspired workflow in a strict ASCII terminal UI. The project uses Ratatui and Crossterm to render a clean keyboard-driven file browser with safe filesystem handling.

## Build

```bash
cargo build
```

## Run

```bash
cargo run --bin tfsv
```

## Features

- ASCII-only terminal rendering
- Keyboard navigation
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
