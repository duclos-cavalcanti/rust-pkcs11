use crate::message::{ProtoMessage, ProtoMessageType};
use crate::utils::log::{Logger, Level};

use prost::Message;

use super::socket::Socket;
use std::error::Error;


#[derive(Hash, Eq, PartialEq, Debug)]
pub enum State {
    Start,
    List,
    Login,
    Encrypt,
    Exit,
}

pub struct Request {
    pub state: State,
}

pub struct Client {
    socket: Socket,
    tick: i32,
    logger: Logger
}

impl Client {
    pub fn new(ipaddr: &str, port: i32) -> Result<Self, Box<dyn Error>> {
        let mut logger = Logger::new(Some(".client.log"))?;
        let client = Client {
            socket: Socket::new(ipaddr, port, &mut logger)?,
            tick: 0,
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
            let m =  match ProtoMessage::decode(&buf[..n]) {
                Ok(m) => m,
                Err(e) => return Err(Box::from(format!("Failed Deserialization: {}", e.to_string())))
            };
            self.logger.log(format!("CLIENT RECV: {:?}", m), Some(Level::EVENT))?;
            Ok(Some(m))
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

    pub fn exchange(&mut self, message: &ProtoMessage) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        self.send(message)?;
        self.recv()
    }

    pub fn request(&mut self, request: &Request) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        match request.state {
            State::List => self.list_request(),
            _ => Err(Box::from("Invalid state"))
        }
    }

    pub fn list_request(&mut self) -> Result<Option<ProtoMessage>, Box<dyn Error>> {
        self.tick += 1;
        let message = ProtoMessage {
            id: self.tick,
            flag: ProtoMessageType::List as i32,
            integer: 0,
            err: false,
            data: vec!{},
        };
        let reply = self.exchange(&message)?;
        Ok(reply)
    }
}

