pub mod cli;
pub mod color;
pub mod flag;
pub mod overlay;

pub mod term {
    use std::env::var;

    pub fn true_color() -> bool {
        var("COLORTERM").is_ok_and(|val| {
            let lowercased = val.to_lowercase();
            lowercased == "truecolor" || lowercased == "24bit"
        })
    }
}
