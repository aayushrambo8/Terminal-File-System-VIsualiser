use std::path::PathBuf;

use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table};
use ratatui::Terminal;

use crate::config::Settings;
use crate::events::{handle_key_event, handle_mouse_event};
use crate::filesystem::{create_directory as fs_create_directory, copy_path, delete_path, load_directory, rename_path, sort_entries, FileEntry, SortMode};

pub struct App {
    settings: Settings,
    current_dir: PathBuf,
    entries: Vec<FileEntry>,
    selected: usize,
    sort_mode: SortMode,
    status: String,
    context_menu_open: bool,
    context_menu_index: usize,
    context_menu_items: Vec<&'static str>,
    dialog_message: Option<String>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let settings = Settings::load()?;
        let current_dir = if settings.start_dir.exists() {
            settings.start_dir.clone()
        } else {
            PathBuf::from("/")
        };
        let mut app = Self {
            settings,
            current_dir,
            entries: Vec::new(),
            selected: 0,
            sort_mode: SortMode::Name,
            status: String::from("TFSV ready"),
            context_menu_open: false,
            context_menu_index: 0,
            context_menu_items: vec!["Open", "Rename", "Delete", "Copy", "Properties"],
            dialog_message: None,
        };
        app.refresh();
        Ok(app)
    }

    pub fn run_main() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = Self::new()?;
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|frame| app.render(frame))?;
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if handle_key_event(&mut app, key)? {
                        break;
                    }
                }
                Event::Mouse(mouse) => handle_mouse_event(&mut app, mouse),
                _ => {}
            }
        }

        disable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn refresh(&mut self) {
        let mut entries = load_directory(&self.current_dir).unwrap_or_default();
        sort_entries(&mut entries, self.sort_mode);
        self.entries = entries;
        self.selected = self.selected.min(self.entries.len().saturating_sub(1));
        self.status = format!("Browsing {}", self.current_dir.display());
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.entries.is_empty() {
            return;
        }
        let current = self.selected as isize;
        let next = current + delta;
        self.selected = next.clamp(0, self.entries.len() as isize - 1) as usize;
    }

    pub fn select_index(&mut self, index: usize) {
        if self.entries.is_empty() {
            return;
        }
        self.selected = index.min(self.entries.len().saturating_sub(1));
    }

    pub fn page_down(&mut self, lines: usize) {
        if self.entries.is_empty() {
            return;
        }
        let delta = lines.max(1) as isize;
        self.move_selection(delta);
    }

    pub fn page_up(&mut self, lines: usize) {
        if self.entries.is_empty() {
            return;
        }
        let delta = -(lines.max(1) as isize);
        self.move_selection(delta);
    }

    pub fn move_to_start(&mut self) {
        self.select_index(0);
    }

    pub fn move_to_end(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        self.selected = self.entries.len() - 1;
    }

    pub fn enter_selected(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let current = &self.entries[self.selected];
        if current.is_dir {
            self.current_dir = current.path.clone();
            self.selected = 0;
            self.refresh();
        }
    }

    pub fn go_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected = 0;
            self.refresh();
        }
    }

    pub fn cycle_sort(&mut self) {
        self.sort_mode = match self.sort_mode {
            SortMode::Name => SortMode::Type,
            SortMode::Type => SortMode::Size,
            SortMode::Size => SortMode::Name,
        };
        self.refresh();
    }

    pub fn create_directory(&mut self, name: &str) {
        let target = self.current_dir.join(name);
        if let Err(err) = fs_create_directory(&target) {
            self.status = format!("Create failed: {err}");
            return;
        }
        self.refresh();
        self.status = format!("Created directory {}", target.display());
    }

    pub fn rename_selected(&mut self, new_name: &str) {
        if self.entries.is_empty() {
            return;
        }
        let old_path = self.entries[self.selected].path.clone();
        match rename_path(&old_path, new_name) {
            Ok(_) => {
                self.refresh();
                self.status = format!("Renamed {} to {}", old_path.display(), new_name);
            }
            Err(err) => self.status = format!("Rename failed: {err}"),
        }
    }

    pub fn delete_selected(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let target = self.entries[self.selected].path.clone();
        match delete_path(&target) {
            Ok(_) => {
                self.refresh();
                self.status = format!("Deleted {}", target.display());
            }
            Err(err) => self.status = format!("Delete failed: {err}"),
        }
    }

    pub fn copy_selected(&mut self, new_name: &str) {
        if self.entries.is_empty() {
            return;
        }
        let source = self.entries[self.selected].path.clone();
        let target = self.current_dir.join(new_name);
        match copy_path(&source, &target) {
            Ok(_) => {
                self.refresh();
                self.status = format!("Copied {} to {}", source.display(), target.display());
            }
            Err(err) => self.status = format!("Copy failed: {err}"),
        }
    }

    pub fn open_selected(&mut self) {
        if self.entries.is_empty() {
            self.status = "No item selected to open".to_string();
            return;
        }

        let target = self.entries[self.selected].path.clone();
        if self.entries[self.selected].is_dir {
            self.enter_selected();
            self.status = format!("Opened directory {}", target.display());
            return;
        }

        match crate::opener::launch_file(&target) {
            Ok(_) => self.status = format!("Opened {}", target.display()),
            Err(err) => self.status = format!("Open failed: {err}"),
        }
    }

    pub fn toggle_context_menu(&mut self) {
        self.context_menu_open = !self.context_menu_open;
        if self.context_menu_open {
            self.context_menu_index = 0;
            self.dialog_message = None;
        }
    }

    pub fn is_context_menu_open(&self) -> bool {
        self.context_menu_open
    }

    pub fn close_context_menu(&mut self) {
        self.context_menu_open = false;
        self.dialog_message = None;
    }

    pub fn move_context_menu(&mut self, delta: isize) {
        if !self.context_menu_open || self.context_menu_items.is_empty() {
            return;
        }
        let next = (self.context_menu_index as isize + delta).clamp(0, self.context_menu_items.len() as isize - 1) as usize;
        self.context_menu_index = next;
    }

    pub fn apply_context_menu(&mut self) {
        if !self.context_menu_open {
            return;
        }
        let action = self.context_menu_items.get(self.context_menu_index).copied().unwrap_or("Open");
        match action {
            "Open" => {
                self.open_selected();
            }
            "Rename" => {
                self.rename_selected("renamed-item");
            }
            "Delete" => {
                self.delete_selected();
            }
            "Copy" => {
                self.copy_selected("copy-of-item");
            }
            "Properties" => {
                let target = if self.entries.is_empty() { "No item selected".to_string() } else { format!("{} [{}]", self.entries[self.selected].name, self.entries[self.selected].kind.indicator()) };
                self.dialog_message = Some(format!("Properties: {target}"));
            }
            _ => {}
        }
        self.close_context_menu();
    }

    fn render(&self, frame: &mut ratatui::Frame<'_>) {
        let _ = &self.settings;
        let area = frame.area();
        let outer = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(3)])
            .split(area);

        let user = std::env::var("USER").unwrap_or_else(|_| String::from("root"));
        let header = Paragraph::new(Line::from(vec![
            Span::styled("TFSV - Terminal File System Visualiser", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::raw(format!("USER: {user}")),
        ]))
        .block(Block::default().borders(Borders::ALL).title("tfsv"));
        frame.render_widget(header, outer[0]);

        let body = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(outer[1]);

        let sidebar = List::new(vec![
            ListItem::new("Home"),
            ListItem::new("Documents"),
            ListItem::new("Downloads"),
            ListItem::new("Projects"),
            ListItem::new("Bookmarks"),
        ])
        .block(Block::default().title("PLACES").borders(Borders::ALL));
        frame.render_widget(sidebar, body[0]);

        let rows = self.entries.iter().enumerate().map(|(index, entry)| {
            let selected = index == self.selected;
            let mut style = Style::default();
            if selected {
                style = style.add_modifier(Modifier::REVERSED);
            }
            let name = if selected { format!("> {}", entry.display_name) } else { entry.display_name.clone() };
            Row::new(vec![
                entry.extension_badge(),
                name,
                if entry.is_dir { "Directory".to_string() } else { "File".to_string() },
                format_size(entry.size),
            ]).style(style)
        }).collect::<Vec<_>>();

        let table = Table::new(
            rows,
            [Constraint::Length(12), Constraint::Min(20), Constraint::Length(12), Constraint::Length(10)],
        )
        .header(Row::new(vec!["TYPE", "NAME", "CLASS", "SIZE"]).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .block(Block::default().title(self.current_dir.to_string_lossy().to_string()).borders(Borders::ALL));
        frame.render_widget(table, body[1]);

        let footer = Paragraph::new("F1 Help  F2 Rename  F5 Download  F6 Upload  F10 Quit  C Context")
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, outer[2]);

        if self.context_menu_open {
            let modal = Rect {
                x: area.x + area.width / 4,
                y: area.y + area.height / 4,
                width: area.width / 2,
                height: 8,
            };
            let menu = Paragraph::new(
                self.context_menu_items
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        let prefix = if idx == self.context_menu_index { ">" } else { " " };
                        format!("{prefix} {item}")
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .block(Block::default().title("Context Menu").borders(Borders::ALL));
            frame.render_widget(menu, modal);
        }

        if let Some(message) = &self.dialog_message {
            let dialog = Rect {
                x: area.x + 2,
                y: area.y + area.height / 2,
                width: area.width.saturating_sub(4),
                height: 3,
            };
            let msg = Paragraph::new(message.as_str())
                .block(Block::default().title("Dialog").borders(Borders::ALL));
            frame.render_widget(msg, dialog);
        }
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit_index = 0;
    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", value, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_navigation_moves_selection() {
        let mut app = App {
            settings: Settings::default(),
            current_dir: std::path::PathBuf::from("/tmp"),
            entries: vec![
                FileEntry::from_path(std::path::PathBuf::from("/tmp/a")),
                FileEntry::from_path(std::path::PathBuf::from("/tmp/b")),
                FileEntry::from_path(std::path::PathBuf::from("/tmp/c")),
                FileEntry::from_path(std::path::PathBuf::from("/tmp/d")),
                FileEntry::from_path(std::path::PathBuf::from("/tmp/e")),
            ],
            selected: 0,
            sort_mode: SortMode::Name,
            status: String::new(),
            context_menu_open: false,
            context_menu_index: 0,
            context_menu_items: vec!["Open", "Rename", "Delete", "Copy", "Properties"],
            dialog_message: None,
        };

        app.page_down(2);
        assert_eq!(app.selected, 2);
        app.page_up(1);
        assert_eq!(app.selected, 1);
        app.move_to_end();
        assert_eq!(app.selected, app.entries.len() - 1);
    }

    #[test]
    fn mouse_selection_clamps_to_valid_index() {
        let mut app = App {
            settings: Settings::default(),
            current_dir: std::path::PathBuf::from("/tmp"),
            entries: vec![
                FileEntry::from_path(std::path::PathBuf::from("/tmp/a")),
                FileEntry::from_path(std::path::PathBuf::from("/tmp/b")),
                FileEntry::from_path(std::path::PathBuf::from("/tmp/c")),
            ],
            selected: 0,
            sort_mode: SortMode::Name,
            status: String::new(),
            context_menu_open: false,
            context_menu_index: 0,
            context_menu_items: vec!["Open", "Rename", "Delete", "Copy", "Properties"],
            dialog_message: None,
        };

        app.select_index(99);
        assert_eq!(app.selected, 2);
    }

    #[test]
    fn context_menu_toggles_and_moves_selection() {
        let mut app = App {
            settings: Settings::default(),
            current_dir: std::path::PathBuf::from("/tmp"),
            entries: vec![
                FileEntry::from_path(std::path::PathBuf::from("/tmp/a")),
                FileEntry::from_path(std::path::PathBuf::from("/tmp/b")),
            ],
            selected: 0,
            sort_mode: SortMode::Name,
            status: String::new(),
            context_menu_open: false,
            context_menu_index: 0,
            context_menu_items: vec!["Open", "Rename", "Delete", "Copy", "Properties"],
            dialog_message: None,
        };

        app.toggle_context_menu();
        assert!(app.context_menu_open);
        app.move_context_menu(2);
        assert_eq!(app.context_menu_index, 2);
        app.close_context_menu();
        assert!(!app.context_menu_open);
    }
}
