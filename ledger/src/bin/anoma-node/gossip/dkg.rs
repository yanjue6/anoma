use anoma::types::{Dkg, Message};
use prost;

pub const TOPIC: &str = "dkg";

pub fn apply(data: Vec<u8>) -> Result<Dkg, prost::DecodeError> {
    Dkg::decode(&data[..])
}
