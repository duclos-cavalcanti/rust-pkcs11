use std::error::Error;
use crate::server::message::ProtoMessage;

pub trait Handler {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>>;
}
