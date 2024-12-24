use std::error::Error;
use std::io::{self, Write};

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

pub fn capture(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", blue(prompt));
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
