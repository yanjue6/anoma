use anoma::types::{Intent, Message};
use async_std::{io, task};
use futures::prelude::*;
use libp2p::gossipsub::MessageId;
use libp2p::gossipsub::{
    Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic,
    MessageAuthenticity, ValidationMode,
};
use libp2p::{gossipsub, identity, PeerId};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::{
    error::Error,
    task::{Context, Poll},
};

pub fn run(peer_addr: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut gossip = Gossip::new()?;
    libp2p::Swarm::listen_on(
        &mut gossip.swarm,
        "/ip4/0.0.0.0/tcp/0".parse().unwrap(),
    )
    .unwrap();

    if let Some(to_dial) = peer_addr {
        let dialing = to_dial.clone();
        match to_dial.parse() {
            Ok(to_dial) => {
                match libp2p::Swarm::dial_addr(&mut gossip.swarm, to_dial) {
                    Ok(_) => println!("Dialed {:?}", dialing),
                    Err(e) => println!("Dial {:?} failed: {:?}", dialing, e),
                }
            }
            Err(err) => println!("Failed to parse address to dial: {:?}", err),
        }
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Kick it off
    let mut listening = false;
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            if let Err(e) = match stdin.try_poll_next_unpin(cx)? {
                Poll::Ready(Some(line)) => {
                    let tix = Intent { msg: line };
                    let mut tix_bytes = vec![];
                    tix.encode(&mut tix_bytes).unwrap();
                    gossip.swarm.publish(gossip.topic.clone(), tix_bytes)
                }
                Poll::Ready(None) => panic!("Stdin closed"),
                Poll::Pending => break,
            } {
                println!("Publish error: {:?}", e);
            }
        }

        loop {
            match gossip.swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(gossip_event)) => match gossip_event {
                    GossipsubEvent::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    } => {
                        let tx =
                            Intent::decode(&message.data[..]).map_err(|e| {
                                format!(
                            "Error decoding a intent: {}, from bytes {:?}",
                            e, message
                        )
                            })?;
                        println!(
                            "Got message: {:?} with id: {} from peer: {:?}",
                            tx, id, peer_id
                        );
                        gossip.mempool.push(tx)
                    }
                    _ => {}
                },
                Poll::Ready(None) | Poll::Pending => break,
            }
        }

        if !listening {
            for addr in libp2p::Swarm::listeners(&gossip.swarm) {
                println!("Listening on {:?}", addr);
                listening = true;
            }
        }

        Poll::Pending
    }))
}

pub struct Gossip {
    local_key: identity::Keypair,
    local_peer_id: PeerId,
    topic: Topic,
    swarm: libp2p::Swarm<Gossipsub>,
    mempool: Vec<Intent>,
}

impl Gossip {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);

        // Create a Gossipsub topic
        let topic = Topic::new("test-net");

        // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
        let transport = libp2p::build_development_transport(local_key.clone())?;

        // Create a Swarm to manage peers and events
        let swarm = {
            // To content-address message, we can take the hash of message and use it as an ID.
            let message_id_fn = |message: &GossipsubMessage| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                MessageId::from(s.finish().to_string())
            };

            // Set a custom gossipsub
            let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(ValidationMode::None)
                .message_id_fn(message_id_fn)
                .build()
                .expect("Valid config");
            // build a gossipsub network behaviour
            let mut gossipsub: gossipsub::Gossipsub =
                gossipsub::Gossipsub::new(
                    MessageAuthenticity::Anonymous,
                    gossipsub_config,
                )
                .expect("Correct configuration");

            // subscribes to our topic
            gossipsub.subscribe(&topic).unwrap();

            // build the swarm
            libp2p::Swarm::new(transport, gossipsub, local_peer_id)
        };

        Ok(Self {
            local_key,
            local_peer_id,
            topic,
            swarm,
            mempool: Vec::new(),
        })
    }
}
