use log::{debug, error,warn, log_enabled, info, Level};
use log::LevelFilter;
use std::env::current_dir;
use kvs::{KVEngine, KVServer};
use kvs::Result;
use std::net::SocketAddr;
use kvs::

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

}

fn run_with_engine<E: KVEngine>(engine: E, addr: SocketAddr) -> Result<()> {
    let server = KVServer::new(engine);
    server.run(addr)
}

fn engine() -> Result<Option<KVEngine>> {

}