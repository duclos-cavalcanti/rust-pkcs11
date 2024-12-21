mod message;

use std::error::Error;

use message::{Message, MessageFlag};
use tokio::sync::mpsc;

fn consume(msg: &Message) -> Option<bool> {
    println!("SERVER RECEIVED: id={} | flag={:?} | data={}", msg.id, msg.flag, msg.data);
    if let MessageFlag::End = msg.flag {
        return None;
    }
    Some(true)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (sender, mut receiver) = mpsc::channel::<Message>(10);
    let handle: tokio::task::JoinHandle<Result<(), &str>> = tokio::spawn(async move {
        while let Some(msg) = receiver.recv().await {
            if let None = consume(&msg) {
                break;
            }
        }
        Ok(())
    });

    for i in 0..6 {
        let mut msg = Message::new(i, MessageFlag::Req, "Hello from Main!");
        if i == 5 {
            msg.id   = 999;
            msg.flag = MessageFlag::End;
        }

        sender.send(msg).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    handle.abort();
    Ok(())
}
