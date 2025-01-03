use std::error::Error;
use std::sync::{Arc, Mutex, mpsc::Sender};
use std::net::{TcpListener};

use crate::message::{ProtoMessageType, ProtoFactory, ProtoError};
use crate::socket::Socket;

use super::manager::Manager;
use super::handler::{Handler, ListHandler, EncryptHandler, SignHandler};

pub struct Server {
    manager:    Arc<Mutex<Manager>>,
    sender:     Sender<String>,
    listener:   TcpListener,
}

impl Server {
    pub fn new(ipaddr: &str, port: i32, sender:Sender<String>) -> Result<Self, Box<dyn Error>> {
        let listener    = TcpListener::bind(format!("{}:{}", ipaddr, port))?;
        let manager     = Arc::new(Mutex::new(Manager::new()?));
        let sender      = sender;
        Ok(Self {manager:manager, sender:sender, listener: listener})
    }

    pub fn serve(&self) -> Result<(), Box<dyn Error>> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let manager = Arc::clone(&self.manager);
                    let sender  = self.sender.clone();
                    std::thread::spawn(move || {
                        let mut socket = Socket::new(stream);
                        if let Err(e) = Self::handle(&mut socket, manager, sender) {
                            eprintln!("{}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn handle(
        socket: &mut Socket,
        manager: Arc<Mutex<Manager>>,
        sender: Sender<String>
    ) -> Result<(), Box<dyn Error>> {
        let mut mgr   = manager.lock().unwrap();

        // receive message
        let message = match socket.recv() {
            Ok(None)    => {
                sender.send(String::from("CONNECTION CLOSED")).unwrap();
                return Ok(());
            },
            Ok(Some(message)) => {
                sender.send(format!("SERVER RECV: {:?}", message)).unwrap();
                message
            },
            Err(e) => {
                let reply = ProtoFactory::err( ProtoError::HandlingError(0, e.to_string()));
                sender.send(format!("SERVER SENT: {:?}", reply)).unwrap();
                let _ = socket.send(&reply); 
                return Err(e);
            }
        };

        // create handler
        let mut handler: Box<dyn Handler> = match message.flag {
            f if f == ProtoMessageType::List as i32 => Box::new(ListHandler::new(&mut *mgr)),
            f if f == ProtoMessageType::Encrypt as i32 => Box::new(EncryptHandler::new(&mut *mgr)),
            f if f == ProtoMessageType::Sign as i32 => Box::new(SignHandler::new(&mut *mgr)),
            _ => return Err(Box::from(format!("Unexpected flag type: {}", message.flag))),
        };

        // handle message
        match handler.process(&message) {
            Ok(reply)    => {
                sender.send(format!("SERVER SENT: {:?}", reply)).unwrap();
                let _ = socket.send(&reply); 
                return Ok(());
            },
            Err(e) => {
                let reply = ProtoFactory::err( ProtoError::MessageError(message, e.to_string()));
                sender.send(format!("SERVER SENT: {:?}", reply)).unwrap();
                let _ = socket.send(&reply); 
                return Err(e);
            }
        }
    }
}
