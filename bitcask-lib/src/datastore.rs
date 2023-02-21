use std::{
    collections, ffi, fs,
    io::{self, Seek},
    iter, ops, path, result,
};

use itertools::Itertools;

use crate::{
    active_file::ActiveFile, datastore_entry::DatastoreEntry, datastore_error::DatastoreError,
    keydir_entry::KeydirEntry, timestamp::timestamp_secs,
};

#[derive(Debug)]
pub struct Datastore {
    active_file: Option<ActiveFile<fs::File>>,
    directory_name: path::PathBuf,
    keydir_map: collections::HashMap<String, KeydirEntry>,
    sync: bool,
}

pub type Result<T> = result::Result<T, DatastoreError>;

const DATA_FILE_EXTENSION: &str = "dat";

impl Datastore {
    pub fn directory_name(&self) -> String {
        self.directory_name.to_string_lossy().to_string()
    }

    pub fn open(directory_name: path::PathBuf, write: bool, sync: bool) -> Result<Self> {
        let entries = read_keydir_entries(&directory_name)?;
        let mut keydir_map = collections::HashMap::with_capacity(entries.len());

        // TODO: remove deleted entries after reading them from files
        keydir_map.extend(entries);

        let active_file = if write {
            let file_name =
                ffi::OsString::from(format!("{}.{}", timestamp_secs(), DATA_FILE_EXTENSION));
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
            directory_name,
            keydir_map,
            sync,
        })
    }

    pub fn get(&self, key: String) -> Result<Option<Vec<u8>>> {
        let Some(keydir_entry) = self.keydir_map.get(&key) else {
            return Ok(None);
        };
        let datastore_entry = self.read_entry(keydir_entry)?;

        Ok(Some(datastore_entry.value))
    }

    pub fn put<V: AsRef<[u8]>>(&mut self, key: String, value: V) -> Result<()> {
        let Some(active_file) = &self.active_file else {
            return Err(DatastoreError::ReadOnlyStore);
        };
        let file_name = active_file.file_name.to_owned();
        let (datastore_entry, position) = &self.write_entry(key.to_owned(), value)?;
        let keydir_entry = KeydirEntry::new(
            file_name,
            datastore_entry.value_size,
            *position,
            datastore_entry.timestamp,
        );
        self.keydir_map.insert(key, keydir_entry);

        Ok(())
    }

    pub fn delete(&mut self, key: String) -> Result<()> {
        self.write_entry(key.to_owned(), b"")?;
        self.keydir_map.remove_entry(&key);

        Ok(())
    }

    pub fn list_keys(&self) -> impl iter::Iterator<Item = &String> + '_ {
        self.keydir_map.keys()
    }

    pub fn fold<B, F>(&self, init: B, f: F) -> Result<B>
    where
        F: ops::Fn(B, String, Vec<u8>) -> B,
    {
        self.keydir_map
            .values()
            .map(|keydir_entry| self.read_entry(keydir_entry))
            .fold_ok(init, |acc, datastore_entry| {
                f(acc, datastore_entry.key, datastore_entry.value)
            })
    }

    pub fn sync(&self) -> Result<()> {
        let Some(active_file) = &self.active_file else {
            return Err(DatastoreError::ReadOnlyStore);
        };

        active_file.handle.sync_data()?;

        Ok(())
    }

    fn read_entry(&self, keydir_entry: &KeydirEntry) -> Result<DatastoreEntry> {
        let path = self.directory_name.join(&keydir_entry.file_name);
        let mut file = fs::OpenOptions::new().read(true).open(path)?;

        // Force the a disk write if the datastore is open in write mode and the file we need to read the datastore
        // entry from is the same than the active one and if sync is not enabled.
        if let Some(active_file) = &self.active_file {
            if keydir_entry.file_name == active_file.file_name && !self.sync {
                active_file.handle.sync_data()?;
            }
        };

        file.seek(io::SeekFrom::Start(keydir_entry.entry_position))?;

        let datastore_entry = DatastoreEntry::read(&mut file)?;

        if keydir_entry.timestamp != datastore_entry.timestamp {
            return Err(DatastoreError::TimestampMismatch);
        }

        Ok(datastore_entry)
    }

    fn write_entry<V: AsRef<[u8]>>(
        &mut self,
        key: String,
        value: V,
    ) -> Result<(DatastoreEntry, u64)> {
        let Some(active_file) = &self.active_file else {
            return Err(DatastoreError::ReadOnlyStore);
        };
        let datastore_entry = DatastoreEntry::new(key, value.as_ref().to_owned());
        let mut handle = &active_file.handle;
        let position = handle.stream_position()?;

        datastore_entry.write(&mut handle)?;

        if self.sync {
            handle.sync_data()?;
        }

        Ok((datastore_entry, position))
    }
}

fn read_keydir_entries(directory_name: &path::Path) -> Result<Vec<(String, KeydirEntry)>> {
    let mut keydir_entries: Vec<(String, KeydirEntry)> = Vec::new();
    let mut dir_entries: Vec<_> = fs::read_dir(directory_name)?
        .map(|entry| entry.unwrap())
        .collect();

    dir_entries.sort_by_key(|dir| dir.path());

    for path in dir_entries
        .iter()
        .map(|dir_entry| dir_entry.path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().unwrap_or_default() == DATA_FILE_EXTENSION)
    {
        if let Some(file_name) = path.file_name() {
            let data = fs::read(&path)?;
            let mut reader = io::Cursor::new(data);
            let mut position = 0;

            while let Ok(entry) = DatastoreEntry::read(&mut reader) {
                // Check for deleted entry
                if entry.value_size == 0 {
                    continue;
                }

                let keydir_entry = KeydirEntry::new(
                    file_name.to_owned(),
                    entry.value_size,
                    position,
                    entry.timestamp,
                );

                position = reader.position();
                keydir_entries.push((entry.key, keydir_entry));
            }
        }
    }

    Ok(keydir_entries)
}
