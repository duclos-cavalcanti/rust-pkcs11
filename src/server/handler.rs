use crate::message::{ProtoMessage, ProtoMessageType};
use crate::server::manager::Manager;

use base64::prelude::*;

use std::error::Error;

pub trait Handler {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>>;
}

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
            id: message.id.clone(),
            seq: message.seq,
            flag: ProtoMessageType::Ack as i32,
            slot: 0,
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
    manager: &'a Manager,
}

impl<'a> EncryptHandler<'a> {
    pub fn new(manager: &'a Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for EncryptHandler<'a> {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id:  message.id.clone(),
            seq: message.seq.clone(),
            flag: ProtoMessageType::Ack as i32,
            slot: 0,
            pin: String::new(),
            err: false,
            data: vec!{},
        };

        let slot:u64    = message.slot;
        let pin:String  = message.pin.clone();
        let data:String = message.data[0].clone();
        let ciphertext = self.manager.encrypt(slot, &pin, data.as_bytes())?;
        reply.data.push(BASE64_STANDARD.encode(ciphertext));
        Ok(reply)
    }
}

pub struct SignHandler<'a> {
    manager: &'a Manager,
}

impl<'a> SignHandler<'a> {
    pub fn new(manager: &'a Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for SignHandler<'a> {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id:  message.id.clone(),
            seq: message.seq.clone(),
            flag: ProtoMessageType::Ack as i32,
            slot: 0,
            pin: String::new(),
            err: false,
            data: vec!{},
        };

        let slot:u64      = message.slot;
        let pin:String  = message.pin.clone();
        let data:String = message.data[0].clone();
        let ciphertext = self.manager.sign(slot, &pin, data.as_bytes())?;

        reply.data.push(BASE64_STANDARD.encode(ciphertext));
        Ok(reply)
    }
}

pub struct DecryptHandler<'a> {
    manager: &'a Manager,
}

impl<'a> DecryptHandler<'a> {
    pub fn new(manager: &'a Manager) -> Self {
        Self { manager }
    }
}

impl<'a> Handler for DecryptHandler<'a> {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>> {
        let mut reply = ProtoMessage {
            id:  message.id.clone(),
            seq: message.seq.clone(),
            flag: ProtoMessageType::Ack as i32,
            slot: 0,
            pin: String::new(),
            err: false,
            data: vec!{},
        };

        let slot:u64    = message.slot;
        let pin:String  = message.pin.clone();
        let data        = BASE64_STANDARD.decode(message.data[0].clone())?;
        let data        = self.manager.decrypt(slot, &pin, &data)?;

        match String::from_utf8(data) {
            Ok(t)  => reply.data.push(t),
            Err(e) => return Err(Box::from(format!("Conversion Bytes to String failed: {}", e))),
        }

        Ok(reply)
    }
}
