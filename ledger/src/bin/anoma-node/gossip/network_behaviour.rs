use super::dkg;
use super::orderbook;
use std::time::Duration;

use libp2p::gossipsub::{
    Gossipsub, GossipsubConfig, GossipsubEvent, GossipsubMessage,
    IdentTopic as Topic, MessageAuthenticity, MessageId, TopicHash,
    ValidationMode,
};

use libp2p::{
    gossipsub::{self},
    identity::Keypair,
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use sha2::{Digest, Sha256};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub gossipsub: Gossipsub,
}

impl Behaviour {
    pub fn new(key: Keypair, topics: Vec<String>) -> Self {
        let gossip_config = Behaviour::gossipsub_config();

        let mut gossipsub: gossipsub::Gossipsub = gossipsub::Gossipsub::new(
            MessageAuthenticity::Signed(key),
            gossip_config,
        )
        .expect("Correct configuration");

        for topic_str in topics {
            let topic = Topic::new(topic_str);
            gossipsub.subscribe(&topic).unwrap();
        }
        Self { gossipsub }
    }

    fn gossipsub_config() -> GossipsubConfig {
        gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(Behaviour::gossipsub_message_id)
            .build()
            .expect("Valid config")
    }

    fn gossipsub_message_id(message: &GossipsubMessage) -> MessageId {
        let mut hasher = Sha256::new();
        hasher.update(message.data.as_slice());
        let address = format!("{:.40X}", hasher.finalize());
        MessageId::from(address.into_bytes())
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    // Called when `gossipsub` produces an event.
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source,
                message_id,
                message:
                    GossipsubMessage {
                        data,
                        topic: topic_hash,
                        ..
                    },
            } => {
                println!(
                    "Got message of id: {} from peer: {:?}",
                    message_id, propagation_source,
                );
                if TopicHash::from(Topic::new(orderbook::TOPIC)) == topic_hash {
                    let tx = orderbook::apply(data);
                    println!("message: {:?}", tx);
                } else if TopicHash::from(Topic::new(dkg::TOPIC)) == topic_hash
                {
                    let tx = dkg::apply(data);
                    println!("Got message: {:?}", tx);
                } else {
                };
            }
            _ => {}
        }
    }
}
