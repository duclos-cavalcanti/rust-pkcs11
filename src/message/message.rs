use std::error::Error;
use prost::Message;

mod proto_message {
    include!(concat!(env!("OUT_DIR"), "/proto_message.rs"));
}

pub use proto_message::{ProtoMessage, ProtoMessageType};

pub struct ProtoFactory;

pub enum ProtoError {
    MessageError(ProtoMessage, String), 
    HandlingError(i32, String)
}

impl ProtoFactory {
    pub fn list(id: i32) -> ProtoMessage {
        ProtoMessage {
            id,
            flag: ProtoMessageType::List as i32,
            slot_id: 0,
            pin: String::new(),
            err: false,
            data: vec![],
        }
    }

    pub fn enc(id: i32, slot_id: u64, pin: String, data:String) -> ProtoMessage {
        ProtoMessage {
            id,
            flag: ProtoMessageType::Encrypt as i32,
            slot_id: slot_id,
            pin: pin.clone(),
            err: false,
            data: vec!{data.clone()},
        }
    }

    pub fn sign(id: i32, slot_id: u64, pin: String, data:String) -> ProtoMessage {
        ProtoMessage {
            id,
            flag: ProtoMessageType::Sign as i32,
            slot_id: slot_id,
            pin: pin.clone(),
            err: false,
            data: vec!{data.clone()},
        }
    }

    pub fn err(p_err: ProtoError) -> ProtoMessage {
        let (id, e) = match p_err {
           ProtoError::MessageError(m, e)   => (m.id, e), 
           ProtoError::HandlingError(id, e) => (id, e)
        };

        ProtoMessage {
            id: id,
            flag: ProtoMessageType::Ack as i32,
            slot_id: 0,
            pin: String::new(),
            err: true,
            data: vec!{e},
        }
    }

    pub fn encode(message: &ProtoMessage) -> Result<(Vec<u8>, usize), Box<dyn Error>> {
        let mut buf = Vec::new();
        let n = match message.encode(&mut buf) {
            Ok(_)  => buf.len(),
            Err(e) => return Err(Box::from(format!("Failed Serialization: {}", e.to_string())))
        };
        Ok((buf, n))
    }
    
    pub fn decode(buf: &mut Vec<u8>, n:usize) -> Result<ProtoMessage, Box<dyn Error>> {
        let m =  match ProtoMessage::decode(&buf[..n]) {
            Ok(m) => m,
            Err(e) => return Err(Box::from(format!("Failed Deserialization: {}", e.to_string())))
        };
        Ok(m)
    }
}
