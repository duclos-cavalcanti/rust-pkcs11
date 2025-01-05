use std::error::Error;
use std::sync::{Arc, mpsc::Sender};
use std::net::{TcpListener};

use crate::message::{ProtoMessageType, ProtoFactory, ProtoError};
use crate::utils::log::{ConcurrentLogger, Level};
use crate::socket::Socket;

use super::manager::Manager;
use super::handler::{Handler, ListHandler, EncryptHandler, SignHandler, DecryptHandler};

pub struct Server {
    manager:    Arc<Manager>,
    logger:     Arc<ConcurrentLogger>,
    listener:   TcpListener,
}

impl Server {
    pub fn new(ipaddr: &str, port: i32, sender:Sender<String>) -> Result<Self, Box<dyn Error>> {
        let addr        = format!("{}:{}", ipaddr, port);
        let manager     = Arc::new(Manager::new()?);
        let listener    = TcpListener::bind(addr.clone())?;
        let logger      = Arc::new(ConcurrentLogger::new(sender)?);

        logger.log(format!("SERVER BOUND: {}", addr), None)?;
        Ok(Self {manager:manager, logger:logger, listener: listener})
    }

    pub fn serve(&self) -> Result<(), Box<dyn Error>> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let manager = Arc::clone(&self.manager);
                    let logger  = Arc::clone(&self.logger);
                    std::thread::spawn(move || {
                        let mut socket = Socket::new(stream);
                        if let Err(e) = Self::handle(&mut socket, manager, logger.clone()) {
                            logger.log(format!("Error in Handle{}", e), Some(Level::URGENT)).unwrap();
                        }
                    });
                }
                Err(e) => self.logger.log(format!("Error in connection: {}", e), Some(Level::URGENT)).unwrap(),
            }
        }
        Ok(())
    }

    fn handle(
        socket: &mut Socket,
        manager: Arc<Manager>,
        logger: Arc<ConcurrentLogger>
    ) -> Result<(), Box<dyn Error>> {
        loop {
            // receive message
            let message = match socket.recv() {
                Ok(None)    => {
                    logger.log(String::from("CONNECTION CLOSED"), None)?;
                    return Ok(());
                },
                Ok(Some(message)) => {
                    logger.log(format!("SERVER RECV: {:?}", message), Some(Level::EVENT))?;
                    message
                },
                Err(e) => {
                    let reply = ProtoFactory::err( ProtoError::HandlingError(0, e.to_string()));
                    logger.log(format!("SERVER FAILED: {:?}", reply), Some(Level::URGENT))?;
                    socket.send(&reply).unwrap(); 
                    return Err(e);
                }
            };

            // create handler
            let handler: Box<dyn Handler> = match message.flag {
                f if f == ProtoMessageType::List as i32 => Box::new(ListHandler::new(&*manager)),
                f if f == ProtoMessageType::Encrypt as i32 => Box::new(EncryptHandler::new(&*manager)),
                f if f == ProtoMessageType::Sign as i32 => Box::new(SignHandler::new(&*manager)),
                f if f == ProtoMessageType::Decrypt as i32 => Box::new(DecryptHandler::new(&*manager)),
                _ =>  {
                    let reply = ProtoFactory::err( ProtoError::HandlingError(0, format!("Unexpected flag type: {}", message.flag)));
                    logger.log(format!("SERVER FAILED: {:?}", reply), Some(Level::EVENT))?;
                    socket.send(&reply).unwrap(); 
                    continue;
                }
            };

            // handle message
            match handler.process(&message) {
                Ok(reply)    => {
                    logger.log(format!("SERVER SENT: {:?}", reply), Some(Level::EVENT))?;
                    socket.send(&reply)?; 
                },
                Err(e) => {
                    let reply = ProtoFactory::err( ProtoError::MessageError(message, e.to_string()));
                    logger.log(format!("SERVER SENT: {:?}", reply), Some(Level::URGENT))?;
                    socket.send(&reply)?; 
                }
            }
        }
    }
}
