use std::convert::{TryInto};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io::{ErrorKind, Write};

use borsh::{BorshDeserialize, BorshSerialize};
pub use libsecp256k1:: SecretKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::types::{address, Address, DbKeySeg, Key, KeySeg};

const SIGNATURE_LEN: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicKey(libsecp256k1::PublicKey);

// serde struggled with serializing/deserializing the wrapped type
//#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature(libsecp256k1::Signature);

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
pub struct PublicKeyHash(pub(crate) String);

#[derive(Debug)]
pub struct Keypair(libsecp256k1::PublicKey, libsecp256k1::SecretKey);

const PK_STORAGE_KEY: &str = "secp256k1_pk";

/// Obtain a storage key for user's public key.
pub fn pk_key(owner: &Address) -> Key {
    Key::from(owner.to_db_key())
        .push(&PK_STORAGE_KEY.to_owned())
        .expect("Cannot obtain a storage key")
}

/// Check if the given storage key is a public key. If it is, returns the owner.
pub fn is_pk_key(key: &Key) -> Option<&Address> {
    match &key.segments[..] {
        [DbKeySeg::AddressSeg(owner), DbKeySeg::StringSeg(key)]
            if key == PK_STORAGE_KEY =>
        {
            Some(owner)
        }
        _ => None,
    }
}

/// Sign the data with a key.
pub fn sign(keypair: &Keypair, data: impl AsRef<[u8]>) -> Signature {
    let message_to_sign = libsecp256k1::Message::parse_slice(&data.as_ref())
        .expect("Message encoding shouldn't fail");
    Signature(libsecp256k1::sign(&message_to_sign, &keypair.1).0)
}

#[derive(Error, Debug)]
pub enum VerifySigError {
    #[error("Signature verification failed: {0}")]
    SigError(libsecp256k1::Error),
    #[error("Signature verification failed to encode the data: {0}")]
    EncodingError(std::io::Error),
}

/// Check that the public key matches the signature on the given data.
pub fn verify_signature<T: BorshSerialize + BorshDeserialize>(
    pk: &libsecp256k1::PublicKey,
    data: &T,
    sig: &libsecp256k1::Signature,
) -> Result<(), VerifySigError> {
    let bytes = &data.try_to_vec().map_err(VerifySigError::EncodingError)?[..];
    let message = &libsecp256k1::Message::parse_slice(bytes).expect("Error parsing given data");
    let check = libsecp256k1::verify(message, sig, pk);
    match check {
        true => Ok(()),
        false => Err(VerifySigError::SigError(libsecp256k1::Error::InvalidSignature))
    }
}

/// Check that the public key matches the signature on the given raw data.
pub fn verify_signature_raw(
    pk: &libsecp256k1::PublicKey,
    data: &[u8],
    sig: &libsecp256k1::Signature,
) -> Result<(), VerifySigError> {
    let message = &libsecp256k1::Message::parse_slice(data)
        .expect("Error parsing raw data");
    let check = libsecp256k1::verify(message, sig, pk);
    match check {
        true => Ok(()),
        false => Err(VerifySigError::SigError(libsecp256k1::Error::InvalidSignature))
    }
        //.map_err(VerifySigError::SigError)
}

impl BorshDeserialize for PublicKey {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        // deserialize the bytes first
        let bytes: [u8; 65] =
            BorshDeserialize::deserialize(buf).map_err(|e| {
                std::io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Error decoding secp256k1 public key: {}", e),
                )
            })?;
        libsecp256k1::PublicKey::parse(&bytes)
            .map(PublicKey)
            .map_err(|e| {
                std::io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Error decoding secp256k1 public key: {}", e),
                )
            })
    }
}

impl BorshSerialize for PublicKey {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // We need to turn the signature to bytes first..
        let vec = self.0.serialize().to_vec();
        // .. and then encode them with Borsh
        let bytes = vec
            .try_to_vec()
            .expect("Public key bytes encoding shouldn't fail");
        writer.write_all(&bytes)
    }
}

// TODO: add serde deserializer

impl BorshDeserialize for Signature {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        // deserialize the bytes first
        let bytes: [u8; 65] =
            BorshDeserialize::deserialize(buf).map_err(|e| {
                std::io::Error::new(
                    ErrorKind::InvalidInput,
                    format!("Error decoding secp256k1 signature: {}", e),
                )
            })?;
        // convert them to an expected size array
        let bytes: [u8; SIGNATURE_LEN] = bytes[..].try_into().map_err(|e| {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                format!("Error decoding secp256k1 signature: {}", e),
            )
        })?;
        // TODO: use parse_standard; handle errors with match instead
        Ok(Signature(libsecp256k1::Signature::parse_overflowing(&bytes)))
    }
}

impl BorshSerialize for Signature {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // We need to turn the signature to bytes first..
        let vec = self.0.serialize().to_vec();
        // .. and then encode them with Borsh
        let bytes = vec
            .try_to_vec()
            .expect("Signature bytes encoding shouldn't fail");
        writer.write_all(&bytes)
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.try_to_vec()
            .expect("Encoding public key shouldn't fail")
            .hash(state);
    }
}

impl PartialOrd for PublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.try_to_vec()
            .expect("Encoding public key shouldn't fail")
            .partial_cmp(
                &other
                    .try_to_vec()
                    .expect("Encoding public key shouldn't fail"),
            )
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.try_to_vec()
            .expect("Encoding signature for hash shouldn't fail")
            .hash(state);
    }
}

impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.try_to_vec()
            .expect("Encoding signature shouldn't fail")
            .partial_cmp(
                &other
                    .try_to_vec()
                    .expect("Encoding signature shouldn't fail"),
            )
    }
}

impl From<libsecp256k1::PublicKey> for PublicKey {
    fn from(pk: libsecp256k1::PublicKey) -> Self {
        Self(pk)
    }
}

impl From<PublicKey> for PublicKeyHash {
    fn from(pk: PublicKey) -> Self {
        let pk_bytes =
            pk.try_to_vec().expect("Public key encoding shouldn't fail");
        let mut hasher = Sha256::new();
        hasher.update(pk_bytes);
        // hex of the first 40 chars of the hash
        Self(format!(
            "{:.width$X}",
            hasher.finalize(),
            width = address::HASH_LEN
        ))
    }
}
