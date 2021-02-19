mod dkg;
mod orderbook;

use anoma::types::{Intent, Message};
use anoma::{config::Config, genesis::Validator};
use async_std::{io, task};
use futures::prelude::*;
use libp2p::gossipsub::MessageId;
use libp2p::gossipsub::{
    GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
    TopicHash, ValidationMode,
};
use libp2p::{gossipsub, identity, PeerId};
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::{
    error::Error,
    task::{Context, Poll},
};
use std::{fs::File, io::Write, path::PathBuf};

pub fn run(
    config: Config,
    peer_addr: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let home_dir = config.orderbook_home_dir();

    let gossip = GossipNode::new()?;

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport =
        libp2p::build_development_transport(gossip.local_key.clone())?;

    // Set a custom gossipsub
    let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .validation_mode(ValidationMode::None)
        // To content-address message, we can take the hash of message and use it as an ID.
        .message_id_fn(|message: &GossipsubMessage| -> MessageId {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            MessageId::from(s.finish().to_string())
        })
        .build()
        .expect("Valid config");
    // build a gossipsub network behaviour
    let mut gossipsub: gossipsub::Gossipsub = gossipsub::Gossipsub::new(
        MessageAuthenticity::Anonymous,
        gossipsub_config,
    )
    .expect("Correct configuration");

    // subscribes to our topic
    gossipsub.subscribe(&gossip.topic).unwrap();

    // Create a Swarm to manage peers and events
    let mut swarm =
        libp2p::Swarm::new(transport, gossipsub, gossip.local_peer_id);

    libp2p::Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    if let Some(to_dial) = peer_addr {
        let dialing = to_dial.clone();
        match to_dial.parse() {
            Ok(to_dial) => {
                match libp2p::Swarm::dial_addr(&mut swarm, to_dial) {
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
                    swarm.publish(gossip.topic.clone(), tix_bytes)
                }
                Poll::Ready(None) => panic!("Stdin closed"),
                Poll::Pending => break,
            } {
                println!("Publish error: {:?}", e);
            }
        }

        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(None) | Poll::Pending => break,
                Poll::Ready(Some(GossipsubEvent::Message {
                    propagation_source,
                    message_id,
                    message:
                        GossipsubMessage {
                            data,
                            topic: topic_hash,
                            ..
                        },
                })) => {
                    println!(
                        "Got message of id: {} from peer: {:?}",
                        message_id, propagation_source,
                    );
                    if TopicHash::from(Topic::new(orderbook::TOPIC))
                        == topic_hash
                    {
                        let tx = orderbook::apply(data);
                        println!("message: {:?}", tx);
                    } else if TopicHash::from(Topic::new(dkg::TOPIC))
                        == topic_hash
                    {
                        let tx = dkg::apply(data);
                        println!("Got message: {:?}", tx);
                    } else {
                    };
                }
                _ => {}
            }
        }

        if !listening {
            for addr in libp2p::Swarm::listeners(&swarm) {
                println!("Listening on {:?}", addr);
                listening = true;
            }
        }

        Poll::Pending
    }))
}

pub struct GossipNode {
    local_key: identity::Keypair,
    local_peer_id: PeerId,
    topic: Topic,
}

impl GossipNode {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);

        // Create a Gossipsub topic
        let topic = Topic::new(orderbook::TOPIC);

        Ok(Self {
            local_key,
            local_peer_id,
            topic,
        })
    }
}

fn write_peer_key(home_dir: PathBuf, account: &Validator) -> io::Result<()> {
    let path = home_dir.join("config").join("priv_validator_key.json");
    let mut file = File::create(path)?;
    let pk = base64::encode(account.keypair.public.as_bytes());
    let sk = base64::encode(account.keypair.to_bytes());
    let key = json!({
        "address": account.address,
        "pub_key": {
            "type": "tendermint/PubKeyEd25519",
            "value": pk,
        },
        "priv_key": {
            "type": "tendermint/PrivKeyEd25519",
            "value": sk,
        }
    });
    println!("key {}", key);
    file.write(key.to_string().as_bytes()).map(|_| ())
}
