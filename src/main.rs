#![feature(int_roundings)]

use output::Output;

mod args;
mod commands;
mod output;
mod tools;

fn run(output: &Output) -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let options = args::parse_args(output)?;

    // Execute command
    Ok(options.command().execute(&options)?)
}

fn main() {
    let output = Output::new();

    match run(&output) {
        Ok(()) => {}
        Err(error) => {
            output.log_error(error.as_ref());
            std::process::exit(1);
        }
    }
}
