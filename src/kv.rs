use crate::error::Result;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

pub struct KVStore {
    path: PathBuf,
    file: File,
    cursor: u64,
    threshold: u64,
    map: HashMap<String, u64>,
}

impl KVStore {
    pub fn new() -> Result<KVStore> {
        KVStore {}
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KVStore> {
        let path = path.into();
        std::fs::create_dir_all(&path)?;

        let log = path.join("log");
        let mut log_file = OpenOptions(path)
            .create(true)
            .read(true)
            .write(true)
            .open(log);

        let mut mem_table = HashMap::new();
        let mut cursor = 0: u64;
    }

    pub fn put(&mut self, key: String, value: String) {
        // 获取当前
        let file_id = self;

        let ts = time();
    }

    pub fn get(&self, key: String) -> Option<String> {
        let value = self.map.get(&key);
        return value.cloned();
    }

    pub fn rm(&mut self, key: String) {
        self.map.remove(&key);
    }
}
