use colorized::{Color, Colors};

pub fn blue(text: &str) -> String {
    format!("{}", text.color(Colors::BrightBlueFg))
}

pub fn red(text: &str) -> String {
    format!("{}", text.color(Colors::BrightRedFg))
}

pub fn green(text: &str) -> String {
    format!("{}", text.color(Colors::BrightGreenFg))
}
