# TFSV Progress Log

This document records the milestones completed for the TFSV project while preserving the original project specification in [TFSV TERMINAL FILE SYSTEM VISUALISER.md](TFSV%20TERMINAL%20FILE%20SYSTEM%20VISUALISER.md).

## Completed milestones

### 1. Project bootstrap
- Initialized the Rust project as a binary crate named `tfsv`.
- Created the default Cargo package configuration and binary entry point.
- Established the workspace structure required for the app and test layout.

### 2. Core application architecture
- Added the main application module and event loop foundation.
- Created the application state model for directory browsing, selection, refresh, and sorting.
- Added the initial terminal UI scaffolding with a TUI layout and text-based file list rendering.
- Added module boundaries for:
  - `app`
  - `events`
  - `config`
  - `filesystem`
  - `opener`
  - `transfer`
  - `search`
  - `ui`
  - `editor`

### 3. Filesystem and metadata layer
- Implemented directory loading and file discovery logic.
- Added file metadata classification with ASCII indicator labels such as `[DIR]`, `[RS]`, `[TXT]`, `[IMG]`, `[PDF]`, and `[EXE]`.
- Implemented sorting modes for name, type, and size.
- Added filesystem safety helpers to sanitize display strings and enforce ASCII-safe output.
- Added tests covering ASCII-only display behavior and escaping of non-ASCII filenames.

### 4. Configuration and extensibility
- Added XDG-style configuration scaffolding with `Settings` persistence in TOML.
- Added a default theme model for future UI customization.
- Added transfer manifest support and an SFTP hint helper for later workflow integration.
- Added opener detection and launcher scaffolding for external file handling.

### 5. Documentation and project readiness artifacts
- Created a project README with build and run instructions.
- Added architecture documentation and configuration notes under the docs folder.
- Added a project-level milestone record in this file so the implementation status remains visible without altering the original specification.

### 6. Verification status
- Verified the project builds successfully with the stable Rust toolchain.
- Verified the ASCII safety tests pass.
- Current validation evidence:
  - `cargo test --quiet` → 2 passed, 0 failed
  - `cargo build --quiet` → successful build

## Current state summary
TFSV is a working terminal file browser with filesystem operations, keyboard and mouse navigation, external file opening, visible file-type icons, and a text-only interface. Unicode filenames and icons are displayed while terminal control characters in names are escaped. Normal traversal is bounded to the launching user's home directory; `sudo` launches can traverse the full filesystem. The original specification remains unchanged.

### 7. File operations milestone
- Added real filesystem actions for create directory, rename, delete, and copy.
- Wired keyboard handlers for create (`n`), rename (`m`), and delete (`d`).
- Added regression tests covering directory creation, rename/delete behavior, and recursive copy operations.
- Validation evidence: `cargo test --quiet` passed with 3 tests passing in the new operation suite.

### 8. Keyboard and mouse navigation milestone
- Added page navigation with `PageUp`, `PageDown`, `Home`, and `End` support.
- Improved selection behavior with direct index selection and clamping.
- Added mouse click selection support in the file list area.
- Added unit tests covering page stepping and selection clamping behavior.
- Validation evidence: `cargo test --quiet` passed with the navigation regression tests included.

### 9. Context menu and dialog milestone
- Added a context menu state model with open/close, selection movement, and action dispatch.
- Wired keyboard interactions for opening the menu (`c`), moving through actions (`j`/`k`/Up/Down), and confirming (`Enter`).
- Added a modal-style dialog and menu rendering in the app UI.
- Added regression tests covering menu toggle and selection movement behavior.
- Validation evidence: `cargo test --quiet` passed with the new context-menu tests included.

### 10. External application integration milestone
- Added real file-open dispatch via the configured editor or default opener.
- Resolved text files to the `EDITOR` environment variable when available and split command arguments safely before launching them.
- Kept file paths as separate OS arguments rather than interpolating filenames into shell strings.
- Wired the context-menu Open action to use the real launcher path for both files and directories.
- Added regression tests for editor detection and command splitting.
- Validation evidence: `cargo test --quiet` passed with the external opener tests included.

### 11. Linux permission-aware safety validation milestone
- Hardened filesystem checks to reject symlinks and special-file types before normal file operations are attempted.
- Added explicit read-access validation for files and directories using Unix permission bits.
- Prevented copy/delete flows from following symlinks or treating device/socket/FIFO paths as ordinary files.
- Added regression tests covering symlink rejection and missing-file readability checks.
- Validation evidence: `cargo test --quiet` passed with the safety validation tests included.

### 12. Multi-user server readiness and deployment packaging milestone
- Added explicit configuration-directory overrides via `TFSV_CONFIG_DIR` so different users or deployments do not share the same settings state.
- Kept the config logic compatible with standard XDG behavior while allowing a deployment-specific override for hosted or SSH environments.
- Documented SSH and multi-user usage in the project README so the app is easier to run under authenticated Linux accounts.
- Validation evidence: `cargo test --quiet` passed with the config override regression included.

### 13. Unicode text display and compact file-type icons milestone
- Preserved Unicode filenames and visible Unicode type icons while continuing to escape control characters that could affect terminal output.
- Added language-specific icons for recognized code formats and compact shared music (`♫`) and video (`▶`) indicators across common formats.
- Kept the interface terminal-text based without image or background artwork.

### 14. Navigation and pointer interaction polish milestone
- Fixed Enter-based directory traversal and kept directory opening inside TFSV rather than sending directories to an external opener.
- Added mouse capture, hover highlighting, row selection, double-click directory traversal, right-click context menu access, and wheel scrolling.
- Kept the selected row visible while navigating and made the context menu overlay near the selected entry, clearing its area before drawing.
- Added regression coverage for pointer traversal and context-menu positioning.

### 15. Home boundary, breadcrumb, and launch behavior milestone
- Confined normal traversal to the canonical user home directory, including protection against directory symlinks that resolve outside it.
- Added a clickable home-relative breadcrumb beginning with the username rather than `/home/username`; breadcrumb hit-testing accounts for Unicode display width.
- Started in the launch-time working directory when it is within the allowed root, then used a valid configured start directory, falling back to the root.
- Allowed unrestricted traversal when launched under `sudo` and kept the invoking shell's working directory unchanged when TFSV exits.
- Added regression coverage for the home boundary, breadcrumb navigation, symlink escape rejection, and startup path selection.
- Fixed a breadcrumb compilation error by providing an explicit empty-path fallback and removed an unreachable duplicate left-click match arm.
- Validation evidence: `cargo build --release --bin tfsv` passed with exit code 0; `cargo test --no-run` passed and compiled all test targets. Full test execution did not return a completion result in the terminal integration, so the suite is not recorded as passing.

### 16. Clickable Places sidebar milestone
- Replaced the static left-pane list with clickable Home, Desktop, Documents, Downloads, Music, Pictures, Videos, Public, Templates, and Trash destinations.
- Marked unavailable destinations as disabled and highlighted the current place; exposed File System when traversal is unrestricted under `sudo`.
- Reused the canonical navigation boundary for sidebar destinations and added regression coverage for folder rows and navigation.
- Validation evidence: full `cargo test --quiet` passed: 15 unit tests, 2 filesystem tests, and 4 operation tests; exit code 0.

### 17. Copy, cut, and paste milestone
- Added an in-app file clipboard with Copy and Cut staging plus Paste into the current directory.
- Added context-menu actions and `Ctrl+C`, `Ctrl+X`, and `Ctrl+V` shortcuts.
- Added guarded move support that rejects unsafe paths and existing destinations, using copy/delete fallback when rename fails.
- Preserved the clipboard after failed pastes and cleared it after successful cut-paste.
- Validation evidence: full `cargo test --quiet` passed: 16 unit tests, 2 filesystem tests, and 5 operation tests; exit code 0.

### 18. Header quick-action ribbon milestone
- Added a compact, text-only action row under the app title with New, Copy, Cut, Paste, Rename, Delete, Refresh, and Sort buttons.
- Wired mouse hit-testing and dispatch to the same file operations used by keyboard and context-menu actions.
- Added coverage for every toolbar button's hit target and row bounds.
- Validation evidence: full `cargo test --quiet` passed: 17 unit tests, 2 filesystem tests, and 5 operation tests; exit code 0.

### 19. Non-emoji official icon glyph milestone
- Replaced emoji file badges with Nerd Fonts Devicons for language logos and Codicons for common file categories.
- Kept icons as single-cell text glyphs and documented the Nerd Font terminal prerequisite.
- Validation evidence: full `cargo test --quiet` passed: 17 unit tests, 2 filesystem tests, and 5 operation tests; exit code 0.

### 20. Linux installation documentation milestone
- Added Rust/Cargo prerequisites and a user-local `cargo install --path .` setup flow to the README.
- Documented how to launch, update, and uninstall TFSV, plus the optional Nerd Font requirement for file-type logos.

### 21. Clone-to-use guide and terminal restoration milestone
- Expanded the README with the verified GitHub clone URL, user-local installation, PATH setup, `tfsv` launch command, first-use navigation/actions, update/uninstall, and sudo traversal instructions.
- Added an RAII terminal session guard so raw mode, mouse capture, and the alternate screen are restored on normal exit and error returns.
- Suspend the TUI before external editors/viewers run, restore it afterward, and clear/redraw the screen so child output does not remain over the interface.
- Added best-effort cleanup at the process error boundary and rollback for failed terminal mode transitions.
- Validation evidence: full `cargo test --quiet` passed (17 unit, 2 filesystem, 5 operation tests); `cargo build --release --bin tfsv` completed successfully.

### 22. Post-clone installer milestone
- Added executable `install.sh` to build the release binary and install it as `~/.local/bin/tfsv`.
- Added idempotent PATH setup for the detected shell startup file; the app is a real executable named `tfsv`, not a shell alias.
- Added stable-toolchain selection guidance without automatically starting rustup downloads.
- Updated the README with the clone/install flow and accurate update/uninstall instructions.
- Validation: shell syntax and editor diagnostics pass. Full tests and release build passed before the installer-only follow-up. Isolated installer execution could not complete in this environment because rustup has no default toolchain selected; README documents `rustup default stable` as the one-time setup.

## Next milestones to target
Wait for user approval before starting another milestone.
