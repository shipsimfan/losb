use super::{ArgumentError, Options};
use argparse::ArgumentParser;
use std::path::PathBuf;

pub fn add_run_arguments(parser: &mut ArgumentParser<Options>) {
    add_ovmf_argument(parser);
    add_debug_port_argument(parser);
    add_no_gdb_argument(parser);
}

fn add_ovmf_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--ovmf", |args, options| {
            Ok(options.set_ovmf_location(PathBuf::from(&args[0])))
        })
        .help("Sets the location of the OVMF BIOS for QEMU")
        .required(false)
        .count(1);
}

fn add_debug_port_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--debug-port", |args, options| match args[0].parse() {
            Ok(debug_port) => Ok(options.set_debug_port(debug_port)),
            Err(_) => Err(Box::new(ArgumentError::InvalidDebugPort(args[0].clone()))),
        })
        .help("Sets the port for connecting a debugger to QEMU")
        .required(false)
        .count(1);
}

fn add_no_gdb_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--no-gdb", |_, options| Ok(options.set_no_gdb()))
        .help("Disables launching gdb in debug mode")
        .required(false)
        .count(0);
}
