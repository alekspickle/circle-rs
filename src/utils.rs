use termion::{color, style};

pub fn print_green(s: &str) -> String {
    let green = color::Fg(color::LightGreen);
    format!("{}{}{}", green, s, color::Fg(color::Reset))
}
