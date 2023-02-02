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
        .required(true)
        .count(1);

    // Parse arguments
    parser
        .parse_args_env(Options::default())
        .map_err(|error| error.into())
}
