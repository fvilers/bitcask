use std::{fs, path, result, time};

use crate::{active_file::ActiveFile, datastore_error::DatastoreError};

#[derive(Debug)]
pub struct Datastore {
    _active_file: Option<ActiveFile<fs::File>>,
    _directory_name: path::PathBuf,
}

pub type Result<T> = result::Result<T, DatastoreError>;

impl Datastore {
    pub fn open(directory_name: path::PathBuf, write: bool) -> Result<Self> {
        // TODO: read entries from existing files and extend the keydir hash map

        let active_file = if write {
            let file_name = format!("{}.dat", timestamp_secs());
            let path = directory_name.join(&file_name);
            let file = fs::OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(path)?;

            Some(ActiveFile::new(file, file_name))
        } else {
            None
        };

        Ok(Datastore {
            _active_file: active_file,
            _directory_name: directory_name,
        })
    }
}

fn timestamp_secs() -> u64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
