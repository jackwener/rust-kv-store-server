use crate::engines::KVEngine;
use std::net::{TcpListener, ToSocketAddrs, TcpStream};


pub struct KVServer<Engine: KVEngine> {
    engine: Engine,
}

impl<Engine: KVEngine> KVServer<Engine> {
    pub fn new(engine: Engine) -> KVServer<Engine> {
        KVServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.serve(stream).unwrap_err();
                }
                Err(e) => {}
            }
        }
        Ok(())
    }

    fn serve(&mut self, tcp: TcpStream) -> Result<()> {}
    }
}

