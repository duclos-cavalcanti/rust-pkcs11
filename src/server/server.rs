use std::error::Error;

use crate::message::{ProtoMessage, ProtoMessageType, ProtoFactory};
use crate::utils::log::{Logger, Level};

use super::socket::Socket;
use super::manager::Manager;
use super::handler::{Handler, ListHandler, EncryptHandler, SignHandler};

pub struct Server {
    manager: Manager,
    socket: Socket,
    logger: Logger
}

impl Server {
    pub fn new(ipaddr: &str, port: i32) -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new()?;
        let mut logger = Logger::new(None)?;
        let socket = Socket::new(ipaddr, port, &mut logger)?;
        Ok(Self {socket:socket, manager:manager, logger:logger})
    }

    pub fn handle(&mut self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut handler: Box<dyn Handler> = match message.flag {
            f if f == ProtoMessageType::List as i32 => Box::new(ListHandler::new(&mut self.manager)),
            f if f == ProtoMessageType::Encrypt as i32 => Box::new(EncryptHandler::new(&mut self.manager)),
            f if f == ProtoMessageType::Sign as i32 => Box::new(SignHandler::new(&mut self.manager)),
            _ => return Err(Box::from(format!("Unexpected flag type: {}", message.flag))),
        };

        let reply = handler.process(&message)?;
        Ok(reply)
    }

    pub fn recv(&mut self) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        let mut buf = vec![0u8; 1024];
        let n = match self.socket.recv(&mut buf) {
            Ok(n)  => n, 
            Err(e) => return Err(Box::from(format!("{}", e.to_string()))),
        };

        if n == 0 {
            self.logger.log(format!("SERVER CLOSED"), None)?;
            Ok(None)
        } else {
            let m = ProtoFactory::decode(&mut buf, n)?;
            self.logger.log(format!("SERVER RECV: {:?}", m), Some(Level::EVENT))?;
            Ok(Some(m))
        }
    }

    pub fn error(&mut self, message: &ProtoMessage, e: Box<dyn Error>) -> Result<usize, Box<dyn Error>> {
        let reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            slot_id: 0,
            pin: String::new(),
            err: true,
            data: vec!{e.to_string()},
        };

        self.logger.log(format!("ERROR HANDLING MESSAGE: {}", e), Some(Level::URGENT))?;
        let n = self.send(&reply)?;
        Ok(n)
    }

    pub fn send(&mut self, message: &ProtoMessage) -> Result<usize, Box<dyn Error>> {
        let (buf, n) = ProtoFactory::encode(message)?;
        self.logger.log(format!("SERVER SENT: {:?}", message), Some(Level::EVENT))?;
        self.socket.send(&buf[..n])?;
        Ok(n)
    }
}
