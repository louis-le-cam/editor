#[derive(Clone, Copy, Debug)]
pub struct Selection {
    start: (usize, usize),
    end: (usize, usize),
}

impl Selection {
    pub fn new() -> Self {
        Self {
            start: (0, 0),
            end: (0, 0),
        }
    }

    pub fn true_start(&self, lines: &[String]) -> (usize, usize) {
        let y = self.start.1.min(lines.len());
        let x = self
            .start
            .0
            .min(lines.get(y).map(|line| line.chars().count()).unwrap_or(0));
        (x, y)
    }

    pub fn true_end(&self, lines: &[String]) -> (usize, usize) {
        let y = self.end.1.min(lines.len());
        let x = self
            .end
            .0
            .min(lines.get(y).map(|line| line.chars().count()).unwrap_or(0));
        (x, y)
    }

    pub fn collapse_to_end(&mut self) {
        self.start = self.end;
    }

    pub fn collapse_to_start(&mut self) {
        self.end = self.start
    }

    pub fn collapse_to_true_end(&mut self, lines: &[String]) {
        let true_end = self.true_end(lines);
        self.start = true_end;
        self.end = true_end;
    }

    pub fn collapse_to_true_start(&mut self, lines: &[String]) {
        let true_start = self.true_start(lines);
        self.start = true_start;
        self.end = true_start;
    }

    pub fn move_left(&mut self, lines: &[String]) {
        self.extend_left(lines);
        self.collapse_to_end();
    }

    pub fn move_right(&mut self, lines: &[String]) {
        self.extend_right(lines);
        self.collapse_to_end();
    }

    pub fn move_down(&mut self, lines: &[String]) {
        self.extend_down(lines);
        self.collapse_to_end();
    }

    pub fn move_up(&mut self) {
        self.extend_up();
        self.collapse_to_end();
    }

    pub fn extend_left(&mut self, lines: &[String]) {
        self.end = self.true_end(lines);

        if self.end.0 == 0 {
            if self.end.1 == 0 {
                self.end = (0, 0);
            } else {
                self.end.1 = self.end.1 - 1;
                self.end.0 = lines.get(self.end.1).map(|line| line.len()).unwrap_or(0);
            }
        } else {
            self.end.0 = self.end.0 - 1;
        }
    }

    pub fn extend_right(&mut self, lines: &[String]) {
        self.end = self.true_end(lines);

        if self.end.0
            >= lines
                .get(self.end.1)
                .map(|line| line.chars().count())
                .unwrap_or(0)
        {
            self.end.0 = 0;
            if self.end.1 < lines.len() {
                self.end.1 = self.end.1.saturating_add(1);
            }
        } else {
            self.end.0 = self.end.0.saturating_add(1);
        }
    }

    pub fn extend_down(&mut self, lines: &[String]) {
        if self.end.1 < lines.len() {
            self.end.1 = self.end.1.saturating_add(1);
        }
    }

    pub fn extend_up(&mut self) {
        self.end.1 = self.end.1.saturating_sub(1);
    }
}
