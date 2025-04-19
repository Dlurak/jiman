use crate::flag::Flag;
use clap::{Parser, Subcommand};
use std::{
    num::{IntErrorKind, NonZero},
    str::FromStr,
};

#[derive(Subcommand, Clone)]
pub enum Command {
    Print(PrintCli),
    List {
        #[arg(long, default_value_t = false)]
        aliases: bool,
    },
}

#[derive(Parser, Clone)]
pub struct PrintCli {
    #[arg(value_parser = parse_flag)]
    pub flag: Flag,
    #[arg(short, long, value_parser = parse_width)]
    pub width: Option<Size>,
    #[arg(long, value_parser = parse_width)]
    pub height: Option<Size>,
    #[arg(long, alias = "angle", default_value_t = unsafe{ NonZero::new_unchecked(1) })]
    pub triangle_angle: NonZero<usize>,
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

fn parse_flag(s: &str) -> Result<Flag, String> {
    Flag::from_str(s).map_err(|_| format!("\"{s}\" isn't a recognized flag, if it should be available please consider opening a pr :)"))
}

#[derive(Clone)]
pub enum Size {
    AbsoluteChars(NonZero<usize>),
    Percentage(NonZero<u8>),
}

impl Size {
    pub fn absolute_width(&self) -> Option<NonZero<usize>> {
        match self {
            Self::AbsoluteChars(c) => Some(*c),
            Self::Percentage(perc) => {
                let (w, _) = term_size::dimensions_stdout()?;
                let w = w as f32;
                let perc = perc.get() as f32;
                NonZero::new((w / 100.0 * perc) as usize)
            }
        }
    }
    pub fn absolute_height(&self) -> Option<NonZero<usize>> {
        match self {
            Self::AbsoluteChars(c) => Some(*c),
            Self::Percentage(perc) => {
                let (_, h) = term_size::dimensions_stdout()?;
                let h = h as f32;
                let perc = perc.get() as f32;
                NonZero::new((h / 100.0 * perc) as usize)
            }
        }
    }
}

fn parse_width(s: &str) -> Result<Size, String> {
    match s.parse() {
        Ok(num) => return Ok(Size::AbsoluteChars(num)),
        Err(err) => match err.kind() {
            IntErrorKind::Zero => return Err(String::from("The width must be greater than zero")),
            IntErrorKind::Empty => return Err(String::from("The width must not be empty")),
            IntErrorKind::NegOverflow | IntErrorKind::PosOverflow => {
                return Err(format!(
                    "Your provided width doesn't fit into the integer (1-{})",
                    usize::MAX
                ));
            }
            _ => {}
        },
    }

    let mut s = String::from(s);
    match s.pop() {
        Some('%') => {}
        Some(_) => return Err(String::from("Relative units must end with a '%'")),
        None => unreachable!("We already remove empty ones above"),
    }

    match s.parse::<NonZero<u8>>() {
        Ok(width) => {
            if width.get() > 100 {
                Err(String::from("The width must not exceed 100%"))
            } else {
                Ok(Size::Percentage(width))
            }
        }
        Err(err) => {
            let msg = match err.kind() {
                IntErrorKind::Zero => "The width must be at least 1%",
                IntErrorKind::Empty | IntErrorKind::InvalidDigit => "Please provide a number",
                IntErrorKind::NegOverflow => "The width must be at least 1%",
                IntErrorKind::PosOverflow => "The width must not exceed 100%",
                _ => todo!("This is a bug, please create a GitHub issue to report it!"),
            };
            Err(msg.into())
        }
    }
}
