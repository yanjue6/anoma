use super::dkg;
use super::orderbook;
use super::orderbook::Orderbook;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use libp2p::gossipsub::{
    self, Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic,
    MessageAuthenticity, MessageId, TopicHash, ValidationMode,
};

use libp2p::{
    identity::Keypair, swarm::NetworkBehaviourEventProcess, NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub gossipsub: Gossipsub,
    #[behaviour(ignore)]
    // XXX TODO instead add a channel and spawn message on it
    orderbook: Orderbook,
}

impl Behaviour {
    pub fn new(key: Keypair) -> Self {
        // To content-address message, we can take the hash of message and use it as an ID.
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        };

        // Set a custom gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .build()
            .expect("Valid config");

        let gossipsub: Gossipsub =
            Gossipsub::new(MessageAuthenticity::Signed(key), gossipsub_config)
                .expect("Correct configuration");
        Self {
            gossipsub,
            orderbook: Orderbook::new(),
        }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    // Called when `gossipsub` produces an event.
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message {
            propagation_source,
            message_id,
            message:
                GossipsubMessage {
                    data,
                    topic: topic_hash,
                    ..
                },
        } = event
        {
            println!(
                "Got message of id: {} from peer: {:?}",
                message_id, propagation_source,
            );
            if TopicHash::from(Topic::new(orderbook::TOPIC)) == topic_hash {
                self.orderbook.apply(data).unwrap();
            } else if TopicHash::from(Topic::new(dkg::TOPIC)) == topic_hash {
                let tx = dkg::apply(data);
                println!("Got DKG message: {:?}", tx);
            } else if TopicHash::from(Topic::new("test-net")) == topic_hash {
                println!("Got other message: {:?}", data);
            } else {
            };
        }
    }
}
