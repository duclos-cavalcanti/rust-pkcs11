use crate::message::{ProtoMessage, ProtoMessageType};
use crate::server::manager::Manager;

use std::error::Error;

pub trait Handler {
    fn process(&mut self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>>;
}

pub struct ListHandler<'a> {
    manager: &'a mut Manager,
}

impl<'a> ListHandler<'a> {
    pub fn new(manager: &'a mut Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for ListHandler<'a> {
    fn process(&mut self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            slot_id: 0,
            pin: String::new(),
            err: false,
            data: vec!{},
        };

        for entry in self.manager.list()? {
            reply.data.push(entry);
        }

        Ok(reply)
    }
}

pub struct EncryptHandler<'a> {
    manager: &'a mut Manager,
}

impl<'a> EncryptHandler<'a> {
    pub fn new(manager: &'a mut Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for EncryptHandler<'a> {
    fn process(&mut self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            slot_id: 0,
            pin: String::new(),
            err: false,
            data: vec!{},
        };

        let id:u64      = message.slot_id;
        let pin:String  = message.pin.clone();
        let data:String = message.data[0].clone();
        let ciphertext = self.manager.encrypt(id, &pin, data.as_bytes())?;

        reply.data.push(base64::encode(ciphertext));
        Ok(reply)
    }
}

pub struct SignHandler<'a> {
    manager: &'a mut Manager,
}

impl<'a> SignHandler<'a> {
    pub fn new(manager: &'a mut Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for SignHandler<'a> {
    fn process(&mut self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id: message.id,
            flag: ProtoMessageType::Ack as i32,
            slot_id: 0,
            pin: String::new(),
            err: false,
            data: vec!{},
        };

        let id:u64      = message.slot_id;
        let pin:String  = message.pin.clone();
        let data:String = message.data[0].clone();
        let ciphertext = self.manager.sign(id, &pin, data.as_bytes())?;

        reply.data.push(base64::encode(ciphertext));
        Ok(reply)
    }
}
