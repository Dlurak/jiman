use super::{Coord, Overlay, Size};
use crate::{color::Color, odd::Odd};
use std::num::NonZero;

const TOP_HORIZONTAL: char = '▀';
const BOTTOM_HORIZONTAL: char = '▄';
const VERTICAL_LEFT: char = '▌';
const VERTICAL_RIGHT: char = '▐';
const DIAGONAL_UP: char = '▞';
const DIAGONAL_DOWN: char = '▚';

pub struct Circle {
    center: Coord,
    diameter: Odd<usize>,
    color: Color,
}

impl Circle {
    pub fn new(center: Coord, diameter: Odd<usize>, color: Color) -> Self {
        Self {
            center,
            diameter,
            color,
        }
    }

    fn offset_from_center(&self, (col, row): Coord) -> Option<(isize, isize)> {
        let center = (
            isize::try_from(self.center.0).ok()?,
            isize::try_from(self.center.1).ok()?,
        );

        Some((
            center.0 - isize::try_from(col).ok()?,
            center.1 - isize::try_from(row).ok()?,
        ))
    }
}

impl Overlay for Circle {
    type Foreground = Color;

    fn foreground(&self) -> Self::Foreground {
        self.color
    }

    fn at_pos(&self, col: usize, row: usize, _: Size) -> Option<char> {
        let diameter = self.diameter.value();
        let radius = (diameter - 1) / 2;
        let radius = radius.try_into().ok()?;

        let diagonal_height = (diameter * 2) / 5;
        // Mathemathically proven to not panic
        let vertical_height = Odd::<usize>::new_panics(diameter - 2 * diagonal_height);
        let total_width = diameter + diagonal_height * 2;

        let offset = self.offset_from_center((col, row))?;

        let col_is_vertical = offset.0.abs() <= radius;

        if col_is_vertical && offset.1 == radius {
            return Some(TOP_HORIZONTAL);
        } else if col_is_vertical && offset.1 == -radius {
            return Some(BOTTOM_HORIZONTAL);
        }

        let vertical_radius = isize::try_from((vertical_height - 1) / 2).ok()?;
        let vertical_column = ((total_width - 1) / 2).try_into().ok()?;

        if (offset.0 == vertical_column) && (offset.1.abs() <= vertical_radius) {
            return Some(VERTICAL_LEFT);
        } else if (offset.0 == -vertical_column) && (offset.1.abs() <= vertical_radius) {
            return Some(VERTICAL_RIGHT);
        }

        if offset.0.abs() <= radius || offset.1.abs() <= vertical_radius {
            return None;
        }

        let offset_from_vertical_line = if offset.1 > 0 {
            unsafe { NonZero::new_unchecked(offset.1 - vertical_radius) }
        } else {
            unsafe { NonZero::new_unchecked(-(offset.1.abs() - vertical_radius)) }
        };

        let offset_from_horizontal_line = if offset.0 > 0 {
            unsafe { NonZero::new_unchecked(offset.0 - radius) }
        } else {
            unsafe { NonZero::new_unchecked(-(offset.0.abs() - radius)) }
        };

        let added_offset =
            offset_from_horizontal_line.abs().get() + offset_from_vertical_line.abs().get() - 1;

        (added_offset == diagonal_height.try_into().ok()?).then(|| {
            let is_top_right = offset_from_horizontal_line.is_positive()
                && offset_from_vertical_line.is_positive();
            let is_bottom_left = offset_from_horizontal_line.is_negative()
                && offset_from_vertical_line.is_negative();

            if is_top_right || is_bottom_left {
                DIAGONAL_UP
            } else {
                DIAGONAL_DOWN
            }
        })
    }
}
