use std::path::PathBuf;

/// The specific operation being performed by the input dialog modal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputMode {
    Rename { target: PathBuf },
    NewFolder,
    ConfirmDelete { target: PathBuf },
}

/// Represents an interactive modal prompt for user text input (such as renaming or creating folders).
#[derive(Clone, Debug)]
pub struct InputDialog {
    pub title: String,
    pub prompt: String,
    pub value: String,
    pub cursor: usize,
    pub mode: InputMode,
}

impl InputDialog {
    /// Creates a new input dialog modal with an initial text value and cursor at the end.
    pub fn new(
        title: impl Into<String>,
        prompt: impl Into<String>,
        initial_value: impl Into<String>,
        mode: InputMode,
    ) -> Self {
        let value = initial_value.into();
        let cursor = value.chars().count();
        Self {
            title: title.into(),
            prompt: prompt.into(),
            value,
            cursor,
            mode,
        }
    }

    /// Inserts a character at the current text cursor position.
    pub fn insert_char(&mut self, ch: char) {
        let char_count = self.value.chars().count();
        if self.cursor >= char_count {
            self.value.push(ch);
            self.cursor = self.value.chars().count();
        } else {
            let byte_idx = self
                .value
                .char_indices()
                .nth(self.cursor)
                .map(|(i, _)| i)
                .unwrap_or(self.value.len());
            self.value.insert(byte_idx, ch);
            self.cursor += 1;
        }
    }

    /// Deletes the character immediately to the left of the cursor (Backspace key).
    pub fn backspace(&mut self) {
        if self.cursor > 0 && !self.value.is_empty() {
            let (target_byte, _) = self
                .value
                .char_indices()
                .nth(self.cursor - 1)
                .expect("valid character index");
            self.value.remove(target_byte);
            self.cursor -= 1;
        }
    }

    /// Deletes the character directly under the cursor (Delete key).
    pub fn delete(&mut self) {
        if self.cursor < self.value.chars().count() {
            let (target_byte, _) = self
                .value
                .char_indices()
                .nth(self.cursor)
                .expect("valid character index");
            self.value.remove(target_byte);
        }
    }

    /// Moves the cursor one position to the left.
    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Moves the cursor one position to the right.
    pub fn cursor_right(&mut self) {
        if self.cursor < self.value.chars().count() {
            self.cursor += 1;
        }
    }

    /// Jumps the cursor to the start of the input.
    pub fn cursor_home(&mut self) {
        self.cursor = 0;
    }

    /// Jumps the cursor to the end of the input.
    pub fn cursor_end(&mut self) {
        self.cursor = self.value.chars().count();
    }
}
