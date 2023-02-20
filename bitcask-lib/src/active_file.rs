use std::{ffi, io};

#[derive(Debug)]
pub struct ActiveFile<F: io::Write> {
    pub handle: F,
    pub file_name: ffi::OsString,
}

impl<T: io::Write> ActiveFile<T> {
    pub fn new(handle: T, file_name: ffi::OsString) -> Self {
        Self { handle, file_name }
    }
}
