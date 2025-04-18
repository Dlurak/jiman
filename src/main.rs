use jiman::color::{AnsiCode, RESET};

const WIDTH: usize = 60;
const HEIGHT: usize = 14;

fn main() {
    let name = std::env::args().nth(1).unwrap_or("asexual".into());
    let Some(flag) = jiman::flag::Flag::by_name(&name) else {
        println!("Not found");
        std::process::exit(1);
    };
    let stripe_height = HEIGHT / flag.stripes.len();

    for stripe_color in flag.stripes {
        let bg = stripe_color.bg();
        let line = format!("{bg}{}{RESET}\n", " ".repeat(WIDTH));
        let stripe = line.repeat(stripe_height);
        print!("{stripe}");
    }
}
