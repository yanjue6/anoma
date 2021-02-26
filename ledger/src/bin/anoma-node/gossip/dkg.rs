pub mod protobuf {
    tonic::include_proto!("gossip");
}
use protobuf::{Intent, Response, Dkg};
use prost::Message;

pub const TOPIC: &str = "dkg";

pub fn apply(data: Vec<u8>) -> Result<Dkg, prost::DecodeError> {
    Dkg::decode(&data[..])
}
