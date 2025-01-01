use crate::message::{ProtoMessage, ProtoMessageType, ProtoFactory};
use crate::utils::log::{Logger, Level};

use super::socket::Socket;
use std::error::Error;

pub struct Request {
    mtype: ProtoMessageType,
    i: u64,
    s: String,
    data: String,
}

impl Request {
    pub fn new(mtype: ProtoMessageType, i: Option<u64>, s: Option<String>, data: Option<String>) -> Self {
        Self {
            mtype: mtype, 
            i: i.unwrap_or(0),
            s: s.unwrap_or(String::new()), 
            data: data.unwrap_or(String::new()), 
        }
    }
}

pub struct Client {
    socket: Socket,
    tick: i32,
    logger: Logger
}

impl Client {
    pub fn new(ipaddr: &str, port: i32) -> Result<Self, Box<dyn Error>> {
        let mut logger = Logger::new(Some(".pkcs11.log"))?;
        let client = Client {
            socket: Socket::new(ipaddr, port, &mut logger)?,
            tick: 1,
            logger: logger
        };

        Ok(client)
    }

    pub fn recv(&mut self) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        let mut buf = vec![0u8; 1024];
        let n = match self.socket.recv(&mut buf) {
            Ok(n)  => n, 
            Err(e) => return Err(Box::from(format!("{}", e.to_string()))),
        };

        if n == 0 {
            Ok(None)
        } else {
            let m = ProtoFactory::decode(&mut buf, n)?;
            self.logger.log(format!("CLIENT RECV: {:?}", m), Some(Level::EVENT))?;
            Ok(Some(m))
        }
    }

    pub fn send(&mut self, message: &ProtoMessage) -> Result<usize, Box<dyn Error>> {
        let (buf, n) = ProtoFactory::encode(message)?;
        self.logger.log(format!("CLIENT SENT: {:?}", message), Some(Level::EVENT))?;
        self.socket.send(&buf[..n])?;
        Ok(n)
    }

    pub fn exchange(&mut self, message: &ProtoMessage) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        self.send(message)?;
        self.recv()
    }

    pub fn request(&mut self, request: &Request) -> Result<Vec<String>, Box<dyn Error>> {
        let message = match request.mtype {
            ProtoMessageType::List      => ProtoFactory::list(self.tick),
            ProtoMessageType::Encrypt   => ProtoFactory::enc(self.tick, request.i, request.s.clone(), request.data.clone()),
            ProtoMessageType::Sign      => ProtoFactory::sign(self.tick, request.i, request.s.clone(), request.data.clone()),
            _ => return Err(Box::from("Invalid state"))
        };

        self.tick += 1;
        match self.exchange(&message) {
            Ok(Some(reply)) => Ok(reply.data),
            Ok(None)        => Err(Box::from("Unexpected connection closure")),
            Err(e)          => Err(Box::from(format!("{}", e))),
        }
    }
}

