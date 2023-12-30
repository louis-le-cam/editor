use editor_action::SingleLineDocumentAction;

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

    pub fn handle_action(&mut self, action: SingleLineDocumentAction) {
        use editor_action::SingleLineDocumentAction::*;

        match action {
            MoveLeft => self.cursor = self.cursor.saturating_sub(1),
            MoveRight => {
                if self.cursor < self.line.len() {
                    self.cursor += 1;
                }
            }
            Insert { char } => {
                let i = self
                    .line
                    .char_indices()
                    .nth(self.cursor)
                    .map(|(i, _)| i)
                    .unwrap_or(self.line.len());
                self.cursor += 1;
                self.line.insert(i, char);
            }
            DeleteBefore => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    if let Some((i, _)) = self.line.char_indices().nth(self.cursor) {
                        self.line.remove(i);
                    }
                }
            }
        }
    }
}
