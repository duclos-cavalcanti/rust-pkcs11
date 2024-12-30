use std::error::Error;

use super::handler::Handler;
use crate::server::manager::Manager;
use crate::server::message::{Message, ProtoMessage, ProtoMessageType};

pub struct LoginHandler<'a> {
    manager: &'a mut Manager,
}

impl<'a> LoginHandler<'a> {
    pub fn new(manager: &'a mut Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for LoginHandler<'a> {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            integer: 0,
            repeat: 0 as i32,
            err: false,
            data: vec!{},
        };

        Ok(reply)
    }
}

