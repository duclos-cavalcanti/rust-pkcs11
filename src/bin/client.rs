#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rust_pkcs11::message::{ProtoMessageType};
use rust_pkcs11::client::{Client, Request};

fn parse() -> Result<Vec<Request>, Box<dyn Error>> {
    let mut ret = Vec::new();
    let file = File::open("REQUESTS.CSV")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').map(str::trim).collect();
        if parts.len() < 4 {
            return Err(Box::from("Invalid format. Expected: Type,SlotID,PIN,Data"));
        }

        let mtype = match parts[0] {
            "Encrypt" => ProtoMessageType::Encrypt,
            "Sign" => ProtoMessageType::Sign,
            _ => return Err(Box::from("Unknown command type")),
        };

        let slot_id = parts[1].parse::<u64>()?;
        let pin = parts[2].to_string();
        let data = parts[3].to_string();

        ret.push(Request::new(mtype, Some(slot_id), Some(pin), Some(data)))
    }

    Ok(ret)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new("127.0.0.1", 9091)?;
    let requests   = parse()?;
    for r in requests {
        let _ = client.request(&r)?;
    }

    Ok(())
}
