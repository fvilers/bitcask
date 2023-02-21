use bitcask_lib::prelude::Datastore;

pub struct Context {
    pub datastore: Datastore,
}

impl Context {
    pub fn new(datastore: Datastore) -> Self {
        Self { datastore }
    }
}
