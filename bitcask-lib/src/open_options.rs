use std::path::Path;

use crate::datastore::{self, Datastore};

#[derive(Debug, Default)]
pub struct OpenOptions {
    write: bool,
    sync: bool,
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions::default()
    }

    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    pub fn sync(&mut self, sync: bool) -> &mut Self {
        self.write = self.write || sync;
        self.sync = sync;
        self
    }

    pub fn open<P: AsRef<Path>>(&self, directory_name: P) -> datastore::Result<Datastore> {
        Datastore::open(directory_name.as_ref().to_path_buf())
    }
}
