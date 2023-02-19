use std::path::PathBuf;

use crate::datastore_error::DatastoreError;

#[derive(Debug)]
pub struct Datastore {
    _directory_name: PathBuf,
}

pub type Result<T> = std::result::Result<T, DatastoreError>;

impl Datastore {
    pub fn open(directory_name: PathBuf) -> Result<Self> {
        // TODO: read entries from existing files and extend the keydir hash map

        Ok(Datastore {
            _directory_name: directory_name,
        })
    }
}
