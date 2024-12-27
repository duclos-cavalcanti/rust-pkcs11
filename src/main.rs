use std::error::Error;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;

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

    let mut rx = vec![0u8; 1024];
    loop {
        let n = match socket.read(&mut rx).await {
            Ok(0) => {
                println!("SOCKET CLOSED");
                break;
            },
            Ok(n) => n,
            Err(e) => {
                return Err(Box::from(format!("Failed to read: {}", e.to_string())))
            }
        };

        let m = message::deser(&rx[..n])?;
        println!("RECV: {:?}", m);

        let m = message::message(m.id, 
                                 message::ProtoMessageType::Ack, 
                                 Some(42), 
                                 vec!["Foo".to_string(), "Bar".to_string()]);
        let tx = message::ser(&m)?;
        socket.write_all(&tx[..tx.len()]).await?;
        println!("SENT: {:?}", m);
    }

    Ok(())
}
