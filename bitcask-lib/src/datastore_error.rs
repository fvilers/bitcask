use std::{error, fmt, io};

#[derive(Debug)]
pub enum DatastoreError {
    InputOutput(io::Error),
}

impl fmt::Display for DatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatastoreError::InputOutput(..) => write!(f, "IO Error"),
        }
    }
}

impl error::Error for DatastoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DatastoreError::InputOutput(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for DatastoreError {
    fn from(err: io::Error) -> DatastoreError {
        DatastoreError::InputOutput(err)
    }
}
