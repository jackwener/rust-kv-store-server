use crate::engines::KVEngine;

pub struct KVServer<Engine: KVEngine> {
    engine: Engine,
}

impl<Engine: KVEngine> KVServer<Engine> {
    fn new(engine: Engine) -> KVServer<Engine> {
        KVServer { engine }
    }

}

