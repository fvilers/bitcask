use std::{env, process};

use bitcask_repl::RunConfig;

fn main() {
    // This crate will be a REPL and accept as a startup argument the directory name of the Bitcask datastore.
    // As for now, there will be no interaction with the user
    let args: Vec<String> = env::args().collect();
    let config = RunConfig::build(&args).unwrap_or_else(|error| {
        println!("{}", error);
        process::exit(1);
    });

    if let Err(error) = bitcask_repl::run(config) {
        eprintln!("{}", error);

        if let Some(source) = error.source() {
            eprintln!("  Caused by: {}", source);
        }

        process::exit(1);
    }
}
