# TFSV
# TERMINAL FILE SYSTEM VISUALISER
## Complete Product, UI/UX, Architecture, Security and Engineering Specification

You are the lead Rust systems engineer, terminal UI developer, software architect and security engineer responsible for building TFSV (Terminal File System Visualiser).

Your task is to develop a complete, polished, lightweight and reliable terminal-native file manager for shared Linux servers.

This is a real software engineering project, not a UI mockup or proof of concept.

You must create the source code, implement functionality, compile the application, test it, resolve errors and provide complete documentation.

Do not stop after producing a plan or architecture proposal.

---

# 1. OFFICIAL PROJECT IDENTITY

| Property | Value |
|---|---|
| Official name | Terminal File System Visualiser |
| Abbreviation | TFSV |
| Rust package | tfsv |
| Executable | tfsv |
| Launch command | tfsv |
| Future Debian/APT package | tfsv |
| Initial installation path | /usr/local/bin/tfsv |
| Future Debian package path | /usr/bin/tfsv |

Application header:

```text
TFSV - Terminal File System Visualiser
```

Use `tfsv` consistently for:

- Cargo package and binary.
- Application identity.
- Build artifacts.
- CLI command.
- Configuration directory.
- Cache directory.
- Documentation.
- Installation instructions.
- Future Debian package metadata.

Do not use the previous name Server Explorer or the executable `server-explorer` in newly created files.

---

# 2. PRODUCT VISION

TFSV is a terminal-native file manager inspired by the organization and usability of GNOME Files (Nautilus).

It provides a structured file-management experience inside an existing SSH terminal.

The application should feel like a polished terminal desktop utility rather than a traditional command-line file browser.

## Core principles

- Clean and professional interface.
- Strict ASCII-only visual presentation.
- Full keyboard interaction.
- Full mouse interaction.
- Lightweight resource consumption.
- Safe filesystem operations.
- Native Linux permission enforcement.
- Independent multi-user sessions.
- External application integration.
- No unnecessary background services.
- No client-side TFSV installation.
- One standalone executable.

## TFSV is NOT

- A web-based file manager.
- A browser application.
- A graphical desktop environment.
- A shell replacement.
- A Midnight Commander clone.
- An SSH server.
- An SFTP server.
- A permanently running daemon.

Users manually launch TFSV from an existing SSH session.

The interface may take inspiration from Nautilus, but interaction should work naturally in a terminal using either keyboard or mouse.

---

# 3. TECHNOLOGY STACK

Use:

- Rust stable.
- Ratatui for terminal rendering.
- Crossterm for keyboard events, mouse events, terminal control and terminal management.
- Rust standard library for filesystem operations.
- Serde for serialization.
- TOML for configuration.
- thiserror for structured error handling or an equivalent lightweight solution.
- XDG-compliant configuration and cache path handling.

Additional crates are permitted when they provide a clear technical benefit.

## Architectural constraints

- Prefer synchronous filesystem operations initially.
- Do not introduce Tokio without a demonstrated requirement.
- No database.
- No web server.
- No background daemon.
- No unnecessary IPC.
- No unnecessary threads.
- No Go/Rust hybrid runtime.
- Keep the application as one standalone executable.

Use idiomatic, maintainable Rust.

---

# 4. PROJECT STRUCTURE

Use this as the initial architectural guide:

```text
tfsv/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── events.rs
│   ├── identity/
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── filesystem/
│   │   ├── mod.rs
│   │   ├── browser.rs
│   │   ├── operations.rs
│   │   ├── safety.rs
│   │   ├── metadata.rs
│   │   └── sorting.rs
│   ├── transfer/
│   │   ├── mod.rs
│   │   ├── manifest.rs
│   │   └── sftp.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── settings.rs
│   │   └── theme.rs
│   ├── opener/
│   │   ├── mod.rs
│   │   ├── detection.rs
│   │   └── launcher.rs
│   ├── editor/
│   │   └── mod.rs
│   ├── search/
│   │   └── mod.rs
│   └── ui/
│       ├── mod.rs
│       ├── layout.rs
│       ├── header.rs
│       ├── ribbon.rs
│       ├── sidebar.rs
│       ├── breadcrumbs.rs
│       ├── file_list.rs
│       ├── file_grid.rs
│       ├── dialogs.rs
│       ├── context_menu.rs
│       └── footer.rs
├── tests/
│   ├── filesystem_tests.rs
│   ├── safety_tests.rs
│   ├── transfer_tests.rs
│   ├── display_tests.rs
│   └── interaction_tests.rs
└── docs/
    ├── architecture.md
    ├── deployment.md
    ├── keyboard-shortcuts.md
    └── configuration.md
```

This is a suggested structure, not a requirement to create empty files.

Only create modules that have a clear responsibility.

There must be no preview module.

Responsibilities:

- `filesystem`: browsing, metadata and filesystem operations.
- `transfer`: transfer manifests and SFTP instructions.
- `opener`: external application detection and launching.
- `editor`: terminal editor integration.
- `config`: persistent settings and themes.
- `ui`: rendering and layout.
- `events`: input event handling and dispatch.
- `app`: shared application state and action coordination.

Keep responsibilities separate.

---

# 5. TARGET ENVIRONMENT

Primary environment:

- Debian 13.
- Linux x86_64.
- Linux ARM64 as a secondary target.
- OpenSSH server.
- Local Linux user accounts.
- Home directories commonly under `/home/username`.

The server may contain approximately 1,000 user accounts.

Multiple users must be able to launch independent TFSV sessions simultaneously.

Supported client environments include:

- Windows Terminal.
- PowerShell.
- Linux terminals.
- macOS terminals.
- Other SSH-compatible terminal applications.

The client device does not need TFSV installed.

Only the remote server needs the executable.

---

# 6. STRICT ASCII-ONLY DISPLAY

This is mandatory.

Every visible character rendered by TFSV must be standard printable ASCII.

Allowed visible range:

```text
0x20 - 0x7E
```

Terminal control sequences may be used internally but must never appear as visible content.

## Forbidden display characters

Never render:

- Unicode characters.
- Emoji.
- Nerd Font icons.
- Unicode box-drawing characters.
- Unicode arrows.
- Unicode file icons.
- Braille characters.
- Unicode progress indicators.
- Decorative non-ASCII typography.
- Non-ASCII separators.

This applies to:

- Header.
- Action ribbon.
- Sidebar.
- Breadcrumbs.
- File list.
- Grid.
- Dialogs.
- Context menus.
- Status messages.
- Error messages.
- Progress indicators.
- Help screens.
- Keyboard shortcut bar.

Use ASCII characters such as:

```text
+ - | : . > < [ ] ( ) / \ = _ #
```

## Illustrative interface

```text
+----------------------------------------------------------------+
| TFSV - Terminal File System Visualiser          USER: saumya   |
+----------------------------------------------------------------+
| File   Edit   View   Navigate   Tools   Help                   |
+----------------------------------------------------------------+
| Home / Projects / TFSV / src                                   |
+----------------------+-----------------------------------------+
| PLACES               | NAME          TYPE       SIZE           |
|                      |                                         |
| > Home               | [DIR] src      Directory  --            |
|   Documents          | [DIR] tests    Directory  --            |
|   Downloads          | [RS] main.rs   Rust       4.2 KB        |
|   Pictures           | [MD] README.md Markdown  2.1 KB         |
|                      | [IMG] photo.png Image     1.4 MB        |
| BOOKMARKS            | [PDF] notes.pdf PDF       800 KB        |
|   Projects           | [TXT] notes.txt Text      1.2 KB        |
|   Work               |                                         |
+----------------------+-----------------------------------------+
| F1 Help  F2 Rename  F5 Download  F6 Upload  F10 Quit           |
+----------------------------------------------------------------+
```

This is only a design reference.

Build a polished, responsive and visually balanced interface.

## Filename rendering

Linux filenames may contain Unicode, invalid UTF-8 bytes and control characters.

Do not corrupt, rename or reject valid filesystem entries because they cannot be displayed directly.

Maintain two separate representations:

1. Original filesystem path.
2. ASCII-safe display representation.

Never use the display representation as the actual filesystem path.

Example:

Original filename:

```text
report-✓.txt
```

Displayed filename:

```text
report-\xE2\x9C\x93.txt
```

Requirements:

- Escape non-ASCII bytes.
- Escape terminal control characters.
- Prevent terminal escape-sequence injection.
- Handle invalid UTF-8 filenames on Unix.
- Handle long filenames.
- Preserve distinguishability.
- Allow inspection of the complete escaped filename.
- Preserve the original path for all operations.

On Unix, use appropriate `OsStr` and `OsStrExt` handling rather than assuming every filename is UTF-8.

Add tests verifying ASCII-only visible output.

---

# 7. ASCII FILE-TYPE INDICATORS

Use compact ASCII labels instead of graphical icons.

Suggested mappings:

| File type | Indicator |
|---|---|
| Directory | [DIR] |
| Rust | [RS] |
| C | [C] |
| C++ | [CPP] |
| Python | [PY] |
| Java | [JAVA] |
| JavaScript | [JS] |
| TypeScript | [TS] |
| HTML | [HTML] |
| CSS | [CSS] |
| JSON | [JSON] |
| TOML | [TOML] |
| YAML | [YAML] |
| Markdown | [MD] |
| Text | [TXT] |
| PDF | [PDF] |
| Image | [IMG] |
| Video | [VID] |
| Audio | [AUD] |
| Archive | [ARC] |
| Shell | [SH] |
| Executable | [EXE] |
| Symbolic link | [LINK] |
| Unknown | [FILE] |

Requirements:

- Consistent classification.
- Directories always distinguishable.
- Executable files distinguishable.
- Symbolic links distinguishable.
- Hidden files supported.
- Extensionless files supported.
- Unknown files use [FILE].
- File metadata may supplement extension classification.
- Labels consistent in list and grid views.

Do not depend on installed fonts.

---

# 8. USER INTERFACE AND LAYOUT

Build a Nautilus-inspired terminal layout.

Do not copy Midnight Commander.

## Main layout

Use two primary columns:

1. Sidebar.
2. Main file browser.

The overall layout includes:

1. Header.
2. Action ribbon.
3. Breadcrumb navigation.
4. Sidebar.
5. Main file area.
6. Bottom keyboard shortcut bar.

There is no preview panel.

## 8.1 Header

Display:

- TFSV identity.
- Current Linux username.
- Current location.
- SSH connection information when available.
- Current view mode.

## 8.2 Action ribbon

Provide:

- New Folder.
- New File.
- Copy.
- Cut.
- Paste.
- Rename.
- Delete.
- Search.
- Refresh.
- Properties.
- View Switch.

All visible actions must be clickable and have keyboard equivalents.

Use concise ASCII labels.

Adapt to narrow terminals without overlapping controls.

## 8.3 Breadcrumb navigation

Example:

```text
Home / Projects / TFSV / src
```

Support:

- Parent navigation.
- Navigation to breadcrumb ancestors.
- Current directory highlighting.
- Long path handling.
- Back and forward navigation history.

## 8.4 Sidebar

Include:

```text
PLACES

Home
Desktop
Documents
Downloads
Pictures
Videos
Music

BOOKMARKS

User-defined bookmarks

LOCATIONS

Filesystem root
Accessible mounted locations
```

Requirements:

- Do not enumerate every user's home directory.
- Do not expose inaccessible directories as usable.
- Do not scan the whole filesystem.
- Persist bookmarks.
- Allow sidebar visibility to be toggled.

## 8.5 Detailed list view

Columns:

- Name.
- Type.
- Size.
- Modified.
- Permissions when space permits.

Support:

- Selection highlighting.
- Sorting by name.
- Sorting by size.
- Sorting by type.
- Sorting by modification time.
- Ascending and descending order.
- Directory distinction.
- Hidden file distinction.
- Symbolic link distinction.
- Responsive column sizing.
- Long filename handling.

## 8.6 Grid view

Provide an ASCII-based grid.

Each item displays:

- ASCII file-type indicator.
- Filename.
- Optional compact metadata.

Support keyboard navigation, mouse interaction, selection, directory opening and responsive columns.

## Default behaviour

- Detailed list view is the initial default.
- Grid view is available as an alternative.
- View preference is persistent.
- Both views share the same directory data and selection state.

---

# 9. MOUSE INTERACTION - FIRST-CLASS FEATURE

Mouse support is mandatory in the first usable release.

TFSV must provide mouse interaction comparable to Midnight Commander, while retaining its own Nautilus-inspired interface.

Use Crossterm mouse events and Ratatui layout geometry for hit-testing.

Do not implement mouse support as an unrelated or optional subsystem.

## 9.1 Supported mouse events

Implement:

- Left click.
- Double left click.
- Right click.
- Mouse wheel up.
- Mouse wheel down.
- Mouse movement where useful.
- Drag selection where practical.

Enable mouse reporting through Crossterm.

## 9.2 Main file browser

- Single left click selects an item.
- Double left click opens a directory.
- Double left click on a file launches its configured external application.
- Right click opens a context menu for the clicked item.
- Clicking an already selected item must not unexpectedly destroy a multi-selection.
- Clicking empty space clears selection where appropriate.
- Mouse selection and keyboard selection must remain synchronized.

## 9.3 Multi-selection

Support:

- Ctrl+click to toggle an item.
- Shift+click to select a range.
- Single click to select.
- Keyboard-based selection.
- Select all.
- Clear selection.

Selection state must be shared between mouse and keyboard interactions.

Display the number of selected items.

Do not let mouse selection trigger unintended file opening.

## 9.4 Sidebar

- Left click navigates to a location.
- Right click opens relevant bookmark or location actions.
- Clicking bookmarks navigates to them.
- Support sidebar scrolling.
- Allow sidebar visibility to be toggled.

## 9.5 Action ribbon

Every visible action must be clickable.

Examples:

- New Folder.
- New File.
- Copy.
- Cut.
- Paste.
- Rename.
- Delete.
- Search.
- Refresh.
- Properties.
- View switch.

Mouse and keyboard must invoke the same underlying application actions.

Do not duplicate business logic.

## 9.6 Breadcrumbs

- Clicking a breadcrumb navigates to that ancestor.
- Each breadcrumb must have a correct clickable region.
- Long breadcrumbs must remain usable.
- Clicking a breadcrumb must not trigger adjacent segments.

## 9.7 Context menus

Right-clicking a file or directory should display an ASCII-only context menu.

Suggested actions:

- Open.
- Open With.
- Copy.
- Cut.
- Rename.
- Delete.
- Properties.
- Download.

Only show actions applicable to the item and current permissions.

Context menu requirements:

- Mouse selection.
- Keyboard navigation.
- Escape to close.
- Close when clicking outside.
- Correct positioning within terminal boundaries.
- No accidental activation of underlying elements.

## 9.8 Scrolling

Mouse wheel must scroll the panel under the pointer.

Support:

- Main list.
- Grid view.
- Sidebar.
- Context menus.
- Scrollable dialogs.

Keep the selected item and viewport synchronized.

## 9.9 Mouse hit-testing

Maintain accurate layout rectangles or equivalent geometry for:

- Header.
- Action ribbon.
- Breadcrumbs.
- Sidebar.
- Main file list.
- Grid.
- Context menus.
- Dialogs.
- Footer.

Recalculate hit areas whenever the terminal is resized.

Do not hardcode fixed screen coordinates.

Clicks outside interactive elements must not trigger unrelated actions.

## 9.10 Terminal compatibility

- Enable supported terminal mouse reporting through Crossterm.
- Handle unavailable mouse reporting gracefully.
- Restore terminal and mouse modes on exit.
- Suspend mouse reporting appropriately when launching external applications.
- Restore mouse reporting when returning to TFSV.

If mouse support is unavailable, keyboard functionality must remain complete.

## 9.11 Testing

Test:

- Left-click selection.
- Double-click navigation.
- Double-click external opening.
- Right-click context menus.
- Mouse wheel scrolling.
- Sidebar navigation.
- Action ribbon clicks.
- Breadcrumb navigation.
- Ctrl+click multi-selection.
- Shift+click range selection.
- Terminal resize and hit-testing.
- Mouse mode restoration.

Mouse support is part of the first usable release, not a future enhancement.

---

# 10. FILE MANAGEMENT

## Navigation

- Open directories.
- Parent directory.
- Home directory.
- Back.
- Forward.
- Refresh.
- Enter a path.
- Handle permission errors.

## File operations

- Create directory.
- Create empty file.
- Rename.
- Copy.
- Cut.
- Paste.
- Delete.
- Multi-select.
- Select all.
- Invert selection.
- Properties.

## Copy, cut and paste

Implement an application-local clipboard per TFSV process.

- Copy creates a copy at the destination.
- Cut followed by paste moves the item.
- Pasting into the same directory is handled safely.
- Preserve original paths.
- Clear cut state after successful move.
- Never share clipboard state between users or processes.

## Safe operations

Never silently overwrite an existing destination.

When a conflict occurs, offer:

- Skip.
- Rename incoming item.
- Cancel.

Before deletion:

- Display confirmation.
- Show affected item names.
- Explain whether recursive deletion is involved.
- Require explicit confirmation.

## Filesystem safety

- Use `symlink_metadata` where appropriate.
- Never recursively follow symbolic links by default.
- Delete symbolic links themselves, not their targets.
- Avoid accidental recursive deletion.
- Validate paths.
- Handle special files safely.
- Never treat sockets, devices or FIFOs as ordinary files.
- Preserve permissions where appropriate.
- Handle interrupted operations safely.
- Report partial failures accurately.

Use Rust filesystem APIs.

Do not execute shell commands for normal filesystem operations.

Document unavoidable filesystem race limitations.

---

# 11. SEARCH AND SORTING

Implement directory-scoped filename search.

Requirements:

- Search current directory.
- Filter as the user types.
- Case-insensitive matching.
- Clear search quickly.
- Preserve underlying directory data.
- Highlight matching names using ASCII-safe styling.

Recursive search is optional future functionality.

Do not scan the entire filesystem at startup.

For large directories:

- Keep memory bounded.
- Do not read file contents.
- Use viewport-based rendering.
- Avoid unnecessary metadata calls.
- Keep interaction responsive.

---

# 12. FILE PROPERTIES

There is no embedded preview system.

Provide a Properties dialog displaying:

- Name.
- Full path.
- Type.
- Extension.
- Size.
- Created time if available.
- Modified time.
- Accessed time where available.
- Permissions.
- Owner.
- Group.
- Symbolic link target.
- Executable status.

Unavailable metadata must be shown as unavailable, not treated as a fatal error.

On Linux, use suitable Unix metadata APIs for UID, GID and permissions.

---

# 13. NO EMBEDDED PREVIEWS

Do not implement:

- Image previews.
- Image thumbnails.
- PDF previews.
- Video previews.
- Audio visualizations.
- Embedded document previews.
- File-content preview panes.
- Image rendering inside the terminal.

Do not create a preview module.

Do not reserve screen space for previews.

Use the available terminal space for the file browser.

Files are opened externally when appropriate.

---

# 14. EXTERNAL APPLICATION INTEGRATION

TFSV must support opening files using applications installed on the server.

Examples:

| File type | Suggested opener |
|---|---|
| Image | Gwenview |
| PDF | xdg-open or configured PDF viewer |
| Text | $EDITOR |
| Video | xdg-open |
| Audio | xdg-open |
| Other | Configured default opener |

Gwenview is a preferred example, not a mandatory dependency.

## Opener configuration

Use structured command configuration.

Example:

```toml
[openers.image]
command = "gwenview"
args = []
append_paths = true

[openers.pdf]
command = "xdg-open"
args = []
append_paths = true

[openers.default]
command = "xdg-open"
args = []
append_paths = true
```

Requirements:

- Detect executable availability.
- Allow user-defined openers.
- Open selected files.
- Support multiple selected paths where the application supports them.
- Pass paths as separate process arguments.
- Never interpolate filenames into shell commands.
- Launch under the authenticated Linux user.
- Handle missing applications.
- Display clear errors.
- Do not claim success before process creation succeeds.

Text files should use the dedicated terminal editor integration.

---

# 15. GRAPHICAL APPLICATIONS OVER SSH

TFSV runs remotely through SSH.

Launching Gwenview on the server does not automatically make it appear on the user's computer.

A working graphical forwarding mechanism is required.

## Initial support

Prioritize SSH X11 forwarding.

Consider:

- `DISPLAY`.
- `XAUTHORITY`.
- SSH connection environment.
- X11 authorization availability.
- Installed graphical application binaries.

Example:

```bash
ssh -X username@server
```

Or, where appropriate:

```bash
ssh -Y username@server
```

Trusted forwarding should only be recommended when the user understands the security implications.

Do not assume `DISPLAY` alone guarantees a usable display.

Do not assume `WAYLAND_DISPLAY` means graphical forwarding works over ordinary SSH.

Wayland support can be added later only if a real supported forwarding mechanism is available.

If graphical launching is unavailable, display:

```text
Graphical display is unavailable.

To use graphical applications over SSH,
connect with X11 forwarding if supported.

Example:

ssh -X username@server
```

Requirements:

- TFSV continues functioning normally.
- Do not modify SSH configuration.
- Do not install GUI applications automatically.
- Do not launch applications on an unrelated server display.
- Report launch failures clearly.

---

# 16. EXTERNAL TERMINAL EDITOR

Support:

1. `$EDITOR`.
2. nano.
3. vi.

Requirements:

- Suspend the TUI.
- Restore terminal mode before launching.
- Launch as the authenticated user.
- Wait for editor exit.
- Reinitialize the TUI.
- Refresh the current directory.

Do not assume a graphical editor is available.

Handle missing editors gracefully.

---

# 17. SFTP TRANSFER WORKFLOW

Users need to transfer files between the remote server and their local computer.

TFSV runs on the server and cannot independently browse the local computer's filesystem merely because it knows the SSH client's IP address.

Do not assume:

- Client has a public IP.
- Client runs an SFTP server.
- Server can initiate a connection into the client filesystem.
- Client has TFSV installed.

Never transfer private SSH keys to the server.

## Chosen architecture: host-initiated OpenSSH SFTP

The local computer initiates the transfer using its existing OpenSSH client and authentication setup.

TFSV prepares transfer instructions and a manifest.

The user executes the transfer locally.

This is the supported initial-release transfer model.

## 17.1 Download

The user selects remote files and chooses Download.

TFSV must:

1. Collect original remote paths.
2. Validate selections.
3. Generate a transfer manifest.
4. Prepare SFTP instructions.
5. Display an ASCII-only command block.
6. Explain that execution happens on the local device.

Example:

```bash
sftp username@your-server
lcd ~/Downloads
get /home/username/Projects/example.zip
```

Directory example:

```text
get -r /home/username/Projects/example
```

## 17.2 Upload

The user chooses Upload.

TFSV must:

1. Let the user choose a remote destination directory.
2. Display the destination.
3. Generate local SFTP instructions.
4. Explain that the user executes them locally.

Example:

```bash
sftp username@your-server
cd /home/username/Projects
lcd ~/Documents
put -r example-project
```

## 17.3 Path handling

SFTP command syntax is not identical to shell syntax.

Implement correct quoting and escaping for the supported OpenSSH SFTP command format.

Test filenames containing:

- Spaces.
- Quotes.
- Backslashes.
- Unicode.
- Special characters.
- Newlines.

Never generate a potentially destructive command for a path that cannot be represented safely.

Instead:

- Explain the limitation.
- Provide a safe alternative.
- Preserve the original path in the manifest.

Do not silently overwrite destination files.

## 17.4 SSH compatibility

Preserve compatibility with:

- SSH config aliases.
- Custom SSH ports.
- SSH agent authentication.
- Existing key configuration.
- ProxyJump configurations.
- Standard OpenSSH SFTP.

Do not require users to copy private keys to the server.

## 17.5 Clipboard limitation

Do not assume a server-side TUI can access the local clipboard.

Display instructions in a selectable and copyable manner supported by the terminal.

OSC 52 clipboard integration may be added as an optional capability-aware enhancement.

Never claim clipboard copying succeeded without support.

Do not pretend transfers happen automatically.

---

# 18. KEYBOARD SHORTCUTS

Use consistent defaults.

| Shortcut | Action |
|---|---|
| Enter | Open selected item |
| Backspace | Parent directory |
| H | Toggle hidden files |
| Ctrl+L | Enter location |
| Ctrl+F | Search |
| Ctrl+N | New folder |
| Ctrl+Shift+N | New file |
| Ctrl+C | Copy |
| Ctrl+X | Cut |
| Ctrl+V | Paste |
| F2 | Rename |
| Delete | Delete |
| F5 | Download instructions |
| F6 | Upload instructions |
| F9 | Toggle sidebar |
| F10 | Quit |
| Ctrl+R | Refresh |
| Ctrl+1 | List view |
| Ctrl+2 | Grid view |
| Space | Toggle selection |
| Esc | Cancel or close dialog |

Important:

- Ctrl+C means Copy when the application is active.
- Do not use Ctrl+C as the normal quit action.
- Use F10 or Ctrl+Q to quit.
- Handle terminal key limitations gracefully.
- Display available shortcuts in the footer.

Bottom bar example:

```text
F1 Help  F2 Rename  F5 Download  F6 Upload  F9 Sidebar  F10 Quit
```

Mouse and keyboard actions must invoke the same underlying application commands.

---

# 19. THEMING

Support:

- Dark theme.
- Light theme.
- Automatic theme mode.
- Terminal color capability detection.

Automatic mode must not assume that the terminal background can always be detected.

Use explicit configuration as the reliable override.

Environment hints such as `COLORFGBG` may be advisory.

Requirements:

- Consistent visual hierarchy.
- Restrained colors.
- Clear selection highlighting.
- Readable labels.
- Limited-color fallback.
- ASCII-only visible content in every theme.

---

# 20. TERMINAL AND INPUT MANAGEMENT

The application must:

- Handle terminal resize.
- Restore terminal state on normal exit.
- Restore terminal state after errors.
- Restore terminal state after panics.
- Handle Ctrl+C safely.
- Handle Ctrl+D where appropriate.
- Support keyboard navigation.
- Support mouse interaction.
- Work in small terminals.
- Avoid flickering.
- Avoid unnecessary redraws.
- Avoid continuous polling.

Use Crossterm mouse capture and release appropriately.

Use a single event-processing architecture for keyboard, mouse and terminal resize events.

Do not create independent conflicting keyboard and mouse state systems.

When an external application is launched:

1. Suspend the TUI.
2. Disable mouse reporting as appropriate.
3. Restore normal terminal modes.
4. Launch the external process.
5. Wait for completion when appropriate.
6. Reinitialize terminal and mouse modes.
7. Refresh the file browser.

A panic or unexpected exit must not leave the terminal or mouse mode unusable.

---

# 21. MULTI-USER ARCHITECTURE

Install one central executable:

```text
/usr/local/bin/tfsv
```

Users launch manually:

```bash
ssh username@server
tfsv
```

## Identity

Resolve the current Linux user using OS account information and authenticated process identity.

Determine:

- UID.
- Primary GID.
- Supplementary groups.
- Username.
- Home directory.

Do not rely exclusively on `$USER` or `$HOME`.

## Permissions

TFSV runs as the authenticated user.

Never:

- Elevate privileges.
- Use sudo internally.
- Bypass Linux permissions.
- Perform operations as root.
- Change ownership without explicit authorization.

Linux permissions remain authoritative.

Do not proactively enumerate other users' private directories.

Do not claim that TFSV prevents access to files that Linux itself permits the current user to access.

## Session isolation

Every process has independent:

- Navigation history.
- Selection state.
- Clipboard.
- Configuration.
- Temporary state.

No shared mutable session state.

No global clipboard.

## Configuration paths

Use:

```text
~/.config/tfsv/
    config.toml
    bookmarks.toml
    keybindings.toml

~/.cache/tfsv/
```

Respect `XDG_CONFIG_HOME` and `XDG_CACHE_HOME`, falling back to the home directory when unset.

Create directories only when necessary.

## SSH environment

Optionally inspect:

- `SSH_CONNECTION`.
- `SSH_CLIENT`.
- `SSH_TTY`.

Use these for informational display only.

They are not authentication evidence.

---

# 22. PERFORMANCE

TFSV must remain lightweight on a shared server.

Requirements:

- One process per active session.
- No global daemon.
- No startup filesystem scan.
- No database.
- No unnecessary threads.
- No continuous polling.
- Lazy directory loading.
- Bounded memory.
- Efficient rendering.
- No unnecessary file-content loading.
- Avoid excessive metadata calls.
- Avoid blocking the UI unnecessarily.

Use viewport-based rendering for large directories.

Performance goals:

- Fast startup.
- Low idle CPU.
- Small memory footprint.
- Small release binary.
- Responsive navigation.

These are targets, not guaranteed claims. Measure actual resource usage.

Suggested release profile:

```toml
[profile.release]
lto = true
codegen-units = 1
strip = true
```

Do not compromise correctness or safety for minor performance improvements.

---

# 23. CONFIGURATION

Use persistent user-level TOML configuration.

Example:

```toml
[general]
show_hidden = false
default_view = "list"
confirm_delete = true

[appearance]
theme = "auto"

[navigation]
show_sidebar = true

[editor]
preferred = ""

[openers.image]
command = "gwenview"
args = []
append_paths = true

[openers.pdf]
command = "xdg-open"
args = []
append_paths = true

[openers.default]
command = "xdg-open"
args = []
append_paths = true
```

Requirements:

- Safe defaults.
- Graceful invalid configuration handling.
- Never unexpectedly overwrite settings.
- Create configuration only when required.
- Isolate configuration per user.
- Validate opener settings.
- Document all options.

---

# 24. SECURITY

Implement:

1. No privilege elevation.
2. No permission bypass.
3. No credential storage.
4. No private key transfer.
5. No arbitrary shell execution from filenames.
6. No silent overwrite.
7. No unconfirmed destructive operation.
8. No unexpected symlink traversal.
9. Safe path handling.
10. Special-file protection.
11. Terminal escape-sequence protection.
12. No network listener.
13. No SSH configuration modification.
14. No shell configuration modification.
15. No forced-command configuration.
16. No automatic launch on login.
17. No sensitive file-content logging.

Normal runtime must not require root.

Administrator privileges are needed only for central installation.

---

# 25. DEVELOPMENT MILESTONES

Implement in the following order.

## Milestone 0 - Repository inspection

Inspect the current directory.

If empty:

- Initialize a Rust Cargo project named `tfsv`.
- Create the necessary source structure.
- Create an initial README.

If existing files are present:

- Inspect before modifying.
- Preserve useful files.
- Do not delete unknown files.

If a previous Go MVP is present, use it only as a design reference. Do not blindly port broken code.

Create a concise implementation plan, then begin implementation.

## Milestone 1 - Working TUI foundation

Implement:

- Cargo project.
- Terminal initialization and cleanup.
- Ratatui rendering.
- Crossterm event loop.
- ASCII-only rendering.
- Safe filename display.
- Responsive layout.
- Header.
- Action ribbon.
- Sidebar.
- Breadcrumbs.
- Detailed file list.
- Bottom shortcut bar.
- Directory navigation.
- Mouse event capture.
- Mouse hit-testing.
- Clickable sidebar and breadcrumbs.
- Clickable action ribbon.
- Mouse selection.
- Double-click navigation.
- Right-click context menu.
- Mouse-wheel scrolling.
- Terminal resize handling.

The application must launch successfully and browse real directories using both keyboard and mouse.

## Milestone 2 - File management

Implement:

- Grid view.
- List view.
- Hidden files.
- Sorting.
- Multi-selection.
- Create folder.
- Create file.
- Rename.
- Copy.
- Cut.
- Paste.
- Delete confirmation.
- Properties.
- Mouse and keyboard interaction for every operation.

Add filesystem safety tests.

## Milestone 3 - Search and metadata

Implement:

- Filename search.
- Filtering.
- Metadata display.
- Large directory handling.
- Permission error handling.
- ASCII rendering tests.

## Milestone 4 - External applications

Implement:

- File-type detection.
- Configurable openers.
- Binary availability detection.
- Gwenview integration.
- PDF and other opener support.
- X11 forwarding awareness.
- Missing application handling.
- Process launch error handling.

Do not implement embedded previews.

## Milestone 5 - Editor integration

Implement:

- `$EDITOR`.
- nano fallback.
- vi fallback.
- Terminal suspension.
- Terminal restoration.
- Directory refresh after editor exit.

## Milestone 6 - SFTP workflow

Implement:

- Download manifest.
- Upload destination workflow.
- OpenSSH SFTP instructions.
- Correct SFTP path quoting.
- Safe command generation.
- Unsupported path handling.
- Transfer documentation.

## Milestone 7 - Configuration and polish

Implement:

- Dark theme.
- Light theme.
- Automatic theme mode.
- Persistent view preference.
- Bookmarks.
- Configuration management.
- Responsive layout.
- Mouse interaction polish.
- Help screen.
- Improved dialogs.

## Milestone 8 - Production validation

Run:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

Resolve relevant failures.

Validate:

- ASCII-only visible output.
- Keyboard navigation.
- Mouse navigation.
- Hit-testing.
- Terminal restoration.
- Mouse mode restoration.
- Permission handling.
- Concurrent sessions.
- Large directory behavior.
- Symlink safety.
- File operations.
- External application launching.
- SFTP instruction generation.
- Debian 13 compatibility.

## Milestone 9 - Debian packaging preparation

Prepare for future Debian packaging.

Package identity:

```text
Package: tfsv
Description: Terminal File System Visualiser
```

Target architectures:

```text
amd64
arm64
```

Prepare:

```text
debian/
    control
    rules
    changelog
    copyright
```

The future package should install:

```text
/usr/bin/tfsv
```

Do not create system services.

Do not modify SSH settings.

Do not claim APT availability until the package is published in a repository.

---

# 26. TESTING REQUIREMENTS

Use temporary directories.

## Filesystem tests

Test:

- Empty directories.
- Large directories.
- Hidden files.
- Unicode filenames.
- Invalid UTF-8 filenames on Unix.
- Filenames with spaces.
- Long filenames.
- Extensionless files.
- Unknown extensions.
- Symbolic links.
- Broken symbolic links.
- Permission-denied directories.
- Read-only files.
- Existing copy destinations.
- Recursive copy.
- Recursive delete confirmation.
- Interrupted operations.
- Special files.

## Display tests

Test:

- Unicode filenames.
- Control characters.
- Terminal escape sequences.
- Long filenames.
- Escaped filename uniqueness.
- Printable ASCII-only output.

## Mouse tests

Test:

- Left-click selection.
- Double-click navigation.
- Right-click context menus.
- Wheel scrolling.
- Sidebar clicks.
- Ribbon clicks.
- Breadcrumb clicks.
- Ctrl+click multi-selection.
- Shift+click range selection.
- Resize and hit-testing.
- Mouse restoration.

## External application tests

Test:

- Available opener.
- Missing opener.
- Invalid opener configuration.
- Paths with spaces.
- Multiple selected files.
- Missing DISPLAY.
- Invalid display environment.

## Transfer tests

Test:

- Spaces.
- Unicode.
- Quotes.
- Backslashes.
- Newlines.
- Nested directories.
- Unsupported path representations.
- Existing destinations.

Never run destructive tests against real user directories.

---

# 27. BUILD AND DEPLOYMENT

Target Debian 13.

Required targets:

```text
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu
```

Do not claim static linking unless verified.

Document runtime library requirements.

## Build

```bash
cargo build --release
```

## Initial installation

```bash
sudo install -m 0755 target/release/tfsv /usr/local/bin/tfsv
```

## Launch

```bash
ssh username@your-server
tfsv
```

## Uninstall

```bash
sudo rm /usr/local/bin/tfsv
```

Do not modify SSH configuration, shell startup files or system services.

Initial manual installation must work independently of future Debian packaging.

---

# 28. DOCUMENTATION

Create:

- README.md
- docs/architecture.md
- docs/deployment.md
- docs/keyboard-shortcuts.md
- docs/configuration.md

Document:

- Product overview.
- Features.
- Requirements.
- Supported platforms.
- Architecture.
- Build instructions.
- Installation.
- Usage.
- Keyboard shortcuts.
- Mouse controls.
- ASCII-only display policy.
- File management.
- External application integration.
- Gwenview and X11 forwarding.
- SFTP workflow.
- Security model.
- Multi-user behaviour.
- Known limitations.
- Troubleshooting.
- Future Debian packaging.

Only document features that actually exist.

---

# 29. OPENCODE IMPLEMENTATION RULES

1. Inspect the repository first.
2. Do not delete files without understanding them.
3. Create a concise implementation plan.
4. Begin implementation immediately.
5. Write actual working source code.
6. Use idiomatic Rust.
7. Keep modules focused.
8. Avoid unnecessary abstractions.
9. Avoid unnecessary dependencies.
10. Handle errors explicitly.
11. Add safety tests.
12. Run compilation and linting regularly.
13. Fix errors before proceeding.
14. Do not claim unimplemented features are complete.
15. Do not replace functionality with placeholder buttons.
16. Explain technical limitations honestly.
17. Keep documentation synchronized.
18. Do not ask for approval for routine engineering decisions.
19. Ask questions only when an essential requirement cannot be inferred.

## Required development behaviour

Do not stop after creating the plan.

Actually create source files, implement features, compile and test.

If the project cannot be completed in one session:

- Finish the current milestone in a working state.
- Record completed features.
- Record incomplete features.
- Record known issues.
- Update development documentation.
- Continue from the existing implementation.

---

# 30. FINAL ACCEPTANCE CRITERIA

## Identity

- Official name: Terminal File System Visualiser.
- Abbreviation: TFSV.
- Cargo package: tfsv.
- Executable: tfsv.
- Future Debian package: tfsv.

## Interface

- Strictly ASCII-only visible output.
- Nautilus-inspired layout.
- Persistent action ribbon.
- Sidebar.
- Breadcrumb navigation.
- Detailed list view.
- ASCII grid view.
- ASCII file-type indicators.
- Bottom shortcut bar.
- Responsive terminal layout.
- No embedded previews.
- No thumbnails.
- No Unicode icons.
- Full keyboard interaction.
- Full mouse interaction.

## Functionality

- Directory browsing.
- Hidden files.
- Sorting.
- Multi-selection.
- Create files and folders.
- Rename.
- Copy.
- Cut.
- Paste.
- Delete with confirmation.
- Search.
- Properties.
- Bookmarks.

## Mouse

- Single-click selection.
- Double-click opening.
- Right-click context menus.
- Wheel scrolling.
- Sidebar interaction.
- Breadcrumb interaction.
- Action ribbon interaction.
- Multi-selection.
- Correct hit-testing after terminal resizing.

## Integrations

- External terminal editor.
- Configurable external applications.
- Gwenview when graphical display is available.
- X11 forwarding detection.
- OpenSSH SFTP instructions.

## Server deployment

- Debian 13 support.
- Ordinary SSH operation.
- No client-side TFSV installation.
- Single central executable.
- Independent concurrent sessions.
- Native Linux permissions.
- No daemon.
- No network listener.
- No automatic privilege elevation.

## Reliability

- Safe filesystem operations.
- Safe filename rendering.
- Terminal restoration.
- Mouse mode restoration.
- Automated tests.
- Build documentation.
- Deployment documentation.
- Future Debian packaging preparation.

---

# 31. FINAL PRODUCT DIRECTION

TFSV should be a practical, polished and lightweight terminal file manager.

Its quality must come from:

- Excellent layout.
- Consistent ASCII typography.
- Clear information hierarchy.
- Intuitive keyboard navigation.
- Natural mouse interaction.
- Reliable filesystem operations.
- Useful metadata.
- External application integration.
- Low resource consumption.

Do not recreate a graphical desktop inside a terminal.

Do not introduce unnecessary visual effects.

Do not add embedded previews.

Do not make mouse support optional.

Prioritize usability, reliability, security and performance.

**Begin by inspecting the current directory. If it is empty, initialize the Rust project. Then implement Milestone 1 and deliver a working terminal file browser with both keyboard and mouse support. Do not stop at planning.**