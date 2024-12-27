use std::error::Error;

pub mod proto_message {
    include!(concat!(env!("OUT_DIR"), "/proto_message.rs"));
}

use prost::Message;

pub use proto_message::{ProtoMessage, ProtoMessageType};

pub fn message(
    id: i32,
    flag: ProtoMessageType,
    integer: Option<i64>,
    data: Vec<String>,
) -> ProtoMessage {
    ProtoMessage {
        id,
        flag: flag as i32,
        integer: integer.unwrap_or(0),
        repeat: data.len() as i32,
        data,
    }
}

pub fn deser(bytes: &[u8]) -> Result<ProtoMessage, Box<dyn Error>> {
    match ProtoMessage::decode(bytes) {
        Ok(m)  => Ok(m),
        Err(e) => Err(Box::from(format!("Failed Deserialization: {}", e.to_string())))

    }
}

pub fn ser(message: &ProtoMessage) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    match message.encode(&mut buf) {
        Ok(_)  => Ok(buf),
        Err(e) => Err(Box::from(format!("Failed Serialization: {}", e.to_string())))
    }
}
