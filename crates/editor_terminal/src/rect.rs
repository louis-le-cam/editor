use crate::vector::TermVec;

/// Represent a rectangle with terminal units (`u16`)
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub struct TermRect {
    pub pos: TermVec,
    pub size: TermVec,
}
impl TermRect {
    pub fn new(pos: impl Into<TermVec>, size: impl Into<TermVec>) -> Self {
        Self {
            pos: pos.into(),
            size: size.into(),
        }
    }

    pub fn x(&self) -> u16 {
        self.pos.x
    }

    pub fn y(&self) -> u16 {
        self.pos.y
    }

    pub fn width(&self) -> u16 {
        self.size.x
    }

    pub fn heigth(&self) -> u16 {
        self.size.y
    }
}
