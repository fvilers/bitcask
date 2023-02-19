use std::process;

fn main() {
    // This crate will be a REPL and accept as a startup argument the directory name of the Bitcask datastore.
    // As for now, there will be no interaction with the user

    if let Err(error) = bitcask_repl::run() {
        eprintln!("{}", error);
        process::exit(1);
    }
}
