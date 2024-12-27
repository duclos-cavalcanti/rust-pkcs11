use std::error::Error;

use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

mod manager;
mod message;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager    = manager::Manager::new()?;
    println!("BINDING");

    let connection = TcpListener::bind("127.0.0.1:9091").await?;
    println!("SERVER UP ON: 127.0.0.1:9091");

    let (mut socket, addr) = connection.accept().await?;
    println!("CONNECTION AVAILABLE: {:?}", addr);

    let mut buf = vec![0u8; 1024];
    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => {
                println!("Socket closed");
                break;
            },
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {:?}", e);
                break;
            }
        };

        let m = message::deser(&buf[..n])?;
        println!("Message: {:?}", m);
    }

    Ok(())
}
