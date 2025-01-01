use crate::message::{ProtoMessage};
use std::error::Error;

pub trait Handler {
    fn process(&self, message: &ProtoMessage) -> Result<ProtoMessage, Box<dyn Error>>;
}
