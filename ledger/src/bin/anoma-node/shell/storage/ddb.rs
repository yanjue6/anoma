use rocksdb::{BlockBasedOptions, DBIterator, Options, WriteBatch};
use std::{path::{PathBuf}};

#[derive(Debug, Clone)]
pub enum Error {
    MissingKey(String),
    RocksDBError(rocksdb::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait DBOperations<'b> {
    fn get(&self, key: &str) -> Result<Vec<u8>>;
    fn write(&self, key: &str, data: Vec<u8>) -> Result<()>;
    fn delete(&self, key: &str) -> Result<()>;
    fn get_batch(&self) -> WriteBatch;
    fn execute_batch(&self, batch: WriteBatch) -> Result<()>;
    fn prefix_iterator<'b>(&self, prefix: String) -> dyn Iterator<Item=(String, Vec<u8>)>;
}

#[derive(Debug)]
pub struct RocksDB {
    path: PathBuf
}

impl RocksDB {
    fn open(&self) -> Result<rocksdb::DB> {
        let mut cf_opts = Options::default();
        // ! recommended initial setup https://github.com/facebook/rocksdb/wiki/Setup-Options-and-Basic-Tuning#other-general-options
        cf_opts.set_level_compaction_dynamic_level_bytes(true);
        // compactions + flushes
        cf_opts.set_max_background_jobs(6);
        cf_opts.set_bytes_per_sync(1048576);
        // TODO the recommended default `options.compaction_pri =
        // kMinOverlappingRatio` doesn't seem to be available in Rust
        let mut table_opts = BlockBasedOptions::default();
        table_opts.set_block_size(16 * 1024);
        table_opts.set_cache_index_and_filter_blocks(true);
        table_opts.set_pin_l0_filter_and_index_blocks_in_cache(true);
        // latest format versions https://github.com/facebook/rocksdb/blob/d1c510baecc1aef758f91f786c4fbee3bc847a63/include/rocksdb/table.h#L394
        table_opts.set_format_version(5);
        cf_opts.set_block_based_table_factory(&table_opts);

        cf_opts.create_missing_column_families(true);
        cf_opts.create_if_missing(true);

        // TODO use column families
        rocksdb::DB::open_cf_descriptors(&cf_opts, &self.path, vec![]).map_err(Error::RocksDBError)
    }
}

impl DBOperations<'b> for RocksDB {

    fn get(&self, key: &str) -> Result<Vec<u8>> {
        let db = self.open()?;
        match db.get(key.as_bytes()) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(Error::MissingKey(key.to_owned())),
            Err(why) => Err(Error::RocksDBError(why))
        }
    }

    fn write(&self, key: &str, data: Vec<u8>) -> Result<()> {
        let db = self.open()?;
        db.put(key.as_bytes(), data).map_err(|e| Error::RocksDBError(e));
        db.flush().map_err(|e| Error::RocksDBError(e))
    }

    fn delete(&self, key: &str) -> Result<()> {
        let db = self.open()?;
        db.delete(key.as_bytes()).map_err(|e| Error::RocksDBError(e));
        db.flush().map_err(|e| Error::RocksDBError(e))
    }

    fn get_batch(&self) -> WriteBatch {
        WriteBatch::default()
    }

    fn execute_batch(&self, batch: WriteBatch) -> Result<()> {
        let db = self.open()?;
        Ok(db.write(batch).is_ok()).map_err(|e| Error::RocksDBError(e));
        db.flush().map_err(|e| Error::RocksDBError(e))
    }

    fn prefix_iterator<'b>(&self, prefix: String) -> DBIterator<'b> {
        let db = self.open().unwrap();
        db.prefix_iterator(&prefix)
    }
}

// TODO: add batch object