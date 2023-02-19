use std::error::Error;

use bitcask_lib::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // This crate will be a REPL and accept as a startup argument the directory name of the Bitcask datastore.
    // As for now, there will be no interaction with the user

    let datastore = OpenOptions::new()
        .write(true)
        .sync(true)
        .open("c:\\temp\\datastore")?;

    println!("{:?}", datastore);

    Ok(())
}
