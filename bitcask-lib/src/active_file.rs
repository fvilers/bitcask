use std::io;

#[derive(Debug)]
pub struct ActiveFile<F: io::Write> {
    _handle: F,
    _file_name: String,
}

impl<T: io::Write> ActiveFile<T> {
    pub fn new(handle: T, file_name: String) -> Self {
        Self {
            _handle: handle,
            _file_name: file_name,
        }
    }
}
