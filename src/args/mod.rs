use crate::output::Output;
use argparse::ArgumentParser;
use common::add_common_arguments;
use create_image::add_create_image_arguments;
use run::add_run_arguments;

mod error;
mod options;

mod common;
mod create_image;
mod run;

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
    add_common_arguments(parser);
    add_create_image_arguments(parser);
    add_run_arguments(parser);
}
