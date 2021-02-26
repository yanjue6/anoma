pub mod rpc {
    tonic::include_proto!("gossip");
}
use rpc::{Intent, Response, Dkg};
use prost::Message;

pub const TOPIC: &str = "orderbook";

pub fn apply(data: Vec<u8>) -> Result<Intent, prost::DecodeError> {
    Intent::decode(&data[..])
}
