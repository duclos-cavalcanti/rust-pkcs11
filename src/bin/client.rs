use std::thread;
use std::sync::mpsc;
use std::env;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rust_pkcs11::message::{ProtoMessageType};
use rust_pkcs11::client::{Client, Request};

fn args() -> Result<usize, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let n = 
    if args.len() > 1 {
        let arg = args[1].clone();
        let ret = arg.parse::<usize>();
        let Ok(n) = ret else {
            return Err(Box::from(format!("Invalid number of clients argument: {}", arg)));
        };
        n
    } else {
        1
    };
    println!("Number of clients: {}", n);
    Ok(n)
}

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
            "Decrypt" => ProtoMessageType::Decrypt,
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
    let n = args()?;
    let (sender, receiver) = mpsc::channel::<String>();
    let requests        = parse()?;
    let mut handles     = Vec::new();
    
    // logger thread
    handles.push(
        thread::spawn(move || {
            while let Ok(text) = receiver.recv() {
                println!("{}", text);
            }
        })
    );

    // create n clients
    for _ in 0..n {
        let sender_clone   = sender.clone();
        let requests_clone = requests.to_vec();
        handles.push(
            thread::spawn(move || {
                let result = Client::new("127.0.0.1", 9091, sender_clone.clone());
                let Ok(mut client) = result else {
                    sender_clone.send(format!("Error: {}", result.err().unwrap())).unwrap();
                    return;
                };
                if let Err(e) = client.request(&requests_clone) {
                    sender_clone.send(format!("Error processing request: {}", e)).unwrap();
                }
            })
        );
    }

    drop(sender);

    for handle in handles {
        if let Err(_) = handle.join() {
            eprintln!("Thread panicked");
        }
    }

    Ok(())
}
