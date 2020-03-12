use std::collections::HashMap;

pub struct KvStore{
    map: HashMap<String,String>,
}

impl KvStore{
    pub fn new() -> KvStore{
        KvStore{map: HashMap::new()}
    }

    pub fn get(&self, key: String) -> Option<String>{
        self.map.get(&key).cloned()
    }

    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key,val);
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}