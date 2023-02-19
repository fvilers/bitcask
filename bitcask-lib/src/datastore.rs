use std::path::PathBuf;

#[derive(Debug)]
pub struct Datastore {
    _directory_name: PathBuf,
}

impl Datastore {
    pub fn open(directory_name: PathBuf) -> Self {
        // TODO: read entries from existing files and extend the keydir hash map

        Datastore {
            _directory_name: directory_name,
        }
    }
}
