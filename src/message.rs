use std::error::Error;

pub mod proto_message {
    include!(concat!(env!("OUT_DIR"), "/proto_message.rs"));
}

use prost::Message;

pub fn make(id: i32, data: &str) -> proto_message::ProtoMessage {
    proto_message::ProtoMessage {
        id,
        data: data.to_string(),
    }
}

pub fn deser(bytes: &[u8]) -> Result<proto_message::ProtoMessage, Box<dyn Error>> {
    match proto_message::ProtoMessage::decode(bytes) {
        Ok(m)  => Ok(m),
        Err(e) => Err(Box::from(format!("Failed Deserialization: {}", e.to_string())))

    }
}

pub fn ser(message: &proto_message::ProtoMessage) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    message.encode(&mut buf)?;
    Ok(buf)
}
