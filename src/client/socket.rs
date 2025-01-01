use crate::utils::log::{Logger};

use std::error::Error;
use std::net::{TcpStream};
use std::io::{Read, Write};


pub struct Socket {
    stream: TcpStream,
}

impl Socket {
    pub fn new(ipaddr: &str, port: i32, logger: &mut Logger) -> Result<Self, Box<dyn Error>> {
        let addr = format!("{}:{}", ipaddr, port);
        let stream = match TcpStream::connect(addr.clone()) {
            Ok(s)  => s,
            Err(e) => return Err(Box::from(format!("Error: {}", e)))
        };

        logger.log(format!("CLIENT CONNECTED TO {}", addr.clone()), None)?;
        Ok(Self {stream: stream})
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
