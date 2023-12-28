#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Selection,
}
impl Mode {
    pub fn abreviation(&self) -> &'static str {
        match self {
            Mode::Normal => "NOR",
            Mode::Insert => "INS",
            Mode::Selection => "SEL",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focused {
    Editor,
    CommandBar,
}
