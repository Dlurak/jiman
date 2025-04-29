use super::Overlay;
use crate::color::Color;

pub struct OverlayCharachter {
    char: char,
    color: Color,
    coord: (usize, usize),
}

impl OverlayCharachter {
    pub fn new_centered(char: char, color: Color, size: super::Size) -> Self {
        Self {
            char,
            color,
            coord: (size.width / 2, size.height / 2),
        }
    }
}

impl Overlay for OverlayCharachter {
    type Foreground = Color;

    fn foreground(&self) -> Self::Foreground {
        self.color
    }

    fn at_pos(&self, col: usize, row: usize, _: super::Size) -> Option<char> {
        (self.coord == (col, row)).then_some(self.char)
    }
}
