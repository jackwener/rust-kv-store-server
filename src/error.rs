#[derive(Debug)]
pub enum KVError {}

pub type Result<T> = std::result::Result<T, KVError>;
