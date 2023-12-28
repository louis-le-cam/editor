use editor_action::DocumentActionHandler;
use log::warn;

pub struct SingleLineDocument {
    line: String,
    cursor: usize,
}

impl SingleLineDocument {
    pub fn new() -> Self {
        Self {
            line: String::new(),
            cursor: 0,
        }
    }

    pub fn line(&self) -> &str {
        &self.line
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn clear(&mut self) {
        self.line = String::new();
        self.cursor = 0;
    }
}

impl DocumentActionHandler for SingleLineDocument {
    fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1)
    }

    fn move_right(&mut self) {
        if self.cursor < self.line.len() {
            self.cursor += 1;
        }
    }

    fn move_up(&mut self) {
        warn!("Can't move up on single line document");
    }

    fn move_down(&mut self) {
        warn!("Can't move down on single line document");
    }

    fn insert(&mut self, ch: char) {
        let i = self
            .line
            .char_indices()
            .nth(self.cursor)
            .map(|(i, _)| i)
            .unwrap_or(self.line.len());
        self.cursor += 1;
        self.line.insert(i, ch);
    }

    fn delete_before(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            if let Some((i, _)) = self.line.char_indices().nth(self.cursor) {
                self.line.remove(i);
            }
        }
    }

    fn insert_line_before_cursor(&mut self) {
        warn!("Can't insert line before cursor on single line document");
    }

    fn write(&mut self) {
        warn!("Can't write single line document");
    }
}
