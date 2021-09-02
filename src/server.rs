use crate::common::{GetResponse, Request, SetResponse};
use crate::engines::KVEngine;
use crate::error::Result;
use log::{debug, error};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct KVServer<Engine: KVEngine> {
    engine: Engine,
}

impl<Engine: KVEngine> KVServer<Engine> {
    pub fn new(engine: Engine) -> KVServer<Engine> {
        KVServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.serve(stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn serve(&mut self, tcp: TcpStream) -> Result<()> {
        let reader = BufReader::new(&tcp);
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

        let mut writer = BufWriter::new(&tcp);
        for req in req_reader {
            match req? {
                Request::Get { key } => {
                    match self.engine.get(key) {
                        Ok(value) => {
                            serde_json::to_writer(&writer, &GetResponse::Ok(value));
                        }
                        Err(e) => {
                            serde_json::to_writer(&writer, &GetResponse::Err(e));
                        }
                    }
                    writer.flush()?;
                }
                Request::Set { key, value } => {
                    match self.engine.set(key) {
                        Ok(_) => {
                            serde_json::to_writer(&writer, &SetResponse::Ok(()));
                        }
                        Err(e) => {
                            serde_json::to_writer(&writer, &SetResponse::Err(e));
                        }
                    }
                    writer.flush()?;
                }
                Request::Remove { key } => {}
            }
        }

        Ok(())
    }
}
