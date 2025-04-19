use clap::Parser;
use jiman::{
    cli::Cli,
    color::{AnsiCode, RESET},
};

fn main() {
    let cli = Cli::parse();
    let width = cli
        .width
        .and_then(|w| w.absolute_width())
        .map_or(60, |v| v.get());
    let height = cli
        .height
        .and_then(|h| h.absolute_height())
        .map_or(14, |v| v.get());

    let stripes = cli.flag.stripes();
    let stripe_height = height / stripes.len();

    for stripe_color in stripes {
        let bg = stripe_color.bg();
        let line = format!("{bg}{}{RESET}\n", " ".repeat(width));
        let stripe = line.repeat(stripe_height);
        print!("{stripe}");
    }
}
