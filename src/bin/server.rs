use std::error::Error;
use std::sync::mpsc;
use std::thread;

use rust_pkcs11::server::{Server};

fn main() -> Result<(), Box<dyn Error>> {
    let (sender, receiver) = mpsc::channel::<String>();
    let handle = thread::spawn(move || {
            let result = Server::new("127.0.0.1", 9091, sender);
            let Ok(server) = result else {
                eprintln!("Error: {}", result.err().unwrap());
                return;
            };
            if let Err(e) = server.serve() {
                eprintln!("Error: {}", e);
                return;
            }
    });

    while let Ok(text) = receiver.recv() {
        println!("{}", text);
    }

    if let Err(_) = handle.join() {
        eprintln!("Thread panicked");
    }

    Ok(())
}
