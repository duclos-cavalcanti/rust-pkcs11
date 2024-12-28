#![allow(dead_code)]

use std::error::Error;

mod utils;
mod server;

use server::Server;
use server::message::{ProtoMessage, ProtoMessageType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new("127.0.0.1", 9091)?;

    loop {
        let Some(message) = server.recv()? else {
            break;
        };

        println!("RECV: {:?}", message);
        let data: Vec<String> = message.data
            .iter()
            .cloned()
            .chain(std::iter::once("Back".to_string()))
            .collect();

        let reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            integer: 0,
            repeat: data.len() as i32,
            data: data,
        };

        server.send(&reply)?;
        println!("RECV: {:?}", message);
    }

    Ok(())
}
