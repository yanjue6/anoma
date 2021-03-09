use libp2p::gossipsub::{
    self, Gossipsub, GossipsubEvent, GossipsubMessage, MessageAuthenticity,
    MessageId, TopicHash, ValidationMode,
};
use libp2p::PeerId;
use libp2p::{
    identity::Keypair, swarm::NetworkBehaviourEventProcess, NetworkBehaviour,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};

/// A behaviour event#
#[derive(Debug)]
pub enum BehaviourEvent {
    /// A new message received from a peer
    Message(Option<PeerId>, TopicHash, MessageId, Vec<u8>),
}
impl From<GossipsubMessage> for BehaviourEvent {
    fn from(msg: GossipsubMessage) -> Self {
        Self::Message(
            msg.source,
            msg.topic.clone(),
            message_id(&msg),
            msg.data.clone(),
        )
    }
}
#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub gossipsub: Gossipsub,
    #[behaviour(ignore)]
    event_chan: Sender<BehaviourEvent>,
}
fn message_id(message: &GossipsubMessage) -> MessageId {
    let mut s = DefaultHasher::new();
    message.data.hash(&mut s);
    MessageId::from(s.finish().to_string())
}

impl Behaviour {
    pub fn new(key: Keypair) -> (Self, Receiver<BehaviourEvent>) {
        // To content-address message, we can take the hash of message and use it as an ID.

        // Set a custom gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(message_id)
            .validate_messages()
            .build()
            .expect("Valid config");

        let gossipsub: Gossipsub =
            Gossipsub::new(MessageAuthenticity::Signed(key), gossipsub_config)
                .expect("Correct configuration");

        let (event_chan, rx) = channel::<BehaviourEvent>(100);
        (
            Self {
                gossipsub,
                event_chan,
            },
            rx,
        )
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for Behaviour {
    // Called when `gossipsub` produces an event.
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message {
            propagation_source,
            message_id,
            message,
        } = event
        {
            println!(
                "Got message of id: {} from peer: {:?}",
                message_id, propagation_source,
            );
            let _res = self.event_chan.try_send(BehaviourEvent::from(message));
        }
    }
}
