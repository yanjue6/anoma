use std::collections::HashMap;

use crate::shell::storage::PersistentPrefixIterator;

// TODO make this generic over the Iterator type
pub struct PrefixIterators<'a> {
    index: PrefixIteratorId,
    iterators: HashMap<PrefixIteratorId, PersistentPrefixIterator<'a>>,
}

impl<'a> PrefixIterators<'a> {
    pub fn new() -> Self {
        PrefixIterators {
            index: PrefixIteratorId::new(0),
            iterators: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        iter: PersistentPrefixIterator<'a>,
    ) -> PrefixIteratorId {
        let id = self.index;
        self.iterators.insert(id, iter);
        self.index = id.next_id();
        id
    }

    /// Returns a key-value pair and the gas cost
    pub fn next(
        &mut self,
        id: PrefixIteratorId,
    ) -> Option<(String, Vec<u8>, u64)> {
        match self.iterators.get_mut(&id) {
            Some(iter) => iter.next(),
            None => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PrefixIteratorId(u64);

impl PrefixIteratorId {
    pub fn new(id: u64) -> Self {
        PrefixIteratorId(id)
    }

    pub fn id(&self) -> u64 {
        self.0
    }

    fn next_id(&self) -> PrefixIteratorId {
        PrefixIteratorId(self.0 + 1)
    }
}
