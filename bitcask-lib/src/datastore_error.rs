use std::{error, fmt, io, string};

#[derive(Debug)]
pub enum DatastoreError {
    ReadOnlyStore,
    CrcMismatch,
    TimestampMismatch,
    InputOutput(io::Error),
    Utf8(string::FromUtf8Error),
}

impl fmt::Display for DatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatastoreError::ReadOnlyStore => write!(f, "Could not write to a read only store"),
            DatastoreError::CrcMismatch => write!(
                f,
                "There's a CRC mismatch between the Keydir entry and the Datastore entry"
            ),
            DatastoreError::TimestampMismatch => write!(
                f,
                "There's a timestamp mismatch between the Keydir entry and the Datastore entry"
            ),
            DatastoreError::InputOutput(e) => e.fmt(f),
            DatastoreError::Utf8(e) => e.fmt(f),
        }
    }
}

impl error::Error for DatastoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DatastoreError::ReadOnlyStore => None,
            DatastoreError::CrcMismatch => None,
            DatastoreError::TimestampMismatch => None,
            DatastoreError::InputOutput(ref e) => Some(e),
            DatastoreError::Utf8(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for DatastoreError {
    fn from(err: io::Error) -> DatastoreError {
        DatastoreError::InputOutput(err)
    }
}

impl From<string::FromUtf8Error> for DatastoreError {
    fn from(err: string::FromUtf8Error) -> DatastoreError {
        DatastoreError::Utf8(err)
    }
}
