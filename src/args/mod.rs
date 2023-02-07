use std::path::PathBuf;

use crate::{commands::Command, output::Output};
use argparse::ArgumentParser;

mod error;
mod options;

pub use error::*;
pub use options::*;

pub fn parse_args<'a>(output: &'a Output) -> Result<Options<'a>, Box<dyn std::error::Error>> {
    let mut parser = create_parser::<'a>();

    add_arguments(&mut parser);

    parser
        .parse_args_env(Options::new(output))
        .map_err(|error| error.into())
}

fn create_parser<'a>() -> ArgumentParser<Options<'a>> {
    let mut parser = ArgumentParser::<Options>::new();
    parser
        .program_name("losb")
        .description("The build utility for LOS")
        .help(true);
    parser
}

fn add_arguments(parser: &mut ArgumentParser<Options>) {
    add_command_argument(parser);
    add_path_argument(parser);
    add_sysroot_argument(parser);
    add_debug_argument(parser);
    add_release_argument(parser);
}

fn add_command_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("COMMAND", |args, options| {
            Ok(options.set_command(Command::parse(&args[0])?))
        })
        .help("The command to execute")
        .required(false)
        .count(1);
}

fn add_path_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--path", |args, options| {
            Ok(options.set_path(PathBuf::from(&args[0])))
        })
        .name("-p")
        .help("Sets the base path to build from")
        .required(false)
        .count(1);
}

fn add_sysroot_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--sysroot", |args, options| {
            Ok(options.set_sysroot(PathBuf::from(&args[0])))
        })
        .name("-s")
        .help("Sets the system root to install to and create images from")
        .required(false)
        .count(1);
}

fn add_debug_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--debug", |_, options| Ok(options.set_debug()))
        .help("Uses debug builds for the command")
        .required(false)
        .count(0);
}

fn add_release_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--release", |_, options| Ok(options.set_release()))
        .help("Uses release builds for the command")
        .required(false)
        .count(0);
}
