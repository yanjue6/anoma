use super::{
    config::NetworkConfig,
    orderbook::{self, Orderbook},
};
use super::{
    dkg::DKG,
    network_behaviour::{Behaviour, BehaviourEvent},
};
use anoma::bookkeeper::Bookkeeper;
use anoma::protobuf::gossip::Intent;
use libp2p::gossipsub::{IdentTopic as Topic, MessageAcceptance};
use libp2p::PeerId;
use libp2p::{identity::Keypair, identity::Keypair::Ed25519};
use prost::Message;
use std::error::Error;
use tokio::{io::{self, AsyncBufReadExt}, sync::mpsc::Receiver};

pub type Swarm = libp2p::Swarm<Behaviour>;
pub fn build_swarm(
    bookkeeper: Bookkeeper,
) -> Result<(Swarm, Receiver<BehaviourEvent>), Box<dyn Error>> {
    // Create a random PeerId
    let local_key: Keypair = Ed25519(bookkeeper.key);
    let local_peer_id: PeerId = PeerId::from(local_key.public());

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::build_development_transport(local_key.clone())?;

    let (gossipsub, network_event_receiver) = Behaviour::new(local_key);

    Ok((
        Swarm::new(transport, gossipsub, local_peer_id),
        network_event_receiver,
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
    mut network_event_receiver: Receiver<BehaviourEvent>,
    mut rpc_event_receiver: Option<Receiver<Intent>>,
    orderbook_node: Option<Orderbook>,
    dkg_node: Option<DKG>,
) -> Result<(), Box<dyn Error>> {
    let mut listening = false;

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    if orderbook_node.is_none() && dkg_node.is_none() {
        panic!("Need at least one module to be active, orderbook or dkg")
    }
    let mut orderbook_node: Orderbook = orderbook_node.unwrap();
    // let mut rpc_event_receiver = rpc_event_receiver_opt.unwrap();
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                let tix = Intent {
                    asset: line.clone(),
                };
                let mut tix_bytes = vec![];
                tix.encode(&mut tix_bytes).unwrap();
                swarm.gossipsub.publish(
                    Topic::new(String::from(orderbook::TOPIC)),
                    tix_bytes).unwrap();
                }
            // event = async {
            //     if let Some (mut rpc_event_receiver) = rpc_event_receiver.take() {
            //         rpc_event_receiver.recv().await
            //     }
            //     else { None }}, if rpc_event_receiver.is_some() =>
            // {
            //     println!("RPC RECEIVED {:?}", event);
            //     if let Some(event) = event{
            //         let mut tix_bytes = vec![];
            //         event.encode(&mut tix_bytes).unwrap();
            //         swarm.gossipsub.publish(
            //             Topic::new(String::from(orderbook::TOPIC)),
            //             tix_bytes)
            //             .unwrap();
            //     }
            // }
            swarm_event = swarm.next() => {
                // All events are handled by the `NetworkBehaviourEventProcess`es.
                // I.e. the `swarm.next()` future drives the `Swarm` without ever
                // terminating.
                panic!("Unexpected event: {:?}", swarm_event);
            }
            event = network_event_receiver.recv() =>
            {
                println!("NETWORK RECEIVED {:?}", event);
                if let Some(event) = event{
                    if orderbook_node.apply(&event)? {
                        let BehaviourEvent::Message(peer_id,
                                                    _topic_hash,
                                                    message_id,
                                                    _data) = event;
                        swarm.gossipsub
                            .report_message_validation_result(
                                &message_id,
                                &peer_id.unwrap(),
                                MessageAcceptance::Accept,)
                        .unwrap();
                    }
                }
                    }
        };
        // if let Some((topic, bytes)) = to_publish {
        //     swarm.gossipsub.publish(topic, bytes).unwrap();
        // }
        if !listening {
            for addr in Swarm::listeners(&swarm) {
                println!("Listening on {:?}", addr);
                listening = true;
            }
        }
    }
}
