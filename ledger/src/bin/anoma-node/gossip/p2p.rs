use super::config::NetworkConfig;
use super::network_behaviour::Behaviour;
use anoma::bookkeeper::Bookkeeper;
use libp2p::{identity::Keypair, identity::Keypair::Ed25519};
use std::error::Error;
use libp2p::PeerId;
use libp2p::gossipsub::IdentTopic as Topic;


pub type Swarm = libp2p::Swarm<Behaviour>;

pub fn build_swarm(bookkeeper: Bookkeeper) -> Result<Swarm, Box<dyn Error>> {
    // Create a random PeerId
    let local_key: Keypair = Ed25519(bookkeeper.key);
    let local_peer_id: PeerId = PeerId::from(local_key.public());

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key.clone())?;

    let gossipsub: Behaviour = Behaviour::new(local_key);

    // build the swarm
    Ok(Swarm::new(transport, gossipsub, local_peer_id))
}

pub fn prepare_swarm(swarm: &mut Swarm, network_config: &NetworkConfig) {
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
