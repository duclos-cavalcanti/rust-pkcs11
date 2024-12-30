#![allow(dead_code)]

use std::error::Error;

mod utils;
mod server;

use server::Server;

fn main() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new("127.0.0.1", 9091)?;

    loop {
        let Some(message) = server.recv()? else {
            break;
        };

        let _n = match server.handle(&message) {
            Ok(reply) => server.send(&reply)?,
            Err(e)    => server.error(&message, e)?
        };
    }

    Ok(())
}
