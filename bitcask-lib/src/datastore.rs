use std::{fs, path, result};

use crate::{
    active_file::ActiveFile, datastore_entry::DatastoreEntry, datastore_error::DatastoreError,
    timestamp::timestamp_secs,
};

#[derive(Debug)]
pub struct Datastore {
    active_file: Option<ActiveFile<fs::File>>,
    _directory_name: path::PathBuf,
    sync: bool,
}

pub type Result<T> = result::Result<T, DatastoreError>;

impl Datastore {
    pub fn open(directory_name: path::PathBuf, write: bool, sync: bool) -> Result<Self> {
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
            active_file,
            _directory_name: directory_name,
            sync,
        })
    }

    // TODO: implement the get() fn

    pub fn put<V: AsRef<[u8]>>(&self, key: String, value: V) -> Result<()> {
        let Some(active_file) = &self.active_file else {
            return Err(DatastoreError::ReadOnlyStore);
        };
        let datastore_entry = DatastoreEntry::new(key, value.as_ref().to_owned());
        let mut handle = &active_file.handle;

        datastore_entry.write(&mut handle)?;

        if self.sync {
            handle.sync_all()?;
        }

        // TODO: save entry to keydir hash map

        Ok(())
    }
}
