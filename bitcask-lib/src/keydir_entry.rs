use std::ffi;

#[derive(Debug)]
pub struct KeydirEntry {
    _file_name: ffi::OsString,
    _value_size: u32,
    _entry_position: u64,
    _timestamp: u64,
}

impl KeydirEntry {
    pub fn new(
        file_name: ffi::OsString,
        value_size: u32,
        entry_position: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            _file_name: file_name,
            _value_size: value_size,
            _entry_position: entry_position,
            _timestamp: timestamp,
        }
    }
}
