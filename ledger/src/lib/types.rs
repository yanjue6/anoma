use prost;
pub use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Message)]
pub struct Transaction {
    #[prost(string)]
    pub src: String,
    #[prost(string)]
    pub dest: String,
    #[prost(uint64)]
    pub amount: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Intent {
    pub msg: String,
}

#[derive(Clone, Eq, PartialEq, Message)]
pub struct Dkg {
    #[prost(string)]
    pub msg: String,
}

#[test]
fn encoding_round_trip() {
    let tx = Transaction {
        src: "a".to_owned(),
        dest: "b".to_owned(),
        amount: 10,
    };
    let mut tx_bytes = vec![];
    tx.encode(&mut tx_bytes).unwrap();
    let tx_hex = hex::encode(tx_bytes);
    let tx_from_hex = hex::decode(tx_hex).unwrap();
    let tx_from_bytes = Transaction::decode(&tx_from_hex[..]).unwrap();
    assert_eq!(tx, tx_from_bytes);
}
