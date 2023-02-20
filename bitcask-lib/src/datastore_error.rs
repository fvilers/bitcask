use std::{error, fmt, io, str};

#[derive(Debug)]
pub enum DatastoreError {
    InputOutput(io::Error),
    ReadOnlyStore,
    StringConversion(str::Utf8Error),
    CrcMismatch,
    TimestampMismatch,
}

impl fmt::Display for DatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatastoreError::InputOutput(..) => write!(f, "IO Error"),
            DatastoreError::ReadOnlyStore => write!(f, "Could not write to a read only store"),
            DatastoreError::StringConversion(..) => {
                write!(f, "Could not convert a byte array to an UTF8 string")
            }
            DatastoreError::CrcMismatch => write!(
                f,
                "There's a CRC mismatch between the Keydir entry and the Datastore entry"
            ),
            DatastoreError::TimestampMismatch => write!(
                f,
                "There's a timestamp mismatch between the Keydir entry and the Datastore entry"
            ),
        }
    }
}

impl error::Error for DatastoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DatastoreError::InputOutput(ref e) => Some(e),
            DatastoreError::ReadOnlyStore => None,
            DatastoreError::StringConversion(ref e) => Some(e),
            DatastoreError::CrcMismatch => None,
            DatastoreError::TimestampMismatch => None,
        }
    }
}

impl From<io::Error> for DatastoreError {
    fn from(err: io::Error) -> DatastoreError {
        DatastoreError::InputOutput(err)
    }
}

impl From<str::Utf8Error> for DatastoreError {
    fn from(err: str::Utf8Error) -> DatastoreError {
        DatastoreError::StringConversion(err)
    }
}
