use crate::error::{Result}
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter};
use std::net::{TcpStream, ToSocketAddrs};


pub struct KVClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufReader<TcpStream>,
}

impl KVClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
            writer: BufWriter::new(tcp_writer),
        })
    }

    pub fn get(key: String) ->Result<Option<String>>{


    }

    pub fn set(key: String, value: String) -> Result<()>{

        Ok(())
    }

    pub fn remove(key:String) -> Result<()> {

    }
}