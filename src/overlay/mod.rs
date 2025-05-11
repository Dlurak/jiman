pub mod charachter;
pub mod circle;
pub mod triangle;

use crate::color::AnsiCode;

pub trait Overlay {
    type Foreground: AnsiCode;

    fn foreground(&self) -> Self::Foreground;

    fn at_pos(&self, col: usize, row: usize, size: Size) -> Option<char>;
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Size {
    pub const fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }

    pub const fn center(&self) -> (usize, usize) {
        (self.width / 2, self.height / 2)
    }
}

pub type Coord = (usize, usize);
