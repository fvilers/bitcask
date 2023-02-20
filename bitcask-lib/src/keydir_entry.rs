use std::ffi;

#[derive(Debug)]
pub struct KeydirEntry {
    pub file_name: ffi::OsString,
    _value_size: u32,
    pub entry_position: u64,
    pub timestamp: u64,
}

impl KeydirEntry {
    pub fn new(
        file_name: ffi::OsString,
        value_size: u32,
        entry_position: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            file_name,
            _value_size: value_size,
            entry_position,
            timestamp,
        }
    }
}
