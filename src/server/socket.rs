use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

pub struct Socket {
    listener: TcpListener,
    stream: TcpStream,
}

impl Socket {
    pub fn new(ipaddr: &str, port: i32) -> Result<Self, Box<dyn Error>> {
        let addr        = format!("{}:{}", ipaddr, port);
        let listener    = TcpListener::bind(addr)?;
        let stream  = match listener.accept() {
            Ok((s, _)) => s,
            Err(_)     =>  return Err(Box::from("Failed Connection")),
        };
        Ok(Self {listener:listener, stream: stream})
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>> {
        let mut tmp = [0u8; 1024];
        let n = self.stream.read(&mut tmp)?;
        buf[..n].copy_from_slice(&tmp[..n]);
        Ok(n)
    }

    pub fn send(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        self.stream.write_all(buf)?;
        Ok(())
    }
}
