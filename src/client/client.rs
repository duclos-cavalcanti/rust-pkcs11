use crate::message::{ProtoMessageType, ProtoFactory};
use crate::socket::Socket;

use std::sync::{mpsc::Sender};
use std::net::{TcpStream};
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
    sender: Sender<String>
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.sender.send(String::from("CLIENT CLOSED")).unwrap();
    }
}

impl Client {
    pub fn new(ipaddr: &str, port: i32, sender:Sender<String>) -> Result<Self, Box<dyn Error>> {
        let sender = sender;
        let stream = match TcpStream::connect(format!("{}:{}", ipaddr, port)) {
            Ok(s)  => s,
            Err(e) => return Err(Box::from(format!("Error: {}", e)))
        };

        let client = Client {
            socket: Socket::new(stream),
            tick: 1,
            sender: sender
        };

        Ok(client)
    }

    pub fn request(&mut self, request: &Request) -> Result<(), Box<dyn Error>> {
        // receive message
        let message = match request.mtype {
            ProtoMessageType::List      => ProtoFactory::list(self.tick),
            ProtoMessageType::Encrypt   => ProtoFactory::enc(self.tick, request.i, request.s.clone(), request.data.clone()),
            ProtoMessageType::Sign      => ProtoFactory::sign(self.tick, request.i, request.s.clone(), request.data.clone()),
            _ => return Err(Box::from("Invalid state"))
        };

        self.tick += 1;

        // send message
        match self.socket.send(&message) { 
            Ok(_)  => { 
                self.sender.send(format!("CLIENT SENT: {:?}", message)).unwrap();
            },
            Err(e) => return Err(Box::from(format!("{}", e))),
        }

        // receive message
        match self.socket.recv() {
            Ok(Some(reply)) => { 
                self.sender.send(format!("CLIENT RECV: {:?}", reply)).unwrap();
            },
            Ok(None)        => return Err(Box::from("Connection closed")),
            Err(e)          => return Err(Box::from(format!("{}", e))),
        }

        Ok(())
    }
}

