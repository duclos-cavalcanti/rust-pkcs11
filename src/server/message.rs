pub mod proto_message {
    include!(concat!(env!("OUT_DIR"), "/proto_message.rs"));
}

pub use proto_message::{ProtoMessage, ProtoMessageType};
pub use prost::Message;
