#![allow(dead_code)]

use std::error::Error;
use std::io::{self, Write};
use colored::Colorize;

use rust_pkcs11::message::{ProtoMessageType};
use rust_pkcs11::client::{Client, Request};

fn parse(text: &str) -> Result<u64, std::num::ParseIntError> {
   text.parse::<u64>()
}

fn capture(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}: ", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new("127.0.0.1", 9091)?;
    let menu  = "Menu (l = list slots, e = encrypt, s = sign, c = create, q = exit)";
    println!("{}", menu);

    loop {
        let command = capture("Command")?;
        match command.as_str() {
            "l" => {
                let data = client.request(
                    &Request::new(ProtoMessageType::List, 
                                  None, 
                                  None, 
                                  None)
                )?;
                for entry in data {
                    println!("{}", entry.blue());
                }
            }

            "e" => {
                let slot = capture("SlotID")?;
                let pin  = capture("PIN")?;
                let data  = capture("Data")?;
                let data = client.request(
                    &Request::new(ProtoMessageType::Encrypt, 
                                  Some(parse(&slot)?),
                                  Some(pin), 
                                  Some(data)) 
                )?;
                for entry in data {
                    println!("{}", entry.blue());
                }
            }

            "s" => {
                let slot = capture("SlotID")?;
                let pin  = capture("PIN")?;
                let data  = capture("Data")?;
                let data = client.request(
                    &Request::new(ProtoMessageType::Sign, 
                                  Some(parse(&slot)?),
                                  Some(pin), 
                                  Some(data)) 
                )?;
                for entry in data {
                    println!("{}", entry.blue());
                }
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
