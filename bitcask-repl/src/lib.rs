use std::error::Error;

use bitcask_lib::prelude::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let datastore = OpenOptions::new()
        .write(true)
        .sync(true)
        .open("c:\\temp\\datastore")?;

    println!("{:?}", datastore);

    Ok(())
}
