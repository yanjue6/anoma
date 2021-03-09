mod config;
mod dkg;
mod mempool;
mod network_behaviour;
mod orderbook;
mod p2p;

use self::config::NetworkConfig;
use self::orderbook::Orderbook;
use anoma::{bookkeeper::Bookkeeper, config::*, protobuf::gossip::Intent};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::{io::Write, path::PathBuf};
use tokio::sync::mpsc::Receiver;

#[warn(unused_variables)]
pub fn run(
    config: Config,
    rpc_event_receiver: Option<Receiver<Intent>>,
    local_address: Option<String>,
    peers: Option<Vec<String>>,
    topics: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let base_dir: PathBuf = config.gossip_home_dir();
    let bookkeeper: Bookkeeper = read_or_generate_bookkeeper_key(&base_dir)?;

    // Create a Gossipsub topic
    let network_config = NetworkConfig::read_or_generate(
        &base_dir,
        local_address,
        peers,
        topics,
    );

    let (mut swarm, event_receiver) = p2p::build_swarm(bookkeeper)?;
    p2p::prepare_swarm(&mut swarm, &network_config);
    p2p::dispatcher(
        swarm,
        event_receiver,
        rpc_event_receiver,
        Some(Orderbook::new()),
        None,
    )
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
