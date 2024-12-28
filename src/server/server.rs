use std::error::Error;

use super::socket::Socket;
use super::message::{Message, ProtoMessage, ProtoMessageType};

pub struct Server {
    socket: Socket,
    buf:  Vec<u8>,
}

impl Server {
    pub fn new(ipaddr: &str, port: i32) -> Result<Self, Box<dyn Error>> {
        let socket = Socket::new(ipaddr, port)?;
        Ok(Self {socket:socket, buf: vec![0u8; 1024]})
    }

    pub fn recv(&mut self) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        let n = match self.socket.recv(&mut self.buf) {
            Ok(n)  => n, 
            Err(e) => return Err(Box::from(format!("{}", e.to_string()))),
        };

        if n == 0 {
            Ok(None)
        } else {
            return match ProtoMessage::decode(&self.buf[..n]) {
                Ok(m) => Ok(Some(m)),
                Err(e) => Err(Box::from(format!("Failed Deserialization: {}", e.to_string())))
            }
        }
    }

    pub fn send(&mut self, message: &ProtoMessage) -> Result<usize, Box<dyn Error>> {
        let mut buf = Vec::new();
        let n = match message.encode(&mut buf) {
            Ok(_)  => buf.len(),
            Err(e) => return Err(Box::from(format!("Failed Serialization: {}", e.to_string())))
        };
        self.socket.send(&buf[..n])?;
        Ok(n)
    }
}
