mod dkg;
mod orderbook;

use anoma::types::{Intent, Message};
use anoma::{config::Config, genesis::Bookkeeper};
use async_std::{io, task};
use futures::{future, prelude::*};
use libp2p::gossipsub::{
    Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic,
    MessageAuthenticity, MessageId, TopicHash, ValidationMode,
};
use libp2p::{
    gossipsub::{self},
    identity,
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId,
};

use serde::Deserialize;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::task::{Context, Poll};
use std::time::Duration;
use std::{error::Error, io::Write, path::PathBuf};

pub fn run(
    config: Config,
    peers: Option<Vec<String>>,
    topics: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let bookkeeper: Bookkeeper =
        read_or_generate_bookkeeper_key(&config.home_dir);
    let gossip_config =
        GossipConfig::read_or_generate(&config.home_dir, peers, topics);

    let mut bytes_key = bookkeeper.keypair.to_bytes();
    let key: identity::Keypair = libp2p::identity::Keypair::Ed25519(
        identity::ed25519::Keypair::decode(&mut bytes_key[..])?,
    );

    let local_peer_id = PeerId::from(key.public());

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(key)?;

    let network_behaviour = MyBehaviour::new(&gossip_config);

    // Create a Swarm to manage peers and events
    let mut swarm =
        libp2p::Swarm::new(transport, network_behaviour, local_peer_id);

    if let Some(peers_to_dial) = &gossip_config.peers {
        for to_dial in peers_to_dial {
            let dialing = to_dial.clone();
            match to_dial.parse() {
                Ok(to_dial) => {
                    match libp2p::Swarm::dial_addr(&mut swarm, to_dial) {
                        Ok(_) => println!("Dialed {:?}", dialing),
                        Err(e) => {
                            println!("Dial {:?} failed: {:?}", dialing, e)
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to parse address to dial: {:?}", err)
                }
            }
        }
    }

    libp2p::Swarm::listen_on(
        &mut swarm,
        gossip_config.local_address.parse().unwrap(),
    )
    .unwrap();

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
                    swarm
                        .gossipsub
                        .publish(Topic::new(orderbook::TOPIC), tix_bytes)
                }
                Poll::Ready(None) => panic!("Stdin closed"),
                Poll::Pending => break,
            } {
                println!("Publish error: {:?}", e);
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

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: Gossipsub,
}

impl MyBehaviour {
    pub fn new(gossip_config: &GossipConfig) -> Self {
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
        if let Some(topics) = &gossip_config.topics {
            for topic_str in topics {
                let topic = Topic::new(topic_str);
                gossipsub.subscribe(&topic).unwrap();
            }
        }

        Self { gossipsub }
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for MyBehaviour {
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

#[derive(Debug, Deserialize)]
struct GossipConfig {
    local_address: String,
    peers: Option<Vec<String>>,
    topics: Option<Vec<String>>,
}

const CONFIG_FILE: &str = "gossipsub.json";
impl GossipConfig {
    fn read_or_generate(
        home_dir: &PathBuf,
        peers: Option<Vec<String>>,
        topics: Option<Vec<String>>,
    ) -> Self {
        if home_dir.join("config").join(CONFIG_FILE).exists() {
            GossipConfig::read_config(home_dir, peers, topics)
        } else {
            let config = GossipConfig::generate_config(peers, topics);
            let _written = config.write_config(home_dir);
            config
        }
    }

    fn read_config(
        home_dir: &PathBuf,
        peers: Option<Vec<String>>,
        topics: Option<Vec<String>>,
    ) -> Self {
        let path = home_dir.join("config").join(CONFIG_FILE);
        let file = File::open(path).unwrap();
        let config: GossipConfig =
            serde_json::from_reader(file).expect("JSON was not well-formatted");
        Self {
            local_address: config.local_address,
            peers: peers.or(config.peers),
            topics: topics.or(config.topics),
        }
    }

    fn generate_config(
        peers: Option<Vec<String>>,
        topics: Option<Vec<String>>,
    ) -> Self {
        let local_address = "/ip4/0.0.0.0/tcp/0";
        let config = Self {
            local_address: local_address.to_string(),
            peers,
            topics,
        };
        config
    }

    fn write_config(&self, home_dir: &PathBuf) -> io::Result<()> {
        let path = home_dir.join("config").join(CONFIG_FILE);
        let mut file = File::create(path)?;
        let config = json!({
            "local_address": self.local_address,
            "peers" : self.peers,
            "topics": self.topics,
        });
        file.write(config.to_string().as_bytes()).map(|_| ())
    }
}
const BOOKKEEPER_KEY_FILE: &str = "priv_bookkepeer_key.json";

fn read_or_generate_bookkeeper_key(home_dir: &PathBuf) -> Bookkeeper {
    if home_dir.join("config").join(BOOKKEEPER_KEY_FILE).exists() {
        read_bookkeeper_key(home_dir)
    } else {
        generate_bookkeeper_key(home_dir)
    }
}

fn read_bookkeeper_key(home_dir: &PathBuf) -> Bookkeeper {
    let path = home_dir.join("config").join(CONFIG_FILE);
    let file = File::open(path).unwrap();
    serde_json::from_reader(file).expect("JSON was not well-formatted")
}

fn generate_bookkeeper_key(home_dir: &PathBuf) -> Bookkeeper {
    let account = Bookkeeper::new();
    let _written = write_bookkeeper_key(home_dir, &account);
    account
}
fn write_bookkeeper_key(
    home_dir: &PathBuf,
    account: &Bookkeeper,
) -> io::Result<()> {
    let path = home_dir.join("config").join("");
    let mut file = File::create(path)?;
    let key = json!({
        "pub_key": base64::encode(account.keypair.public.as_bytes()),
        "priv_key": base64::encode(account.keypair.to_bytes()),
    });
    file.write(key.to_string().as_bytes()).map(|_| ())
}
