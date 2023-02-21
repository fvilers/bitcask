mod context;
mod custom_errors;
mod custom_prompt;
mod run_config;

use std::{collections::HashMap, env, process};

use bitcask_lib::prelude::*;
use context::Context;
use custom_errors::CustomError;
use custom_prompt::CustomPrompt;
use repl_rs::{Command, Convert, Parameter, Repl, Value};
use run_config::RunConfig;

fn main() -> Result<(), repl_rs::Error> {
    let args: Vec<String> = env::args().collect();
    let config = RunConfig::build(&args).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });
    let datastore = OpenOptions::new()
        .sync(true)
        .open(config.directory_name)
        .unwrap_or_else(|error| {
            eprintln!("{}", error);
            process::exit(1);
        });
    let context = Context::new(datastore);
    let mut repl = Repl::new(context)
        .with_name("bitcask")
        .with_prompt(&CustomPrompt)
        .add_command(
            Command::new("get", get)
                .with_parameter(Parameter::new("key").set_required(true)?)?
                .with_help("Retrieve a value by key from a Bitcask datastore"),
        );

    repl.run()
}

fn get(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, CustomError> {
    let key = args["key"].convert()?;
    let value = context
        .datastore
        .get(key)?
        .map(String::from_utf8)
        .transpose();

    value.map_err(Into::into)
}
