mod datastore;
mod datastore_error;
mod open_options;

pub mod prelude {
    pub use crate::open_options::*;
}

#[cfg(test)]
mod tests {}
