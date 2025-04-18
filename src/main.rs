use clap::Parser;
use jiman::{
    cli::Cli,
    color::{AnsiCode, RESET},
};

const WIDTH: usize = 60;
const HEIGHT: usize = 14;

fn main() {
    let cli = Cli::parse();
    let stripes = cli.flag.stripes();
    let stripe_height = HEIGHT / stripes.len();

    for stripe_color in stripes {
        let bg = stripe_color.bg();
        let line = format!("{bg}{}{RESET}\n", " ".repeat(WIDTH));
        let stripe = line.repeat(stripe_height);
        print!("{stripe}");
    }
}
