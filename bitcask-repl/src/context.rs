use bitcask_lib::prelude::*;

pub struct Context<T: Read + Write> {
    pub datastore: T,
}

impl<T: Read + Write> Context<T> {
    pub fn new(datastore: T) -> Self {
        Self { datastore }
    }
}
