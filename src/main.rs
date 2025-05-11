use clap::Parser;
use jiman::{
    cli::{Cli, Command, PrintCli},
    color::{AnsiCode, RESET},
    flag::Flag,
    overlay::Size,
};
use std::fmt::Write;

fn main() {
    let Cli { command } = Cli::parse();

    match command {
        Command::List { aliases } => {
            for (variant, alts) in Flag::VALUES {
                let alts = (aliases && !alts.is_empty()).then(|| alts.join(", "));
                match alts {
                    Some(alts) => {
                        println!("{variant} ({alts})");
                    }
                    None => {
                        println!("{variant}");
                    }
                }
            }
        }
        Command::Print(cli) => print_handler(cli),
        #[cfg(feature = "complete")]
        Command::Complete { shell } => {
            use clap::CommandFactory;

            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "jiman", &mut std::io::stdout());
        }
    }
}

fn print_handler(cli: PrintCli) {
    let width = cli
        .width
        .clone()
        .and_then(|w| w.absolute_width())
        .map(|w| w.get())
        .or_else(|| {
            let (wid, _) = term_size::dimensions_stdout()?;
            Some(wid.min(71))
        })
        .unwrap_or(71);

    let height = cli
        .height
        .and_then(|h| h.absolute_height())
        .map_or(15, |v| v.get());

    let stripes = cli.flag.stripes();
    let stripe_height = height / stripes.len();
    let height = stripe_height * stripes.len();
    let size = Size::new(height, width);

    let overlays = cli.flag.overlays(cli.slope, size);

    for y in 0..height {
        let stripe = &stripes[y / stripe_height];
        let line = (0..width).fold(String::new(), |mut line, x| {
            let Some((index, overlay, ch)) = overlays.iter().enumerate().find_map(|(i, ov)| {
                let char = ov.at_pos(x, y, size)?;
                Some((i, ov, char))
            }) else {
                write!(line, "{} ", stripe.bg()).expect("Writing to a String can't fail");
                return line;
            };
            let fg = overlay.foreground().fg();
            let bg = overlays
                .iter()
                .skip(index + 1)
                .find(|overlay| overlay.at_pos(x, y, size).is_some())
                .map_or_else(|| stripe.bg(), |overlay| overlay.foreground().bg());

            write!(line, "{bg}{fg}{ch}").expect("Writing to a String can't fail");
            line
        });
        println!("{line}{RESET}");
    }
}
