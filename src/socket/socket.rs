use std::error::Error;
use std::net::{TcpStream};
use std::io::{Read, Write};

use crate::message::{ProtoMessage, ProtoFactory};

pub struct Socket {
    stream: TcpStream,
}

impl Socket {
    pub fn new(stream: TcpStream) -> Self {
        Self {stream: stream}
    }

    pub fn recv(&mut self) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        let mut buf = vec![0u8; 1024];
        let n = match self.stream.read(&mut buf) {
            Ok(n)  => n, 
            Err(e) => return Err(Box::from(format!("{}", e.to_string()))),
        };

        if n == 0 {
            Ok(None)
        } else {
            let message = ProtoFactory::decode(&mut buf, n)?;
            Ok(Some(message))
        }
    }

    pub fn send(&mut self, message: &ProtoMessage) -> Result<usize, Box<dyn Error>> {
        let (buf, n) = ProtoFactory::encode(message)?;
        self.stream.write_all(&buf[..n])?;
        Ok(n)
    }
}
