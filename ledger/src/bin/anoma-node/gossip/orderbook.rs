use super::mempool::{IntentId, Mempool};
use super::network_behaviour::BehaviourEvent;
use anoma::protobuf::gossip::Intent;
use libp2p::gossipsub::{IdentTopic as Topic, TopicHash};
use prost::Message;

pub const TOPIC: &str = "orderbook";

pub struct Orderbook {
    pub mempool: Mempool,
    topic_hash: TopicHash,
}
impl Orderbook {
    pub fn new() -> Self {
        Self {
            mempool: Mempool::new(),
            topic_hash: Topic::new(String::from(TOPIC)).hash(),
        }
    }

    pub fn apply(
        &mut self,
        BehaviourEvent::Message(_peer_id, topic_hash,_message_id, data): &BehaviourEvent,
    ) -> Result<bool, prost::DecodeError> {
        if topic_hash == &self.topic_hash {
            let intent = Intent::decode(&data[..])?;
            println!("Adding intent {:?} to mempool", intent);
            self.mempool.put(&IntentId::new(&intent), intent);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
