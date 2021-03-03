use super::mempool::{IntentId, Mempool};
use anoma::protobuf::gossip::Intent;
use prost::Message;

pub const TOPIC: &str = "orderbook";

pub struct Orderbook {
    pub mempool: Mempool,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            mempool: Mempool::new(),
        }
    }

    pub fn apply(&mut self, data: Vec<u8>) -> Result<(), prost::DecodeError> {
        let intent = Intent::decode(&data[..])?;
        self.mempool.put(&IntentId::new(&intent), intent);
        Ok(())
    }
}
