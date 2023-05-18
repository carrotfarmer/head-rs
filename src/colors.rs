use rand::{Rng, thread_rng};

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Red,
    CyanBold,
    Cyan,
    GreenBold,
    Green,
    YellowBold,
    Yellow,
    BlueBold,
    Blue,
    MagentaBold,
    Magenta
}

pub fn colorize(text: &str, color: Color) -> String {
    match color {
        Color::Red => format!("\x1b[31m{}\x1b[0m", text),
        Color::CyanBold => format!("\x1b[1;36m{}\x1b[0m", text),
        Color::Cyan => format!("\x1b[36m{}\x1b[0m", text),
        Color::GreenBold => format!("\x1b[1;32m{}\x1b[0m", text),
        Color::Green => format!("\x1b[32m{}\x1b[0m", text),
        Color::YellowBold => format!("\x1b[1;33m{}\x1b[0m", text),
        Color::Yellow => format!("\x1b[33m{}\x1b[0m", text),
        Color::BlueBold => format!("\x1b[1;34m{}\x1b[0m", text),
        Color::Blue => format!("\x1b[34m{}\x1b[0m", text),
        Color::MagentaBold => format!("\x1b[1;35m{}\x1b[0m", text),
        Color::Magenta => format!("\x1b[35m{}\x1b[0m", text),
    }
}

pub fn pick_rand_color() -> (Color, Color) {
    let mut rng = thread_rng();
    let rand_num: u8 = rng.gen_range(0..5); 

    match rand_num {
        0 => (Color::CyanBold, Color::Cyan),
        1 => (Color::GreenBold, Color::Green),
        2 => (Color::YellowBold, Color::Yellow),
        3 => (Color::BlueBold, Color::Blue),
        4 => (Color::MagentaBold, Color::Magenta),
        _ => (Color::Red, Color::Red),
    } 
}
