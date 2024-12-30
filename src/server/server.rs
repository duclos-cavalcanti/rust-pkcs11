use std::error::Error;

use super::socket::Socket;
use super::message::{Message, ProtoMessage, ProtoMessageType};
use super::manager::Manager;
use super::log::{Logger, Level};
use super::handler::{Handler, ListHandler};

pub struct Server {
    manager: Manager,
    socket: Socket,
    logger: Logger
}

impl Server {
    pub fn new(ipaddr: &str, port: i32) -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new()?;
        let socket = Socket::new(ipaddr, port)?;
        let mut logger = Logger::new(None)?;
        
        logger.log(format!("SERVER LISTENING ON {}:{}", ipaddr, port), None)?;
        Ok(Self {socket:socket, manager:manager, logger:logger})
    }

    pub fn handle(&mut self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let handler = match message.flag {
            f if f == ProtoMessageType::List as i32 => ListHandler::new(&self.manager),
            _ => return Err(Box::from(format!("Unexpected flag type: {}", message.flag))) 
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
            let m =  match ProtoMessage::decode(&buf[..n]) {
                Ok(m) => m,
                Err(e) => return Err(Box::from(format!("Failed Deserialization: {}", e.to_string())))
            };
            self.logger.log(format!("SERVER RECV: {:?}", m), Some(Level::EVENT))?;
            Ok(Some(m))
        }
    }

    pub fn error(&mut self, message: &ProtoMessage, e: Box<dyn Error>) -> Result<usize, Box<dyn Error>> {
        let reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            integer: 0,
            repeat: 0,
            err: true,
            data: vec!{e.to_string()},
        };

        self.logger.log(format!("SERVER SENT: {:?}", message), Some(Level::URGENT))?;
        let n = self.send(&reply)?;
        Ok(n)
    }

    pub fn send(&mut self, message: &ProtoMessage) -> Result<usize, Box<dyn Error>> {
        let mut buf = Vec::new();
        let n = match message.encode(&mut buf) {
            Ok(_)  => buf.len(),
            Err(e) => return Err(Box::from(format!("Failed Serialization: {}", e.to_string())))
        };
        self.logger.log(format!("SERVER SENT: \n{:?}", message), Some(Level::EVENT))?;
        self.socket.send(&buf[..n])?;
        Ok(n)
    }
}
