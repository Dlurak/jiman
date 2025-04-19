use std::num::NonZero;

use crate::color::Color;

use super::{Overlay, Size};

pub struct Triangle {
    padding: usize,
    insert: usize,
    slope: NonZero<usize>,
    color: Color,
}

impl Triangle {
    pub fn new(padding: usize, insert: usize, slope: NonZero<usize>, color: Color) -> Self {
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

enum TriangleChar {
    Fill = '█' as isize,
    UpperDiagonal = '▙' as isize,
    // UpperDiagonal = '🭀' as isize,
    LowerDiagonal = '▛' as isize,
    // LowerDiagonal = '🭛' as isize,
    CenterHalf = '▌' as isize,
}

enum Region {
    Top,
    Center,
    Bottom,
}

impl Region {
    fn at_pos(height: usize, row: usize) -> Self {
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
            Region::Top => TriangleChar::UpperDiagonal,
            Region::Center => TriangleChar::CenterHalf,
            Region::Bottom => TriangleChar::LowerDiagonal,
        }
    }
}

type Coord = (usize, usize);

impl TriangleChar {
    pub fn at_pos(
        height: usize,
        slope: NonZero<usize>,
        (col, row): Coord,
        insert: usize,
    ) -> Option<Self> {
        let region = Region::at_pos(height, row);
        let actual_row = if let Region::Bottom = region {
            height - row - 1
        } else {
            row
        };

        let filler_width = actual_row * slope.get() + insert;
        if (filler_width + 1) == col + 1 {
            Some(region.into())
        } else if col < filler_width {
            Some(TriangleChar::Fill)
        } else {
            None
        }
    }
}

impl From<TriangleChar> for char {
    fn from(value: TriangleChar) -> Self {
        unsafe { char::from_u32_unchecked(value as u32) }
    }
}
