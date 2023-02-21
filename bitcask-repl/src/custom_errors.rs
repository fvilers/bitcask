use std::{fmt, string};

use bitcask_lib::prelude::DatastoreError;

#[derive(Debug)]
pub enum CustomError {
    Repl(repl_rs::Error),
    Datastore(DatastoreError),
    Utf8(string::FromUtf8Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Repl(e) => write!(f, "REPL error: {}", e),
            CustomError::Datastore(e) => write!(f, "Datastore error: {}", e),
            CustomError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
        }
    }
}

impl std::error::Error for CustomError {}

impl From<repl_rs::Error> for CustomError {
    fn from(e: repl_rs::Error) -> Self {
        CustomError::Repl(e)
    }
}

impl From<DatastoreError> for CustomError {
    fn from(e: DatastoreError) -> Self {
        CustomError::Datastore(e)
    }
}

impl From<string::FromUtf8Error> for CustomError {
    fn from(err: string::FromUtf8Error) -> CustomError {
        CustomError::Utf8(err)
    }
}
