mod config;
mod dkg;
mod network_behaviour;
mod orderbook;

use self::config::NetworkConfig;
use self::network_behaviour::Behaviour;
use anoma::{bookkeeper::Bookkeeper, config::*, protobuf::gossip::Intent};
use async_std::{io, task};
use futures::prelude::*;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::PeerId;
use libp2p::{identity::Keypair, identity::Keypair::Ed25519, Swarm};
use std::fs;
use std::fs::File;
use std::{
    error::Error,
    task::{Context, Poll},
};
use std::{io::Write, path::PathBuf};
use prost::Message;

#[warn(unused_variables)]
pub fn run(
    config: Config,
    rpc: bool,
    local_address: Option<String>,
    peers: Option<Vec<String>>,
    topics: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let base_dir: PathBuf = config.gossip_home_dir();
    let bookkeeper: Bookkeeper = read_or_generate_bookkeeper_key(&base_dir)?;

    // Create a Gossipsub topic
    let topic = Topic::new(String::from(orderbook::TOPIC));

    let network_config = NetworkConfig::read_or_generate(
        &base_dir,
        local_address,
        peers,
        topics,
    );

    let mut swarm = build_swarm(bookkeeper)?;
    prepare_swarm(&mut swarm, &network_config);

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Kick it off
    let mut listening = false;
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            if let Err(e) = match stdin.try_poll_next_unpin(cx)? {
                Poll::Ready(Some(line)) => {
                    let tix = Intent {
                        asset: line.clone(),
                    };
                    let mut tix_bytes = vec![];
                    tix.encode(&mut tix_bytes).unwrap();
                    swarm.gossipsub.publish(topic.clone(), tix_bytes)
                }
                Poll::Ready(None) => {
                    println!("panicking stding");
                    panic!("Stdin closed")
                }
                Poll::Pending => break,
            } {
                println!("Publish error: {:?}", e);
            }
        }

        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => println!("EVENT {:?}", event),
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => break,
            }
        }

        if !listening {
            for addr in Swarm::listeners(&swarm) {
                println!("Listening on {:?}", addr);
                listening = true;
            }
        }

        Poll::Pending
    }))
}
fn build_swarm(
    bookkeeper: Bookkeeper,
) -> Result<Swarm<Behaviour>, Box<dyn Error>> {
    // Create a random PeerId
    let local_key: Keypair = Ed25519(bookkeeper.key);
    let local_peer_id: PeerId = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key.clone())?;

    let gossipsub: Behaviour = Behaviour::new(local_key);

    // build the swarm
    Ok(Swarm::new(transport, gossipsub, local_peer_id))
}

fn prepare_swarm(swarm: &mut Swarm<Behaviour>, network_config: &NetworkConfig) {
    for topic_string in &network_config.gossip.topics {
        let topic = Topic::new(topic_string);
        swarm.gossipsub.subscribe(&topic).unwrap();
    }

    // Listen on all interfaces and whatever port the OS assigns
    Swarm::listen_on(swarm, network_config.local_address.parse().unwrap())
        .unwrap();

    // Reach out to another node if specified
    for to_dial in &network_config.peers {
        let dialing = to_dial.clone();
        match to_dial.parse() {
            Ok(to_dial) => match Swarm::dial_addr(swarm, to_dial) {
                Ok(_) => println!("Dialed {:?}", dialing),
                Err(e) => {
                    println!("Dial {:?} failed: {:?}", dialing, e)
                }
            },
            Err(err) => {
                println!("Failed to parse address to dial: {:?}", err)
            }
        }
    }
}
const BOOKKEEPER_KEY_FILE: &str = "priv_bookkepeer_key.json";

fn read_or_generate_bookkeeper_key(
    home_dir: &PathBuf,
) -> Result<Bookkeeper, std::io::Error> {
    if home_dir.join("config").join(BOOKKEEPER_KEY_FILE).exists() {
        println!(
            "Reading key {:?}",
            home_dir.join("config").join(BOOKKEEPER_KEY_FILE)
        );
        let home_dir = home_dir;
        let conf_file = home_dir.join("config").join(BOOKKEEPER_KEY_FILE);
        let json_string = fs::read_to_string(conf_file.as_path())?;
        let bookkeeper = serde_json::from_str::<Bookkeeper>(&json_string)?;
        Ok(bookkeeper)
    } else {
        println!(
            "Generating key {:?}",
            home_dir.join("config").join(BOOKKEEPER_KEY_FILE)
        );
        let account: Bookkeeper = Bookkeeper::new();
        let home_dir = home_dir;
        let path = home_dir.join("config").join(BOOKKEEPER_KEY_FILE);
        let mut file = File::create(path)?;
        let json = serde_json::to_string(&account)?;
        file.write_all(json.as_bytes()).map(|_| ()).unwrap();
        Ok(account)
    }
}
