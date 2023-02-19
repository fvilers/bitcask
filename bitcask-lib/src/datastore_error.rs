use std::{error, fmt};

#[derive(Debug)]
pub enum DatastoreError {}

impl fmt::Display for DatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An unknown error has occurred")
    }
}

impl error::Error for DatastoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
