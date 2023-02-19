use bitcask_lib::prelude::*;

fn main() {
    // This crate will be a REPL and accept as a startup argument the directory name of the Bitcask datastore.
    // As for now, there will be no interaction with the user

    let mut options = OpenOptions::new();
    let options = options.write(true).sync(true);

    println!("{:?}", options);
}
