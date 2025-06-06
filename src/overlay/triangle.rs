use super::{Coord, Overlay, Size};
use crate::color::Color;
use std::num::NonZero;

pub struct Triangle {
    padding: usize,
    insert: usize,
    slope: NonZero<usize>,
    color: Color,
}

impl Triangle {
    pub const fn new(padding: usize, insert: usize, slope: NonZero<usize>, color: Color) -> Self {
        Self {
            padding,
            insert,
            slope,
            color,
        }
    }
}

impl Overlay for Triangle {
    type Foreground = Color;

    fn foreground(&self) -> Self::Foreground {
        self.color
    }

    fn at_pos(&self, col: usize, row: usize, size: Size) -> Option<char> {
        let padding = self.padding;

        if !(padding..size.height.saturating_sub(padding)).contains(&row) {
            return None;
        }

        TriangleChar::at_pos(
            size.height - padding * 2,
            self.slope,
            (col, row - padding),
            self.insert,
        )
        .map(char::from)
    }
}

#[repr(u16)]
enum TriangleChar {
    Fill = '█' as u16,
    UpperDiagonal = '▙' as u16,
    // UpperDiagonal = '🭀' as u16,
    LowerDiagonal = '▛' as u16,
    // LowerDiagonal = '🭛' as u16,
    CenterHalf = '▌' as u16,
}

enum Region {
    Top,
    Center,
    Bottom,
}

impl Region {
    const fn at_pos(height: usize, row: usize) -> Self {
        if height % 2 == 1 && height / 2 == row {
            Self::Center
        } else if height / 2 <= row {
            Self::Bottom
        } else {
            Self::Top
        }
    }
}

impl From<Region> for TriangleChar {
    fn from(value: Region) -> Self {
        match value {
            Region::Top => Self::UpperDiagonal,
            Region::Center => Self::CenterHalf,
            Region::Bottom => Self::LowerDiagonal,
        }
    }
}

impl TriangleChar {
    pub fn at_pos(
        height: usize,
        slope: NonZero<usize>,
        (col, row): Coord,
        insert: usize,
    ) -> Option<Self> {
        let region = Region::at_pos(height, row);
        let actual_row = if matches!(region, Region::Bottom) {
            height - row - 1
        } else {
            row
        };

        let filler_width = actual_row * slope.get() + insert;
        if (filler_width + 1) == col + 1 {
            Some(region.into())
        } else if col < filler_width {
            Some(Self::Fill)
        } else {
            None
        }
    }
}

impl From<TriangleChar> for char {
    fn from(value: TriangleChar) -> Self {
        unsafe { Self::from_u32_unchecked(value as u32) }
    }
}
