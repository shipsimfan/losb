use std::path::PathBuf;

use crate::commands::Command;
use argparse::ArgumentParser;

mod error;
mod options;

pub use error::*;
pub use options::*;

pub fn parse_args() -> Result<Options, Box<dyn std::error::Error>> {
    // Create parser
    let mut parser = ArgumentParser::<Options>::new();
    parser
        .program_name("losb")
        .description("The build utility for LOS")
        .help(true);

    // Add arguments
    parser
        .add_argument("COMMAND", |args, options| {
            Ok(options.set_command(Command::parse(&args[0])?))
        })
        .help("The command to execute")
        .required(false)
        .count(1);
    parser
        .add_argument("--path", |args, options| {
            Ok(options.set_path(PathBuf::from(&args[0])))
        })
        .name("-p")
        .help("Sets the base path to build from")
        .required(false)
        .count(1);

    // Parse arguments
    parser
        .parse_args_env(Options::default())
        .map_err(|error| error.into())
}
