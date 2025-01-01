use std::error::Error;

use crate::server::manager::Manager;
use crate::message::{ProtoMessage, ProtoMessageType};

use super::handler::Handler;

pub struct ListHandler<'a> {
    manager: &'a Manager,
}

impl<'a> ListHandler<'a> {
    pub fn new(manager: &'a Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for ListHandler<'a> {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            integer: 0,
            err: false,
            data: vec!{},
        };

        for entry in self.manager.list()? {
            reply.data.push(entry);
        }

        Ok(reply)
    }
}
