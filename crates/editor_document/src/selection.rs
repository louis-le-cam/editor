use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct Selection {
    true_start: (usize, usize),
    true_end: (usize, usize),
}

impl Selection {
    pub fn start(&self) -> (usize, usize) {
        self.true_start
    }

    pub fn end(&self) -> (usize, usize) {
        self.true_end
    }

    pub fn min(&self) -> (usize, usize) {
        match self.true_start.1.cmp(&self.true_end.1) {
            Ordering::Less => self.true_start,
            Ordering::Equal => match self.true_start.0.cmp(&self.true_end.0) {
                Ordering::Less => self.true_start,
                Ordering::Equal => self.true_start,
                Ordering::Greater => self.true_end,
            },
            Ordering::Greater => self.true_end,
        }
    }

    pub fn max(&self) -> (usize, usize) {
        match self.true_start.1.cmp(&self.true_end.1) {
            Ordering::Less => self.true_end,
            Ordering::Equal => match self.true_start.0.cmp(&self.true_end.0) {
                Ordering::Less => self.true_end,
                Ordering::Equal => self.true_end,
                Ordering::Greater => self.true_start,
            },
            Ordering::Greater => self.true_start,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InternalSelection {
    start: (usize, usize),
    end: (usize, usize),
}

impl InternalSelection {
    pub fn new() -> Self {
        Self {
            start: (0, 0),
            end: (0, 0),
        }
    }

    pub fn len(&self, lines: &[String]) -> usize {
        let (min, max) = self.true_min_max(lines);

        (min.1..max.1 + 1).fold(0, |len, y| {
            let Some(line) = lines.get(y) else {
                return len;
            };

            if y == min.1 && y == max.1 {
                len + line
                    .chars()
                    .skip(min.0)
                    .take(max.0.saturating_sub(min.0))
                    .count()
            } else if y == min.1 {
                len + line.chars().skip(min.0).count()
            } else if y == max.1 {
                len + line.chars().take(max.0).count()
            } else {
                len + line.chars().count()
            }
        })
    }

    pub fn to_selection(&self, lines: &[String]) -> Selection {
        Selection {
            true_start: self.true_start(lines),
            true_end: self.true_end(lines),
        }
    }

    fn true_min_max(&self, lines: &[String]) -> ((usize, usize), (usize, usize)) {
        let true_start = self.true_start(lines);
        let true_end = self.true_end(lines);

        match true_start.1.cmp(&true_end.1) {
            Ordering::Less => (true_start, true_end),
            Ordering::Equal => match true_start.0.cmp(&true_end.0) {
                Ordering::Less => (true_start, true_end),
                Ordering::Equal => (true_start, true_end),
                Ordering::Greater => (true_end, true_start),
            },
            Ordering::Greater => (true_end, true_start),
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
        self.extend_end_left(lines);
        self.collapse_to_end();
    }
    pub fn move_right(&mut self, lines: &[String]) {
        self.extend_end_right(lines);
        self.collapse_to_end();
    }
    pub fn move_down(&mut self, lines: &[String]) {
        self.extend_end_down(lines);
        self.collapse_to_end();
    }
    pub fn move_up(&mut self) {
        self.extend_end_up();
        self.collapse_to_end();
    }

    pub fn extend_end_left(&mut self, lines: &[String]) {
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
    pub fn extend_end_right(&mut self, lines: &[String]) {
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
    pub fn extend_end_down(&mut self, lines: &[String]) {
        if self.end.1 < lines.len() {
            self.end.1 = self.end.1.saturating_add(1);
        }
    }
    pub fn extend_end_up(&mut self) {
        self.end.1 = self.end.1.saturating_sub(1);
    }

    pub fn extend_start_left(&mut self, lines: &[String]) {
        self.start = self.true_start(lines);

        if self.start.0 == 0 {
            if self.start.1 == 0 {
                self.start = (0, 0);
            } else {
                self.start.1 = self.start.1 - 1;
                self.start.0 = lines.get(self.start.1).map(|line| line.len()).unwrap_or(0);
            }
        } else {
            self.start.0 = self.start.0 - 1;
        }
    }
    pub fn extend_start_right(&mut self, lines: &[String]) {
        self.start = self.true_start(lines);

        if self.start.0
            >= lines
                .get(self.start.1)
                .map(|line| line.chars().count())
                .unwrap_or(0)
        {
            self.start.0 = 0;
            if self.start.1 < lines.len() {
                self.start.1 = self.start.1.saturating_add(1);
            }
        } else {
            self.start.0 = self.start.0.saturating_add(1);
        }
    }
    pub fn extend_start_down(&mut self, lines: &[String]) {
        if self.start.1 < lines.len() {
            self.start.1 = self.start.1.saturating_add(1);
        }
    }
    pub fn extend_start_up(&mut self) {
        self.start.1 = self.start.1.saturating_sub(1);
    }

    pub fn move_selection_left(&mut self, lines: &[String]) {
        self.extend_end_left(lines);
        self.extend_start_left(lines);
    }
    pub fn move_selection_right(&mut self, lines: &[String]) {
        self.extend_end_right(lines);
        self.extend_start_right(lines);
    }
    pub fn move_selection_down(&mut self, lines: &[String]) {
        self.extend_end_down(lines);
        self.extend_start_down(lines);
    }
    pub fn move_selection_up(&mut self) {
        self.extend_end_up();
        self.extend_start_up();
    }
}
