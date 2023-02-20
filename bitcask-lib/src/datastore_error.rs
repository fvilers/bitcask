use std::{error, fmt, io};

#[derive(Debug)]
pub enum DatastoreError {
    InputOutput(io::Error),
    ReadOnlyStore,
}

impl fmt::Display for DatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatastoreError::InputOutput(..) => write!(f, "IO Error"),
            DatastoreError::ReadOnlyStore => write!(f, "Could not write to a read only store"),
        }
    }
}

impl error::Error for DatastoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DatastoreError::InputOutput(ref e) => Some(e),
            DatastoreError::ReadOnlyStore => None,
        }
    }
}

impl From<io::Error> for DatastoreError {
    fn from(err: io::Error) -> DatastoreError {
        DatastoreError::InputOutput(err)
    }
}
