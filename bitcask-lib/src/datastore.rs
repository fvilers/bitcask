use std::{collections, ffi, fs, io::Seek, path, result};

use crate::{
    active_file::ActiveFile, datastore_entry::DatastoreEntry, datastore_error::DatastoreError,
    keydir_entry::KeydirEntry, timestamp::timestamp_secs,
};

#[derive(Debug)]
pub struct Datastore {
    active_file: Option<ActiveFile<fs::File>>,
    _directory_name: path::PathBuf,
    keydir_map: collections::HashMap<String, KeydirEntry>,
    sync: bool,
}

pub type Result<T> = result::Result<T, DatastoreError>;

impl Datastore {
    pub fn open(directory_name: path::PathBuf, write: bool, sync: bool) -> Result<Self> {
        let entries = read_keydir_entries(&directory_name)?;
        let mut keydir_map = collections::HashMap::with_capacity(entries.len());

        keydir_map.extend(entries);

        let active_file = if write {
            let file_name = ffi::OsString::from(format!("{}.dat", timestamp_secs()));
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
            keydir_map,
            sync,
        })
    }

    // TODO: implement the get() fn

    pub fn put<V: AsRef<[u8]>>(&mut self, key: String, value: V) -> Result<()> {
        let Some(active_file) = &self.active_file else {
            return Err(DatastoreError::ReadOnlyStore);
        };
        let datastore_entry = DatastoreEntry::new(key.to_owned(), value.as_ref().to_owned());
        let mut handle = &active_file.handle;
        let position = handle.stream_position()?;

        datastore_entry.write(&mut handle)?;

        if self.sync {
            handle.sync_all()?;
        }

        let keydir_entry = KeydirEntry::new(
            active_file.file_name.to_owned(),
            datastore_entry.value_size,
            position,
            datastore_entry.timestamp,
        );
        self.keydir_map.insert(key, keydir_entry);

        Ok(())
    }
}

fn read_keydir_entries(_directory_name: &path::Path) -> Result<Vec<(String, KeydirEntry)>> {
    // TODO: read entries from existing files and extend the keydir hash map
    let keydir_entries: Vec<(String, KeydirEntry)> = Vec::new();

    Ok(keydir_entries)
}
