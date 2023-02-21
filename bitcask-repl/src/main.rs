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
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_description(env!("CARGO_PKG_DESCRIPTION"))
        .with_prompt(&CustomPrompt)
        .add_command(
            Command::new("get", get)
                .with_parameter(Parameter::new("key").set_required(true)?)?
                .with_help("Retrieve a value by key from the datastore"),
        )
        .add_command(
            Command::new("put", put)
                .with_parameter(Parameter::new("key").set_required(true)?)?
                .with_parameter(Parameter::new("value").set_required(true)?)?
                .with_help("Store a key and value in the datastore"),
        )
        .add_command(
            Command::new("delete", delete)
                .with_parameter(Parameter::new("key").set_required(true)?)?
                .with_help("Delete a key from the datastore"),
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

fn put(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>, CustomError> {
    let key = args["key"].convert()?;
    let value: String = args["value"].convert()?;

    context.datastore.put(key, value)?;

    Ok(None)
}

fn delete(
    args: HashMap<String, Value>,
    context: &mut Context,
) -> Result<Option<String>, CustomError> {
    let key = args["key"].convert()?;

    context.datastore.delete(key)?;

    Ok(None)
}
