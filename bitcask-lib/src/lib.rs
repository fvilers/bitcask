mod active_file;
mod datastore;
mod datastore_entry;
mod datastore_error;
mod keydir_entry;
mod open_options;
mod timestamp;

pub mod prelude {
    pub use crate::datastore::Datastore;
    pub use crate::datastore_error::DatastoreError;
    pub use crate::open_options::OpenOptions;
}

#[cfg(test)]
mod tests {}
