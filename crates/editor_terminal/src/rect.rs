use glam::U16Vec2;

/// Represent a rectangle with terminal units (`u16`)
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash)]
pub struct TermRect {
    pub pos: U16Vec2,
    pub size: U16Vec2,
}
impl TermRect {
    pub fn new(pos: impl Into<U16Vec2>, size: impl Into<U16Vec2>) -> Self {
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
