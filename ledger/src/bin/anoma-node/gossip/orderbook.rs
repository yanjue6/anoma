pub mod protobuf {
    tonic::include_proto!("gossip");
}
use protobuf::{Intent};
use prost::Message;

pub const TOPIC: &str = "orderbook";

pub fn apply(data: Vec<u8>) -> Result<Intent, prost::DecodeError> {
    Intent::decode(&data[..])
}
