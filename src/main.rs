use clap::Parser;
use jiman::{
    cli::{Cli, Command, PrintCli},
    color::{AnsiCode, RESET},
    flag::Flag,
    triangle,
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
                        println!("{} ({alts})", variant);
                    }
                    None => {
                        println!("{}", variant);
                    }
                }
            }
        }
        Command::Print(cli) => print_handler(cli),
    }
}

fn print_handler(cli: PrintCli) {
    let width = match cli.width {
        Some(w) => w.absolute_width().map(|v| v.get()),
        None => term_size::dimensions_stdout().map(|(w, _)| w.min(70)),
    };
    let width = width.unwrap_or(70);
    let height = cli
        .height
        .and_then(|h| h.absolute_height())
        .map_or(16, |v| v.get());

    let stripes = cli.flag.stripes();
    let stripe_height = height / stripes.len();
    let height = stripe_height * stripes.len();

    let overlays = cli.flag.overlays();

    for y in 0..height {
        let stripe = &stripes[y / stripe_height];
        let line = (0..width).fold(String::new(), |mut line, x| {
            let Some((tr, overlay)) = overlays.iter().find_map(|o| {
                if !(o.padding..height - o.padding).contains(&y) {
                    return None;
                }
                let tr = triangle::TriangleChar::at_pos(
                    height - o.padding * 2,
                    cli.triangle_angle.get(),
                    (x, y - o.padding),
                    o.insert,
                )?;
                Some((tr, o))
            }) else {
                return match write!(line, "{} ", stripe.bg()) {
                    Ok(_) => line,
                    Err(_) => format!("{line}{} ", stripe.bg()),
                };
            };
            let fg = overlay.fg.fg();
            let bg = overlay.bg.map(|bg| bg.bg()).unwrap_or_else(|| stripe.bg());
            let c = char::from(tr);

            match write!(line, "{bg}{fg}{c}") {
                Ok(_) => line,
                Err(_) => format!("{line}{bg}{fg}{c}"),
            }
        });
        println!("{line}{RESET}");
    }
}
