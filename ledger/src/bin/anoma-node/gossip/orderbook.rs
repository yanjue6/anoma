use super::mempool::{IntentId, Mempool};
use super::network_behaviour::{Behaviour, BehaviourEvent};
use anoma::protobuf::gossip::Intent;
use futures::channel::mpsc::{channel, Receiver, Sender};
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

    pub fn apply(
        &mut self,
        BehaviourEvent::Message(peer_id, topic_hash,message_id, data): &BehaviourEvent,
    ) -> Result<bool, prost::DecodeError> {
        let intent = Intent::decode(&data[..])?;
        println!("ITENT : {:?}",intent);
        self.mempool.put(&IntentId::new(&intent), intent);
        Ok(true)
    }
}
