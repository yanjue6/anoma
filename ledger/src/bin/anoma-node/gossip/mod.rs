mod config;
mod dkg;
pub mod network_behaviour;
mod orderbook;

use crate::rpc;
use anoma::{bookkeeper::Bookkeeper, config::*};
use async_std::{io, task};
use config::NetworkConfig;
use futures::{future, prelude::*};
use libp2p::identity::Keypair::Ed25519;
use libp2p::{
    gossipsub::{self},
    swarm::{NetworkBehaviourEventProcess, Swarm},
    NetworkBehaviour, PeerId,
};
use serde::Deserialize;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::error;
use std::fs;
use std::fs::File;
use std::task::{Context, Poll};
use std::{error::Error, io::Write, path::PathBuf};

pub fn run(
    config: Config,
    local_address: Option<String>,
    peers: Option<Vec<String>>,
    topics: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let base_dir: PathBuf = config.gossip_home_dir();
    let bookkeeper: Bookkeeper = read_or_generate_bookkeeper_key(&base_dir)?;

    println!("Bookkeper key {:?}", bookkeeper);

    let network_config = NetworkConfig::read_or_generate(
        &base_dir,
        local_address,
        peers,
        topics,
    );

    let mut swarm = prepare_swarm(bookkeeper, network_config)?;

    let _res = rpc::rpc_server();

    let mut listening = false;
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => println!("{:?}", event),
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => {
                    if !listening {
                        for addr in Swarm::listeners(&swarm) {
                            println!("Listening in {}", addr);
                            listening = true;
                        }
                    }
                    break;
                }
            }
        }
        Poll::Pending
    }))
}

fn prepare_swarm(
    bookkeeper: Bookkeeper,
    network_config: NetworkConfig,
) -> std::io::Result<Swarm<Behaviour>> {
    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let keypair = Ed25519(bookkeeper.key);
    let local_peer_id = PeerId::from(keypair.public());

    let transport = libp2p::build_development_transport(keypair.clone())?;

    let network_behaviour =
        Behaviour::new(keypair, network_config.gossip.topics);

    // Create a Swarm to manage peers and events
    let mut swarm =
        libp2p::Swarm::new(transport, network_behaviour, local_peer_id);

    for to_dial in &network_config.peers {
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

    libp2p::Swarm::listen_on(
        &mut swarm,
        network_config.local_address.parse().unwrap(),
    )
    .unwrap();
    Ok(swarm)
}

const BOOKKEEPER_KEY_FILE: &str = "priv_bookkepeer_key.json";

fn read_or_generate_bookkeeper_key(
    home_dir: &PathBuf,
) -> Result<Bookkeeper, Box<dyn error::Error>> {
    if home_dir.join("config").join(BOOKKEEPER_KEY_FILE).exists() {
        println!(
            "Reading key {:?}",
            home_dir.join("config").join(BOOKKEEPER_KEY_FILE)
        );
        Ok(read_bookkeeper_key(home_dir)?)
    } else {
        println!(
            "Generating key {:?}",
            home_dir.join("config").join(BOOKKEEPER_KEY_FILE)
        );
        let account = Bookkeeper::new();
        let _write = write_bookkeeper_key(home_dir, &account);
        Ok(account)
    }
}

fn read_bookkeeper_key(
    home_dir: &PathBuf,
) -> Result<Bookkeeper, Box<dyn error::Error>> {
    let conf_file = home_dir.join("config").join(BOOKKEEPER_KEY_FILE);
    let json_string = fs::read_to_string(conf_file.as_path())?;
    let bookkeeper = serde_json::from_str::<Bookkeeper>(&json_string)?;
    Ok(bookkeeper)
}

fn write_bookkeeper_key(
    home_dir: &PathBuf,
    account: &Bookkeeper,
) -> io::Result<()> {
    let path = home_dir.join("config").join(BOOKKEEPER_KEY_FILE);
    let mut file = File::create(path)?;
    let json = serde_json::to_string(&account)?;
    file.write_all(json.as_bytes()).map(|_| ())
}
