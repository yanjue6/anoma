use super::mempool::{IntentId, Mempool};
use super::network_behaviour::{Behaviour, BehaviourEvent};
use anoma::protobuf::gossip::Dkg;
use futures::channel::mpsc::{channel, Receiver, Sender};
use prost::Message;

pub const TOPIC: &str = "dkg";

pub struct DKG {}

impl DKG {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn apply(
        &mut self,
        data: Vec<u8>,
    ) -> Result<bool, prost::DecodeError> {
        let dkg_msg = Dkg::decode(&data[..])?;
        Ok(true)
    }
}
