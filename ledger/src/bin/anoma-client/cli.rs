//! The docstrings on types and their fields with `derive(Clap)` are displayed
//! in the CLI `--help`.

use anoma::cli::{ClientOpts, Gossip, InlinedClientOpts, Transfer};
use anoma::types::{Intent, Message, Transaction};
use clap::Clap;
use libp2p::gossipsub::{
    Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic,
    MessageAuthenticity, ValidationMode,
};
use libp2p::{gossipsub, identity, PeerId};
use reqwest;
use tendermint_rpc::{Client, HttpClient};

pub async fn main() {
    match ClientOpts::parse() {
        ClientOpts::Inlined(ops) => exec_inlined(ops).await,
    }
}

async fn exec_inlined(ops: InlinedClientOpts) {
    match ops {
        InlinedClientOpts::Transfer(transaction) => transfer(transaction).await,
        InlinedClientOpts::Gossip(Gossip {
            orderbook_addr,
            msg,
        }) => gossip(orderbook_addr, msg).await,
    }
}

async fn transfer(Transfer { src, dest, amount }: Transfer) {
    // TODO add a counter
    let tx = Transaction { src, dest, amount };
    let mut tx_bytes = vec![];
    tx.encode(&mut tx_bytes).unwrap();
    let client =
        HttpClient::new("tcp://127.0.0.1:26657".parse().unwrap()).unwrap();
    // TODO broadcast_tx_commit shouldn't be used live
    let response = client.broadcast_tx_commit(tx_bytes.into()).await;
    println!("{:#?}", response);
}

async fn gossip(orderbook_addr: String, msg: String) {
    let local_key = identity::Keypair::generate_ed25519();
    let peerId = PeerId::from_public_key(local_key.public());
    let tix = Intent { msg };
    let mut tix_bytes = vec![];
    tix.encode(&mut tix_bytes).unwrap();
    let response = reqwest::Client::new()
        .post(&orderbook_addr)
        .body(tix_bytes)
        .send()
        .await;
    println!("{:#?}", response);
}
