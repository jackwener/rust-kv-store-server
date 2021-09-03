use crate::KVEngine;
use std::collections::BTreeMap;
use crate::error::Result;

type BTree = BTreeMap<String, String>;

#[derive(Clone)]
pub struct BTreeKV(BTree);

impl BTreeKV {
    pub fn new() -> Self {
        BTreeKV(BTree::new())
    }
}

impl KvsEngine for BTreeKV {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        &self.0.insert(key, value)?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.0.get(&key).cloned())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.0.remove(&key);
        Ok(())
    }
}