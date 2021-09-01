use std::collections::BTreeMap;

pub struct TreeEntry {
    file_id: u64,
    value_size: u64,
    offset: u64,
    timestamp,
}