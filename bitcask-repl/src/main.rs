mod context;
mod custom_prompt;
mod run_config;

use std::{env, error::Error, process};

use bitcask_lib::prelude::*;
use context::Context;
use custom_prompt::CustomPrompt;
use repl_builder::prelude::*;
use run_config::RunConfig;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = RunConfig::build(&args).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });
    let datastore = OpenOptions::default()
        .with_write(true)
        .open(config.directory_name)
        .unwrap_or_else(|error| {
            eprintln!("{}", error);
            process::exit(1);
        });
    let context = Context::new(datastore);
    let mut repl = ReplBuilder::new(context)
        .with_prompt(&CustomPrompt)
        .add_command(Command::new("get", get))
        .add_command(Command::new("keys", list_keys))
        .add_command(Command::new("put", put))
        .add_command(Command::new("delete", delete))
        .build();

    if let Err(e) = repl.run() {
        eprintln!("{}", e);

        if let Some(source) = e.source() {
            eprintln!("Source: {}", source);
        }

        process::exit(1);
    }
}

fn get<T: Read + Write>(args: Vec<&str>, context: &mut Context<T>) -> CommandResult {
    let Some(key) = args.first() else {
        return Err(ReplError::MissingArgument("name".into()));
    };

    let Ok(result) = context.datastore.get(*key) else {
        return Err(ReplError::Execution("Error while getting value from the datastore".into()))
    };

    let Some(buffer) = result else {
        return Ok(None)
    };

    let Ok(value) = String::from_utf8(buffer) else {
        return Err(ReplError::Execution("Error while converting value to UTF-8".into()));
    };

    Ok(Some(value))
}

fn list_keys<T: Read + Write>(_args: Vec<&str>, context: &mut Context<T>) -> CommandResult {
    let keys = context
        .datastore
        .keys()
        .map(|k| k.to_owned())
        .collect::<Vec<_>>()
        .join(", ");

    Ok(Some(keys))
}

fn put<T: Read + Write>(args: Vec<&str>, context: &mut Context<T>) -> CommandResult {
    let Some(key) = args.first() else {
        return Err(ReplError::MissingArgument("name".into()));
    };
    let Some(value) = args.get(1) else {
        return Err(ReplError::MissingArgument("value".into()));
    };

    let Ok(_) = context.datastore.put(*key, value) else {
        return Err(ReplError::Execution("Error while putting value to the datastore".into()));
    };

    Ok(None)
}

fn delete<T: Read + Write>(args: Vec<&str>, context: &mut Context<T>) -> CommandResult {
    let Some(key) = args.first() else {
        return Err(ReplError::MissingArgument("name".into()));
    };

    let Ok(_) = context.datastore.delete(*key) else {
        return Err(ReplError::Execution("Error while deleting value from the datastore".into()));
    };

    Ok(None)
}
