mod arguments;
mod command;
mod config;
mod error;

use command::Command;
use error::Error;

fn fatal_error(error: Box<dyn std::error::Error>) -> ! {
    println!("\x1B[31;1mFatal Error:\x1B[0m {}", error);

    std::process::exit(1);
}

fn main() {
    // Parse arguments
    let (command, configuration) = match arguments::parse_command_line() {
        Ok(arguments) => arguments,
        Err(error) => fatal_error(Box::new(error)),
    };

    // Set defaults if nescessary
    let command = command.unwrap_or(config::DEFAULT_COMMAND);
    let configuration = configuration.unwrap_or(config::DEFAULT_CONFIGURATION.to_string());

    // Process command
    match command.perform(&configuration) {
        Ok(()) => {}
        Err(error) => fatal_error(error),
    }
}
