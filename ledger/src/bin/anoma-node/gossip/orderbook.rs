use super::mempool::{IntentId, Mempool};
use super::network_behaviour::BehaviourEvent;
use anoma::protobuf::gossip::Intent;
use libp2p::gossipsub::{IdentTopic as Topic, TopicHash};
use prost::Message;

pub const TOPIC: &str = "orderbook";

#[derive(Debug, Clone)]
pub enum Error{
    DecodeError(prost::DecodeError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
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
    ) -> Result<bool> {
        if topic_hash == &self.topic_hash {
            let intent = Intent::decode(&data[..]).map_err(Error::DecodeError)?;
            println!("Adding intent {:?} to mempool", intent);
            self.mempool.put(&IntentId::new(&intent), intent);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
