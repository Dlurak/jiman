use std::num::NonZero;

use crate::{
    color::{AnsiCode, AnsiColor, Color},
    overlay::{Overlay, Size, charachter::OverlayCharachter, triangle::Triangle},
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
    const fn new(ideal: Color, fallback: AnsiColor) -> Self {
        Self {
            true_color: ideal,
            ansi_color: fallback,
        }
    }
}

macro_rules! flags {
    (
        $(
            $key:ident $(| $alt:expr)* => [
                $( ($color:expr, $ansi:expr) ),+ $(,)?
            ]
        ),* $(,)?
    ) => {
        use clap::{ValueEnum, builder::PossibleValue};

        #[derive(Clone, Copy)]
        pub enum Flag {
            $($key,)*
        }

        impl ValueEnum for Flag {
            fn value_variants<'a>() -> &'a [Self] {
                &[$(Self::$key,)*]
            }

            fn to_possible_value(&self) -> Option<PossibleValue> {
                Some(match self {
                    $(
                        Self::$key => {
                            PossibleValue::new(stringify!($key).to_lowercase())
                                .aliases([
                                    String::from(stringify!($key)),
                                    $(
                                        $alt.into(),
                                        $alt.to_lowercase(),
                                    )*
                                ])
                        },
                    )*
                })
            }
        }

        impl Flag {
            pub const VALUES: &[(&'static str, &[&'static str])] = &[
                $(
                    (
                        stringify!($key),
                        &[$($alt,)*]
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
    Aroace => [
        (Color::new(227, 140, 1), AnsiColor::Red),
        (Color::new(236, 205, 0), AnsiColor::Yellow),
        (Color::WHITE, AnsiColor::White),
        (Color::new(98, 175, 222), AnsiColor::Cyan),
        (Color::new(32, 56, 87), AnsiColor::Blue),
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
    Demisexual => [
        (Color::WHITE, AnsiColor::White),
        (Color::WHITE, AnsiColor::White),
        (Color::new(128, 0, 128), AnsiColor::Magenta),
        (Color::gray(210), AnsiColor::Black),
        (Color::gray(210), AnsiColor::Black),
    ],
    Demiromantic => [
        (Color::WHITE, AnsiColor::White),
        (Color::WHITE, AnsiColor::White),
        (Color::new(51,  138, 55), AnsiColor::Green),
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
    Polyamory => [
        (Color::new(0, 0, 255), AnsiColor::Blue),
        (Color::new(255, 0, 0), AnsiColor::Red),
        (Color::BLACK, AnsiColor::Black),
    ],
    Demigirl => [
        (Color::gray(127), AnsiColor::Black),
        (Color::gray(196), AnsiColor::White),
        (Color::new(255,174,201), AnsiColor::Magenta),
        (Color::WHITE, AnsiColor::White),
        (Color::new(255,174,201), AnsiColor::Magenta),
        (Color::gray(196), AnsiColor::White),
        (Color::gray(127), AnsiColor::Black),
    ],
    Demiboy => [
        (Color::gray(127), AnsiColor::Black),
        (Color::gray(196), AnsiColor::White),
        (Color::new(193,217,235), AnsiColor::Cyan),
        (Color::WHITE, AnsiColor::White),
        (Color::new(193,217,235), AnsiColor::Cyan),
        (Color::gray(196), AnsiColor::White),
        (Color::gray(127), AnsiColor::Black),
    ],
}

impl Flag {
    pub fn overlays(
        &self,
        slope: NonZero<usize>,
        size: Size,
    ) -> Vec<Box<dyn Overlay<Foreground = Color>>> {
        match self {
            Self::Lgbtqia => {
                // How far the triangles should be inserted
                // Usefull for making space for the intersex circle
                let insert = 0;

                let colors = [
                    Color::WHITE,
                    Color::new(244, 174, 200),
                    Color::new(123, 204, 229),
                    Color::new(148, 85, 22),
                    Color::BLACK,
                ];

                let mut res: Vec<Box<dyn Overlay<Foreground = Color>>> =
                    Vec::with_capacity(colors.len() + 1);

                res.push(Box::new(Triangle::new(
                    2,
                    insert,
                    slope,
                    Color::new(253, 216, 23),
                )));

                for (i, &color) in colors.iter().enumerate() {
                    res.push(Box::new(Triangle::new(0, insert + 1 + 4 * i, slope, color)));
                }

                res
            }
            Self::Demisexual | Self::Demiromantic => {
                vec![Box::new(Triangle::new(0, 0, slope, Color::BLACK))]
            }
            Self::Polyamory => vec![Box::new(OverlayCharachter::new_centered(
                'π',
                Color::new(255, 255, 0),
                size,
            ))],
            _ => Vec::new(),
        }
    }
}
