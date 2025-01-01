#![allow(dead_code)]

use std::error::Error;
use std::io::{self, Write};
use colored::Colorize;

use rust_pkcs11::client::{Client, Request, State};

pub fn capture(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt.blue());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new("127.0.0.1", 9091)?;

    loop {
        let prompt  = "Enter command (l = list slots, s = session, c = create, q = exit): ";
        let command = capture(prompt)?;

        match command.as_str() {
            "l" => {
                client.request(
                    &Request{
                        state: State::List
                    }
                )?;
            }

            "q" => {
                println!("{}", "Exiting...".green());
                break;
            }

            _ => {
                println!("{}", "Unknown command...".red());
                break;
            }  

        }
    }

    Ok(())
}
