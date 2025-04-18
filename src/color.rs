pub const RESET: &str = "\x1b[0m";

pub struct Color(u8, u8, u8);

pub trait AnsiCode {
    fn fg(&self) -> String;
    fn bg(&self) -> String;
}

impl Color {
    pub const BLACK: Self = Self(0, 0, 0);
    pub const WHITE: Self = Self(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub const fn gray(lightness: u8) -> Self {
        Self(lightness, lightness, lightness)
    }
}

impl AnsiCode for Color {
    fn fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.0, self.1, self.2)
    }

    fn bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.0, self.1, self.2)
    }
}

#[derive(Clone, Copy)]
pub enum AnsiColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
}

impl AnsiCode for AnsiColor {
    fn fg(&self) -> String {
        format!("\x1b[{}m", *self as u8)
    }

    fn bg(&self) -> String {
        let num = *self as u8;
        format!("\x1b[{}m", num + 10)
    }
}
