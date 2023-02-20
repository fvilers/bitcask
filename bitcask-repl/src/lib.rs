use rand::distributions::{Alphanumeric, DistString};
use std::{error::Error, str};

use bitcask_lib::prelude::*;

pub struct RunConfig {
    pub directory_name: String,
}

impl RunConfig {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Usage: bitcask-repl <directory_name>");
        }

        let directory_name = args[1].to_owned();

        Ok(Self { directory_name })
    }
}

pub fn run(config: RunConfig) -> Result<(), Box<dyn Error>> {
    let mut datastore = OpenOptions::new()
        .write(true)
        .sync(true)
        .open(config.directory_name)?;
    let key = String::from("my-key");

    // Trying to get a value from a previous put()
    if let Some(value) = datastore.get(key.to_owned())? {
        let value = str::from_utf8(&value)?;
        println!("Got previous value for {} -> {}", key, value);
    }

    let value = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    datastore.put(key.to_owned(), value)?;

    if let Some(value) = datastore.get(key.to_owned())? {
        let value = str::from_utf8(&value)?;
        println!("Got value for {} -> {}", key, value);
    }

    let key = String::from("deleted-key");
    let value = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    datastore.put(key.to_owned(), value)?;

    println!("List of all keys");
    for key in datastore.list_keys() {
        println!("{}", key);
    }

    println!("Folding...");
    let fold_result = datastore.fold(String::new(), |acc, key, value| {
        format!("{}\n{}: {}", acc, key, str::from_utf8(&value).unwrap())
    })?;
    println!("{}", fold_result);

    println!("Deleting {}", key);
    datastore.delete(key.to_owned())?;

    if datastore.get(key.to_owned())?.is_none() {
        println!("Value for {} could not be found", key);
    }

    Ok(())
}
