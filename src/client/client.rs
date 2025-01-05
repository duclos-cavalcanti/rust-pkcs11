use crate::message::{ProtoMessageType, ProtoFactory};
use crate::socket::Socket;
use crate::utils::log::{ConcurrentLogger, Level};

use std::sync::{mpsc::Sender};
use std::net::{TcpStream};
use std::error::Error;
use std::collections::HashMap;

use rand::Rng;
use base64::prelude::*;

#[derive(Debug, Clone)]
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
    logger: ConcurrentLogger, 
    tick: i32,
    history: HashMap<String, String>,
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.logger.log(String::from("CLIENT CLOSED"), Some(Level::INFO));
    }
}

impl Client {
    pub fn new(ipaddr: &str, port: i32, sender:Sender<String>) -> Result<Self, Box<dyn Error>> {
        let addr = format!("{}:{}", ipaddr, port);
        let logger = ConcurrentLogger::new(sender)?;
        let stream = match TcpStream::connect(addr.clone()) {
            Ok(s)  => s,
            Err(e) => return Err(Box::from(format!("Error: {}", e)))
        };
        let map = HashMap::new();

        logger.log(format!("CLIENT CONNECTED: {}", addr), Some(Level::INFO))?;
        let client = Client {
            socket: Socket::new(stream),
            logger: logger, 
            tick: 1, 
            history: map
        };

        Ok(client)
    }

    pub fn request(&mut self, requests: &Vec<Request>) -> Result<(), Box<dyn Error>> {
        // rng init
        let mut rng = rand::thread_rng();

        for request in requests {
            // create request ID
            let id: [u8; 16] = rng.gen();
            let id = BASE64_STANDARD.encode(id);

            // form message
            let message = match request.mtype {
                ProtoMessageType::List      => ProtoFactory::list(id, self.tick),
                ProtoMessageType::Encrypt   => ProtoFactory::enc(id,  self.tick, request.i, &request.s, &request.data),
                ProtoMessageType::Sign      => ProtoFactory::sign(id, self.tick, request.i, &request.s, &request.data),
                ProtoMessageType::Decrypt   =>  {
                    let Some(data) = self.history.get(&request.data) else {
                        self.logger.log(format!("To-be-decrypted data not found in history: {}", request.data), Some(Level::URGENT))?;
                        continue;
                    };
                    ProtoFactory::dec(id, self.tick, request.i, &request.s, &data)
                }
                _ => return Err(Box::from("Invalid state"))
            };

            // increment tick
            self.tick += 1;

            // send message
            match self.socket.send(&message) { 
                Ok(_)  => { 
                    self.logger.log(format!("CLIENT SENT: {:?}", message), Some(Level::EVENT))?;
                },
                Err(e) => return Err(Box::from(format!("{}", e))),
            }

            // receive message
            match self.socket.recv() {
                Ok(Some(reply)) => { 
                    let mut level = Level::EVENT;
                    if reply.err { level = Level::URGENT; }
                    self.logger.log(format!("CLIENT RECV: {:?}", reply), Some(level))?;

                    if message.flag == ProtoMessageType::Encrypt as i32 {
                        self.history.insert(message.data[0].clone(), reply.data[0].clone());
                    }
                },
                Ok(None)        => return Err(Box::from("Connection closed")),
                Err(e)          => return Err(Box::from(format!("{}", e))),
            }
        }
        Ok(())
    }
}

