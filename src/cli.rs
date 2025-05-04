use crate::flag::Flag;
use clap::{Parser, Subcommand};
use std::num::{IntErrorKind, NonZero};

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Print a pride flag
    Print(PrintCli),
    #[command(alias = "ls", alias = "l")]
    /// List all available flags
    List {
        /// Show aliases
        #[arg(long, default_value_t = false)]
        aliases: bool,
    },
    /// Output shell completion scripts to stdout, usefull for package maintainers!
    #[cfg(feature = "complete")]
    #[command(hide = true)]
    Complete {
        /// Name of the shell
        shell: clap_complete::Shell,
    },
}

#[derive(Parser, Clone)]
pub struct PrintCli {
    /// The name (or alias) of the flag to output
    #[arg(value_enum)]
    pub flag: Flag,
    #[arg(
        short,
		long,
		value_parser = parse_width,
		help = "The width of the flag",
		long_help = "The width of the flag, either an absolute length (charachters) or percentages of the terminal width"
    )]
    pub width: Option<Size>,
    #[arg(
        long,
        value_parser = parse_width,
		help = "The height of the flag",
		long_help = "The height of the flag, either an absolute length (charachters) or percentages of the terminal height"
    )]
    pub height: Option<Size>,
    /// The angle of the triangle on the side present on some flags
    #[arg(long, alias = "angle", default_value_t = unsafe{ NonZero::new_unchecked(1) })]
    pub triangle_angle: NonZero<usize>,
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
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
