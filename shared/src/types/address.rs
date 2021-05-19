//! Implements transparent addresses as described in [Accounts
//! Addresses](tech-specs/src/explore/design/ledger/accounts.md#addresses).

use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::FromIterator;
use std::string;

use bech32::{self, FromBase32, ToBase32, Variant};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::types::key;

/// human-readable part of Bech32m encoded address
const ADDRESS_HRP: &str = "a";
const ADDRESS_BECH32_VARIANT: bech32::Variant = Variant::Bech32m;
pub(crate) const HASH_LEN: usize = 40;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error decoding address from Bech32m: {0}")]
    DecodeBech32(bech32::Error),
    #[error("Error decoding address from base32: {0}")]
    DecodeBase32(bech32::Error),
    #[error(
        "Unexpected Bech32m human-readable part {0}, expected {ADDRESS_HRP}"
    )]
    UnexpectedBech32Prefix(String),
    #[error(
        "Unexpected Bech32m variant {0:?}, expected {ADDRESS_BECH32_VARIANT:?}"
    )]
    UnexpectedBech32Variant(bech32::Variant),
    #[error("Address must be encoded with utf-8")]
    NonUtf8Address(string::FromUtf8Error),
    #[error("Invalid address encoding")]
    InvalidAddressEncoding(std::io::Error),
    #[error("Unexpected address hash length {0}, expected {HASH_LEN}")]
    UnexpectedHashLength(usize),
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum Address {
    Established(EstablishedAddress),
    Implicit(ImplicitAddress),
}

impl Address {
    /// Encode an address with Bech32m encoding
    pub fn encode(&self) -> String {
        let bytes = self
            .try_to_vec()
            .expect("Encoding an address shouldn't fail");
        bech32::encode(ADDRESS_HRP, bytes.to_base32(), ADDRESS_BECH32_VARIANT)
            .unwrap_or_else(|_| {
                panic!(
                    "The human-readable part {} should never cause a failure",
                    ADDRESS_HRP
                )
            })
    }

    /// Decode an address from Bech32m encoding
    pub fn decode(string: impl AsRef<str>) -> Result<Self> {
        let (prefix, hash_base32, variant) =
            bech32::decode(string.as_ref()).map_err(Error::DecodeBech32)?;
        if prefix != ADDRESS_HRP {
            return Err(Error::UnexpectedBech32Prefix(prefix));
        }
        match variant {
            ADDRESS_BECH32_VARIANT => {}
            _ => return Err(Error::UnexpectedBech32Variant(variant)),
        }
        let bytes: Vec<u8> = FromBase32::from_base32(&hash_base32)
            .map_err(Error::DecodeBase32)?;
        let address = BorshDeserialize::try_from_slice(&bytes[..])
            .map_err(Error::InvalidAddressEncoding)?;
        match &address {
            Address::Established(established) => {
                if established.hash.len() != HASH_LEN {
                    return Err(Error::UnexpectedHashLength(
                        established.hash.len(),
                    ));
                }
            }
            Address::Implicit(ImplicitAddress::Ed25519(pkh)) => {
                if pkh.0.len() != HASH_LEN {
                    return Err(Error::UnexpectedHashLength(pkh.0.len()));
                }
            }
        }
        Ok(address)
    }

    fn pretty_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            match self {
                Address::Established(_) => {
                    "Established"
                }
                Address::Implicit(_) => {
                    "Implicit"
                }
            },
            self.encode(),
        )
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_fmt(f)
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_fmt(f)
    }
}

#[derive(
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct EstablishedAddress {
    hash: String,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct EstablishedAddressGen {
    last_hash: String,
}

impl EstablishedAddressGen {
    pub fn new(seed: impl AsRef<str>) -> Self {
        Self {
            last_hash: seed.as_ref().to_owned(),
        }
    }

    /// Generate a new established address. Requires a source of randomness as
    /// arbitrary bytes. In the ledger, this could be some unpredictable value,
    /// such as hash of the transaction that has initialized the new address.
    pub fn generate_address(
        &mut self,
        rng_source: impl AsRef<[u8]>,
    ) -> Address {
        let gen_bytes = self
            .try_to_vec()
            .expect("Encoding established addresses generator shouldn't fail");
        let mut hasher = Sha256::new();
        let bytes = [&gen_bytes, rng_source.as_ref()].concat();
        hasher.update(bytes);
        // hex of the first 40 chars of the hash
        let hash = format!("{:.width$X}", hasher.finalize(), width = HASH_LEN);
        self.last_hash = hash.clone();
        Address::Established(EstablishedAddress { hash })
    }
}

#[derive(
    Debug,
    Clone,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ImplicitAddress {
    Ed25519(key::ed25519::PublicKeyHash),
}

pub fn adrian() -> Address {
    Address::decode("a1qq5qqqqqxgeyzdeng4zrw33sxez5y3p3xqerz3psx5e5x32rg3rryw2ygc6yy3p4xpq5gvfnw3nwp8").expect("The address decoding shouldn't fail")
}
pub fn alberto() -> Address {
    Address::decode("a1qq5qqqqq8yerw3jxx565y333gfpnjwzygcc5zd6xxarr2dzzgcm5xv3kxazrjve589p5vv34vl0yy3").expect("The address decoding shouldn't fail")
}
pub fn ash() -> Address {
    Address::decode("a1qq5qqqqqxue5vs69xc6nwvfcgdpyy3pnxv6rxw2zx3zryv33gyc5xdekxaryydehgvunsvzz2hjedu").expect("The address decoding shouldn't fail")
}
pub fn awa() -> Address {
    Address::decode("a1qq5qqqqqg565zv34gcc52v3nxumr23z9gezrj3pnx56rwse4xc6yg3phgcun2d33xyenqv2x4xyw62").expect("The address decoding shouldn't fail")
}
pub fn celso() -> Address {
    Address::decode("a1qq5qqqqq8qmrwsjyxcerqwzpx9pnzve3gvc5xw29gdqnvv2yx5mrvsjpxgcrxv6pg5engvf5hgjscj").expect("The address decoding shouldn't fail")
}
pub fn chris() -> Address {
    Address::decode("a1qq5qqqqqgye5xwpcxqu5z3p4g5ens3zr8qm5xv69xfznvwzzx4p5xwpkxc6n2v6x8yc5gdpeezdqc4").expect("The address decoding shouldn't fail")
}
pub fn gabriella() -> Address {
    Address::decode("a1qq5qqqqq8ycn2djrxqmnyd3sxcunsv2zgyeyvwzpgceyxdf3xyu5gv2pgeprxdfe8ycrzwzzkezpcp").expect("The address decoding shouldn't fail")
}
pub fn gianmarco() -> Address {
    Address::decode("a1qq5qqqqq89prqsf38qcrzd6zxym5xdfjg4pyg3pjg3pyx32zg5u5y3jpgc65zdej8pznwwf3jqzsws").expect("The address decoding shouldn't fail")
}
pub fn joe() -> Address {
    Address::decode("a1qq5qqqqqgvuyv335g9z5v32xgdz523zxgsuy23fjxazrjve5g4pnydphxyu5v33cxarrzd692045xh").expect("The address decoding shouldn't fail")
}
pub fn nat() -> Address {
    Address::decode("a1qq5qqqqq89rygsejx9q5yd6pxpp5x3f38ymyydp3xcu523zzx4prw3fc8qu5vvjpxyeyydpnfha6qt").expect("The address decoding shouldn't fail")
}
pub fn simon() -> Address {
    Address::decode("a1qq5qqqqqgfqnqdecxcurq33hxcey2sf4g5mygdjyxfrrjse4xyc52vpjxyenwve4gv6njsecz4tzen").expect("The address decoding shouldn't fail")
}
pub fn sylvain() -> Address {
    Address::decode("a1qq5qqqqqgccnyvp3gyergvp5xgmr2s3s8yung3f4gdq52wzpxvurysfhgycnwd29xfryxvekfwc00t").expect("The address decoding shouldn't fail")
}
pub fn tomas() -> Address {
    Address::decode("a1qq5qqqqqggcrzsfj8ym5g3psxuurxv2yxseyxwpsxdpy2s35gsc5zdzpx9pyxde48ppnqd3cnzlava").expect("The address decoding shouldn't fail")
}
pub fn yuji() -> Address {
    Address::decode("a1qq5qqqqqgvcrz3f5x4prssj9x5enydecxfznzdj9g5cnj3fcxarrxdjpx5cnwv69xye5vvfeva4z85").expect("The address decoding shouldn't fail")
}

/// Temporary helper for testing
pub fn xan() -> Address {
    Address::decode("a1qq5qqqqqxuc5gvz9gycryv3sgye5v3j9gvurjv34g9prsd6x8qu5xs2ygdzrzsf38q6rss33xf42f3").expect("The token address decoding shouldn't fail")
}

/// Temporary helper for testing
pub fn btc() -> Address {
    Address::decode("a1qq5qqqqq8q6yy3p4xyurys3n8qerz3zxxeryyv6rg4pnxdf3x3pyv32rx3zrgwzpxu6ny32r3laduc").expect("The token address decoding shouldn't fail")
}

/// Temporary helper for testing
pub fn eth() -> Address {
    Address::decode("a1qq5qqqqqxeryzvjxxsmrj3jpxapygve58qerwsfjxaznvd3n8qenyv2ygsc52335xue5vve5m66gfm").expect("The token address decoding shouldn't fail")
}

/// Temporary helper for testing
pub fn xtz() -> Address {
    Address::decode("a1qq5qqqqqx3z5xd3ngdqnzwzrgfpnxd3hgsuyx3phgfry2s3kxsc5xves8qe5x33sgdprzvjptzfry9").expect("The token address decoding shouldn't fail")
}

/// Temporary helper for testing
pub fn doge() -> Address {
    Address::decode("a1qq5qqqqqx9rrq3zrg5myzv3eg9zyxvf3gery2dfhgg6nsdjrxscrgv6rgsunx33sxg6nvdjrkujezz").expect("The token address decoding shouldn't fail")
}

/// Temporary helper for testing
pub fn matchmaker() -> Address {
    Address::decode("a1qq5qqqqqxu6rvdzpxymnqwfkxfznvsjxggunyd3jg5erg3p3geqnvv35gep5yvzxx5m5x3fsfje8td").expect("The token address decoding shouldn't fail")
}

impl<'a> FromIterator<&'a Address> for HashSet<Address> {
    fn from_iter<T: IntoIterator<Item = &'a Address>>(iter: T) -> Self {
        let mut set = HashSet::new();
        for addr in iter {
            set.insert(addr.clone());
        }
        set
    }
}

#[cfg(test)]
pub mod tests {
    use rand::prelude::ThreadRng;
    use rand::{thread_rng, RngCore};

    use super::*;

    /// Run `cargo test gen_established_address -- --nocapture` to generate a
    /// new established address.
    #[test]
    fn gen_established_address() {
        let seed = "such randomness, much wow";
        let mut key_gen = EstablishedAddressGen::new(seed);

        let mut rng: ThreadRng = thread_rng();
        let mut rng_bytes = vec![0u8; 32];
        rng.fill_bytes(&mut rng_bytes[..]);
        let rng_source = rng_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join("");
        for _ in 0..13 {
            let address = key_gen.generate_address(rng_source.clone());
            println!("address {}", address);
        }
    }

    /// A sampled established address for tests
    pub fn established_address_1() -> Address {
        Address::decode("a1qq5qqqqqgcmyxd35xguy2wp5xsu5vs6pxqcy232pgvm5zs6yggunssfs89znv33h8q6rjde4cjc3dr").expect("The token address decoding shouldn't fail")
    }

    /// A sampled established address for tests
    pub fn established_address_2() -> Address {
        Address::decode("a1qq5qqqqqgcuyxv2pxgcrzdecx4prq3pexccr2vj9xse5gvf3gvmnv3f3xqcyyvjyxv6yvv34e393x7").expect("The token address decoding shouldn't fail")
    }
}
