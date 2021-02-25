pub mod rpc {
    tonic::include_proto!("gossip");
}
use rpc::{Intent, Response, Dkg};
use prost;

pub const TOPIC: &str = "dkg";

pub fn apply(data: Vec<u8>) -> Result<Dkg, prost::DecodeError> {
    Dkg::decode(&data[..])
}
