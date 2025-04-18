use crate::color::{AnsiCode, AnsiColor, Color};

pub struct Stripe {
    true_color: Color,
    ansi_color: AnsiColor,
}

impl AnsiCode for Stripe {
    fn fg(&self) -> String {
        if crate::term::true_color() {
            self.true_color.fg()
        } else {
            self.ansi_color.fg()
        }
    }

    fn bg(&self) -> String {
        if crate::term::true_color() {
            self.true_color.bg()
        } else {
            self.ansi_color.bg()
        }
    }
}

impl Stripe {
    fn new(ideal: Color, fallback: AnsiColor) -> Self {
        Self {
            true_color: ideal,
            ansi_color: fallback,
        }
    }
}

macro_rules! flags {
    (
        $(
            $key:ident => [
                $( ($color:expr, $ansi:expr) ),* $(,)?
            ]
        ),* $(,)?
    ) => {
        impl Flag {
            $(
                #[inline]
                pub fn $key() -> Self {
                    Self {
                        stripes: vec![
                            $( Stripe::new($color, $ansi) ),*
                        ]
                    }
                }
            )*
            pub fn by_name(name: &str) -> Option<Self> {
                match name {
                    $(
                        stringify!($key) => Some(Self::$key()),
                    )*
                    _ => None
                }
            }
        }
    };
}

pub struct Flag {
    pub stripes: Vec<Stripe>,
}

impl Flag {
    pub fn new(stripes: Vec<Stripe>) -> Self {
        Self { stripes }
    }
}

flags! {
    lgbt => [
        (Color::new(228, 3, 3), AnsiColor::Red),
        (Color::new(255, 140, 0), AnsiColor::Red),
        (Color::new(255, 237, 0), AnsiColor::Yellow),
        (Color::new(0, 128, 38), AnsiColor::Green),
        (Color::new(0, 77, 255), AnsiColor::Blue),
        (Color::new(117, 7, 135), AnsiColor::Magenta)
    ],
    bisexual => [
        (Color::new(214, 2, 122), AnsiColor::Magenta),
        (Color::new(214, 2, 122), AnsiColor::Magenta),
        (Color::new(155, 79, 150), AnsiColor::Magenta),
        (Color::new(0, 56, 168), AnsiColor::Blue),
        (Color::new(0, 56, 168), AnsiColor::Blue),
    ],
    polysexual => [
        (Color::new(246, 28, 185), AnsiColor::Magenta),
        (Color::new(7, 218, 105), AnsiColor::Green),
        (Color::new(28, 146, 246), AnsiColor::Cyan),
    ],
    pansexual => [
        (Color::new(255, 33, 140), AnsiColor::Magenta),
        (Color::new(255, 216, 0), AnsiColor::Yellow),
        (Color::new(33, 177, 255), AnsiColor::Cyan),
    ],
    asexual => [
        (Color::BLACK, AnsiColor::Black),
        (Color::new(163, 163, 163), AnsiColor::Black),
        (Color::WHITE, AnsiColor::White),
        (Color::new(128, 0, 128), AnsiColor::Magenta),
    ],
    aromantic => [
        (Color::new(62, 167,68), AnsiColor::Green),
        (Color::new(169, 212, 120), AnsiColor::Green),
        (Color::WHITE, AnsiColor::White),
        (Color::gray(170), AnsiColor::Black),
        (Color::BLACK, AnsiColor::Black),
    ],
    lesbian => [
        (Color::new(214, 44,0), AnsiColor::Red),
        (Color::new(255, 153, 86), AnsiColor::Red),
        (Color::WHITE, AnsiColor::White),
        (Color::new(211, 98,164), AnsiColor::Magenta),
        (Color::new(164, 1, 98), AnsiColor::Magenta),
    ],
}
