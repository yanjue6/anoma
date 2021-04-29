//! The persistent storage
//!
//! The current storage tree is:
//! - `chain_id`
//! - `height`: the last committed block height
//! - `h`: for each block at height `h`:
//!   - `tree`: merkle tree
//!     - `root`: root hash
//!     - `store`: the tree's store
//!   - `hash`: block hash
//!   - `subspace`: any byte data

mod mock;
mod rocksdb;

use std::collections::HashMap;
use std::path::Path;

use anoma_shared::types::{BlockHash, BlockHeight, Key};
use thiserror::Error;

use super::types::{MerkleTree, PrefixIterator};

type Result<T> = std::result::Result<T, Error>;

// TODO the DB schema will probably need some kind of versioning

#[derive(Error, Debug)]
pub enum Error {
    #[error("TEMPORARY error: {error}")]
    Temporary { error: String },
    #[error("Found an unknown key: {key}")]
    UnknownKey { key: String },
    #[error("Key error {0}")]
    KeyError(anoma_shared::types::Error),
    #[error("Found an unknown DB type: {db_type}")]
    UnknownDB { db_type: String },
    #[error("DB error: {error}")]
    DBError { error: String },
}

pub(super) struct BlockState {
    pub chain_id: String,
    pub tree: MerkleTree,
    pub hash: BlockHash,
    pub height: BlockHeight,
    pub subspaces: HashMap<Key, Vec<u8>>,
}

pub(super) trait DB: std::fmt::Debug {
    /// Flush data on the memory to persistent them
    fn flush(&self) -> Result<()>;

    /// Write a block
    fn write_block(
        &mut self,
        tree: &MerkleTree,
        hash: &BlockHash,
        height: BlockHeight,
        subspaces: &HashMap<Key, Vec<u8>>,
    ) -> Result<()>;

    /// Write the chain ID
    #[allow(clippy::ptr_arg)]
    fn write_chain_id(&mut self, chain_id: &String) -> Result<()>;

    /// Read the value with the given height and the key from the DB
    fn read(&self, height: BlockHeight, key: &Key) -> Result<Option<Vec<u8>>>;

    /// Read key value pairs with the given prefix from the DB
    fn iter_prefix(&self, height: BlockHeight, prefix: &Key) -> PrefixIterator;

    /// Read the last committed block
    fn read_last_block(&mut self) -> Result<Option<BlockState>>;
}

pub(super) fn open(
    db_type: &str,
    path: impl AsRef<Path>,
) -> Result<Box<dyn DB>> {
    match db_type {
        "rocksdb" => Ok(Box::new(rocksdb::open(path)?)),
        "mock" => Ok(Box::new(mock::open()?)),
        _ => Err(Error::UnknownDB {
            db_type: db_type.to_owned(),
        }),
    }
}
