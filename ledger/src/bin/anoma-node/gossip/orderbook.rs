use anoma::types::{Intent, Message};
use prost;

pub const TOPIC: &str = "orderbook";

pub fn apply(data: Vec<u8>) -> Result<Intent, prost::DecodeError> {
    Intent::decode(&data[..])
}
