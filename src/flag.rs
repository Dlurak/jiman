use crate::{
    color::{AnsiCode, AnsiColor, Color},
    overlay::{Overlay, new_overlays},
};

pub struct FallbackedColor {
    true_color: Color,
    ansi_color: AnsiColor,
}

impl AnsiCode for FallbackedColor {
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

impl FallbackedColor {
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
            $key:ident $( | $alt:pat )? => [
                $( ($color:expr, $ansi:expr) ),+ $(,)?
            ]
        ),* $(,)?
    ) => {
        use std::str::FromStr;
        #[derive(Clone, Copy)]
        pub enum Flag {
            $($key,)*
        }

        impl FromStr for Flag {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut c = s.chars();
                let f = c.next().ok_or(())?;
                let serialized = f.to_uppercase().collect::<String>() + c.as_str();
                match serialized.as_str() {
                    $(
                        stringify!($key) $( | $alt )* => Ok(Self::$key),
                    )*
                    _ => Err(())
                }
            }
        }

        impl Flag {
            pub const VALUES: &[(&'static str, &[&'static str])] = &[
                $(
                    (
                        stringify!($key),
                        &[$(stringify!($alt),)*]
                    ),
                )*
            ];

            #[inline]
            pub fn stripes(&self) -> Vec<FallbackedColor> {
                match self {
                    $(
                        Self::$key => vec![
                            $( FallbackedColor::new($color, $ansi) ),*
                        ],
                    )*
                }
            }
        }
    };
}

flags! {
    Lgbtqia | "Lgbtqia+" => [
        (Color::new(228, 3, 3), AnsiColor::Red),
        (Color::new(255, 140, 0), AnsiColor::Red),
        (Color::new(255, 237, 0), AnsiColor::Yellow),
        (Color::new(0, 128, 38), AnsiColor::Green),
        (Color::new(0, 77, 255), AnsiColor::Blue),
        (Color::new(117, 7, 135), AnsiColor::Magenta)
    ],
    Lgbt => [
        (Color::new(228, 3, 3), AnsiColor::Red),
        (Color::new(255, 140, 0), AnsiColor::Red),
        (Color::new(255, 237, 0), AnsiColor::Yellow),
        (Color::new(0, 128, 38), AnsiColor::Green),
        (Color::new(0, 77, 255), AnsiColor::Blue),
        (Color::new(117, 7, 135), AnsiColor::Magenta)
    ],
    Bisexual | "Bi" => [
        (Color::new(214, 2, 122), AnsiColor::Magenta),
        (Color::new(214, 2, 122), AnsiColor::Magenta),
        (Color::new(155, 79, 150), AnsiColor::Magenta),
        (Color::new(0, 56, 168), AnsiColor::Blue),
        (Color::new(0, 56, 168), AnsiColor::Blue),
    ],
    Polysexual | "Poly" => [
        (Color::new(246, 28, 185), AnsiColor::Magenta),
        (Color::new(7, 218, 105), AnsiColor::Green),
        (Color::new(28, 146, 246), AnsiColor::Cyan),
    ],
    Pansexual | "Pan" => [
        (Color::new(255, 33, 140), AnsiColor::Magenta),
        (Color::new(255, 216, 0), AnsiColor::Yellow),
        (Color::new(33, 177, 255), AnsiColor::Cyan),
    ],
    Asexual | "Ace" => [
        (Color::BLACK, AnsiColor::Black),
        (Color::gray(163), AnsiColor::Black),
        (Color::WHITE, AnsiColor::White),
        (Color::new(128, 0, 128), AnsiColor::Magenta),
    ],
    Aromantic | "Aro" => [
        (Color::new(62, 167,68), AnsiColor::Green),
        (Color::new(169, 212, 120), AnsiColor::Green),
        (Color::WHITE, AnsiColor::White),
        (Color::gray(170), AnsiColor::Black),
        (Color::BLACK, AnsiColor::Black),
    ],
    Lesbian => [
        (Color::new(214, 44,0), AnsiColor::Red),
        (Color::new(255, 153, 86), AnsiColor::Red),
        (Color::WHITE, AnsiColor::White),
        (Color::new(211, 98,164), AnsiColor::Magenta),
        (Color::new(164, 1, 98), AnsiColor::Magenta),
    ],
    Gay => [
        (Color::new(7, 141, 112), AnsiColor::Green),
        (Color::new(152, 232, 193), AnsiColor::Green),
        (Color::WHITE, AnsiColor::White),
        (Color::new(123, 173, 226), AnsiColor::Cyan),
        (Color::new(61, 26, 120), AnsiColor::Blue),
    ],
    Demisexual | "Demi" => [
        (Color::WHITE, AnsiColor::White),
        (Color::WHITE, AnsiColor::White),
        (Color::new(128, 0, 128), AnsiColor::Magenta),
        (Color::gray(210), AnsiColor::Black),
        (Color::gray(210), AnsiColor::Black),
    ],
    Trans => [
        (Color::new(115, 207, 244), AnsiColor::Cyan),
        (Color::new(238, 175, 192), AnsiColor::Magenta),
        (Color::WHITE, AnsiColor::White),
        (Color::new(238, 175, 192), AnsiColor::Magenta),
        (Color::new(115, 207, 244), AnsiColor::Cyan),
    ],
    Nonbinary => [
        (Color::new(252, 244, 52), AnsiColor::Yellow),
        (Color::WHITE, AnsiColor::White),
        (Color::new( 156, 89, 209), AnsiColor::Magenta),
        (Color::gray(44), AnsiColor::Black),
    ],
}

impl Flag {
    pub fn overlays(&self) -> Vec<Overlay<Color, Color>> {
        match self {
            Self::Lgbtqia => new_overlays(&[
                (Color::new(253, 216, 23), 2, 0),
                (Color::WHITE, 0, 1),
                (Color::new(244, 174, 200), 0, 5),
                (Color::new(123, 204, 229), 0, 9),
                (Color::new(148, 85, 22), 0, 13),
                (Color::BLACK, 0, 17),
            ]),
            Self::Demisexual => vec![Overlay::new(None, 0, 0, Color::BLACK)],
            _ => Vec::new(),
        }
    }
}
