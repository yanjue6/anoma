use super::{config::NetworkConfig, orderbook::Orderbook};
use super::network_behaviour::{Behaviour, BehaviourEvent};
use anoma::bookkeeper::Bookkeeper;
use anoma::protobuf::gossip::Intent;
use libp2p::gossipsub::{IdentTopic as Topic, MessageAcceptance};
use libp2p::PeerId;
use libp2p::{identity::Keypair, identity::Keypair::Ed25519};
use prost::Message;
use std::error::Error;
use tokio::{
    io::{self, AsyncBufReadExt},
    sync::mpsc::Receiver,
};

pub type Swarm = libp2p::Swarm<Behaviour>;
pub fn build_swarm(
    bookkeeper: Bookkeeper,
) -> Result<(Swarm, Receiver<BehaviourEvent>), Box<dyn Error>> {
    // Create a random PeerId
    let local_key: Keypair = Ed25519(bookkeeper.key);
    let local_peer_id: PeerId = PeerId::from(local_key.public());

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key.clone())?;

    let (gossipsub, event_receiver) = Behaviour::new(local_key);

    Ok((
        Swarm::new(transport, gossipsub, local_peer_id),
        event_receiver,
    ))
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

#[tokio::main]
pub async fn dispatcher(
    mut swarm: Swarm,
    mut event_receiver: Receiver<BehaviourEvent>,
    mut rpc_event_receiver: Receiver<BehaviourEvent>,
    mut orderbook_node: Option<Orderbook>,
    dkg: Option<super::dkg::DKG>,
) -> Result<(), Box<dyn Error>> {
    let topic = Topic::new("orderbook");

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Kick it off
    let mut listening = false;
    loop {
        let to_publish = {
            tokio::select! {
                line = stdin.next_line() => {
                    let line = line?.expect("stdin closed");
                    let tix = Intent {
                        asset: line.clone(),
                    };
                    let mut tix_bytes = vec![];
                    tix.encode(&mut tix_bytes).unwrap();
                    Some((topic.clone(), tix_bytes))
                }
                swarm_event = swarm.next() => {
                    // All events are handled by the `NetworkBehaviourEventProcess`es.
                    // I.e. the `swarm.next()` future drives the `Swarm` without ever
                    // terminating.
                    panic!("Unexpected event: {:?}", swarm_event)
                }
                event = event_receiver.recv() =>
                {
                    let event = event.unwrap();
                    if orderbook_node.apply(&event)? {
                        let BehaviourEvent::Message(peer_id, _topic_hash, message_id, _data) = event;
                        swarm.gossipsub.report_message_validation_result(
                            &message_id, &peer_id.unwrap(),
                            MessageAcceptance::Accept).unwrap();
                    }
                    None
                }
            }
        };
        if let Some((topic, bytes)) = to_publish {
            let _res = swarm.gossipsub.publish(topic.clone(), bytes);
        }
        if !listening {
            for addr in Swarm::listeners(&swarm) {
                println!("Listening on {:?}", addr);
                listening = true;
            }
        }
    }
}
