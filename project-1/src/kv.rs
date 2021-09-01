use std::collections::{HashMap};

pub struct KVStore {
    map: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> KVStore {
        KVStore {
            map: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let value = self.map.get(&key);
        return value.cloned();
    }

    pub fn rm(&mut self, key: String) {
        self.map.remove(&key);
    }
}