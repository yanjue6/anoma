use crate::shell::storage::types;
use crate::shell::storage::ddb;

pub use self::types::{
    Address, Balance, BasicAddress, BlockHash, {BlockHeight, KeySeg}, {MerkleTree, Value},
    ValidatorAddress, Account
};
use ddb::{RocksDB, DBOperations};
use std::{collections::HashMap};
use sparse_merkle_tree::{H256};

trait StorageOperations {
    fn commit_block(&self) -> Result<()>;
    fn read_block(&self, height: BlockHeight) -> Result<BlockStorage>;
    fn write_chain_id(&self, chain_id: &String) -> Result<bool>;
    fn read_chain_id(&self) -> Result<String>;
}

trait BlockStorageEncode {
    fn db_prefix(&self) -> Result<String>;
    fn db_encode_tree(&self) -> Result<(String, &[u8])>;
    fn db_encode_hash(&self) -> Result<(String, Vec<u8>)>;
    fn db_encode_height(&self) -> Result<(String, Vec<u8>)>;
    fn db_encode_balances(&self) -> Result<Vec<(String, Vec<u8>)>>;
}

trait BlockStorageDecode {
    fn db_decode_prefix(&self, bytes: Option<Vec<u8>>) -> Result<String>;
    fn db_decode_tree(&self, bytes: Option<Vec<u8>>) -> Result<H256>;
    fn db_decode_hash(&self, bytes: Option<Vec<u8>>) -> Result<H256>;
    fn db_decode_height(&self, bytes: Option<Vec<u8>>) -> Result<BlockHash>;
    fn db_decode_account(&self, bytes: Option<(Box<[u8]>, Box<[u8]>)>, prefix: String) -> Result<(Address, Balance)>;
}

#[derive(Debug, Clone)]
pub enum Error {
    // TODO strong types
    NullDecode,
    BadAddressEncode,
    DBError(ddb::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub struct BlockStorage {
    tree: MerkleTree,
    hash: BlockHash,
    height: BlockHeight,
    balances: HashMap<Address, Balance>,
}

impl BlockStorageEncode for BlockStorage {
    fn db_prefix(&self) -> Result<String> {
        let prefix = format!("{}/tree", self.height.to_key_seg());
        Ok(prefix)
    }

    fn db_encode_tree(&self) -> Result<(String, &[u8])> {
        let prefix = self.db_prefix()?;
        let key = format!("{}/root", prefix);
        let value = self.tree.0.root().as_slice();
        Ok((key, value))
    }

    fn db_encode_hash(&self) -> Result<(String, Vec<u8>)> {
        let prefix = self.db_prefix()?;
        let key = format!("{}/root", prefix);
        let value = self.tree.0.store();
        Ok((key, value.encode()))
    }

    fn db_encode_height(&self) -> Result<(String, Vec<u8>)> {
        let prefix = self.db_prefix()?;
        let key = format!("{}/hash", prefix);
        let value = self.hash.clone();
        Ok((key, value.encode()))
    }

    fn db_encode_balances(&self) -> Result<Vec<(String, Vec<u8>)>> {
        let prefix = self.db_prefix()?;
        let test =  self.balances.iter().map(|(addr, balance)| {
            let key = format!("{}/balance/{}", prefix, addr.to_key_seg());
            return (key, balance.encode());
        }).collect();
        Ok(test)
    }
}

impl BlockStorageDecode for BlockStorage {
    fn db_decode_prefix(&self, bytes: Option<Vec<u8>>) -> Result<String> {
        match bytes {
            Some(b) => Ok(String::decode(b)),
            None => Err(Error::NullDecode)
        }
    }

    fn db_decode_tree(&self, bytes: Option<Vec<u8>>) -> Result<H256> {
        match bytes {
            Some(b) => Ok(H256::decode(b)),
            None => Err(Error::NullDecode)
        }
    }

    fn db_decode_hash(&self, bytes: Option<Vec<u8>>) -> Result<H256> {
        match bytes {
            Some(b) => Ok(H256::decode(b)),
            None => Err(Error::NullDecode)
        }
    }

    fn db_decode_height(&self, bytes: Option<Vec<u8>>) -> Result<BlockHash> {
        match bytes {
            Some(b) => Ok(BlockHash::decode(b)),
            None => Err(Error::NullDecode)
        }
    }

    fn db_decode_account(&self, account: Option<(Box<[u8]>, Box<[u8]>)>, prefix: String) -> Result<(Address, Balance)> {
        match account {
            Some(info) => {
                let path = &String::from_utf8(info.0.to_vec());
                let balance = Balance::decode(info.1.to_vec());
                match path {
                    Ok(s) => {
                        match s.strip_prefix(&prefix) {
                            Some(a) => {
                                let addr = Address::from_key_seg(&a.to_owned()).unwrap();
                                return Ok((addr, balance));
                            }
                            None => return Err(Error::BadAddressEncode)
                        }
                    }
                    Err(_) => return Err(Error::BadAddressEncode)
                }
            }
            None => return Err(Error::BadAddressEncode)
        }
    }
}

pub struct Storage {
    db: RocksDB,
    chain_id: String,
    block: BlockStorage,
    current_block: BlockStorage
}

impl StorageOperations for Storage {
    fn commit_block(&self) -> Result<()> {
        let mut batch = self.db.get_batch();

        let (root_key,root_valuee) = self.current_block.db_encode_hash()?;
        batch.put(root_key, root_valuee);

        // self.current_block.tree.write_to_batch(&batch);

        let (tree_key, tree_value) = self.current_block.db_encode_tree()?;
        batch.put(tree_key, tree_value);

        let (hash_key,hash_valuee) = self.current_block.db_encode_tree()?;
        batch.put(hash_key, hash_valuee);

        let balances = self.current_block.db_encode_balances()?;
        balances.iter().for_each(|(balance_key,balance_valuee)| {
            batch.put(balance_key, balance_valuee);
        });

        self.db.execute_batch(batch).map_err(Error::DBError)
    }

    fn read_block(&self, height: BlockHeight) -> Result<BlockStorage> {
        for (key, bytes) in self.db.prefix_iterator(&height.to_key_seg()) {
            self.current_block.db_decode_account(Option<(key, byes)>)
        }
        Ok()
    }

    fn write_chain_id(&self, chain_id: &String) -> Result<bool> {
        Ok(self.db.write("chain_id", chain_id.encode()).is_ok())
    }

    fn read_chain_id(&self) -> Result<String> {
        let chain_id = self.db.get("chain_id");
        match chain_id {
            Ok(c) => {
                Ok(String::decode(c))
            }
            Err(_) => Err(Error::NullDecode)
        }
    }
}