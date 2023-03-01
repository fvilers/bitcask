use std::{marker, path};

use crate::datastore::{self, Datastore, Read, Write};

pub struct ReadOnlyMode;
pub struct ReadWriteMode;

#[derive(Debug)]
pub struct OpenOptions<Mode = ReadOnlyMode> {
    mode: marker::PhantomData<Mode>,
    sync: bool,
}

impl Default for OpenOptions<ReadOnlyMode> {
    fn default() -> Self {
        Self {
            mode: marker::PhantomData,
            sync: false,
        }
    }
}

impl OpenOptions<ReadOnlyMode> {
    pub fn with_write(self, sync: bool) -> OpenOptions<ReadWriteMode> {
        OpenOptions {
            mode: marker::PhantomData,
            sync,
        }
    }

    pub fn open<P: AsRef<path::Path>>(self, directory_name: P) -> datastore::Result<impl Read> {
        Datastore::open(directory_name.as_ref().to_path_buf(), false, false)
    }
}

impl OpenOptions<ReadWriteMode> {
    pub fn open<P: AsRef<path::Path>>(
        self,
        directory_name: P,
    ) -> datastore::Result<impl Read + Write> {
        Datastore::open(directory_name.as_ref().to_path_buf(), true, self.sync)
    }
}
