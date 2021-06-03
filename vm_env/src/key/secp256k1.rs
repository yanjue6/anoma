use anoma_shared::types::key::secp256k1::{self, PublicKey};
use anoma_shared::types::Address;

use crate::imports::vp;

/// Get the public key associated with the given address. Panics if not found.
pub fn get(owner: &Address) -> Option<PublicKey> {
    let key = secp256k1::pk_key(owner).to_string();
    vp::read_pre(&key)
}
