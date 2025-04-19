use crate::color::AnsiCode;

pub struct Overlay<F, B>
where
    F: AnsiCode,
    B: AnsiCode,
{
    pub bg: Option<B>,
    pub padding: usize,
    pub insert: usize,
    pub fg: F,
}

impl<F, B> Overlay<F, B>
where
    F: AnsiCode,
    B: AnsiCode,
{
    pub fn new(bg: Option<B>, padding: usize, insert: usize, fg: F) -> Self {
        Self {
            bg,
            padding,
            insert,
            fg,
        }
    }
}

pub fn new_overlays<F>(params: &[(F, usize, usize)]) -> Vec<Overlay<F, F>>
where
    F: AnsiCode + Copy,
{
    let mut overlays = Vec::with_capacity(params.len());

    for (i, (fg, padding, insert)) in params.iter().enumerate() {
        let bg = params.get(i + 1).map(|x| x.0);
        overlays.push(Overlay::new(bg, *padding, *insert, *fg));
    }

    overlays
}
