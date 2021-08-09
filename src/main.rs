mod arguments;
mod build;
mod clean;
mod command;
mod config;
mod debug;
mod error;
mod help;
mod image;
mod run;
mod vbox;
mod version;

use command::Command;
use error::Error;

fn fatal_error(error: Box<dyn std::error::Error>) -> ! {
    println!("\x1B[31;1mFatal Error:\x1B[0m {}", error);

    std::process::exit(1);
}

fn main() {
    // Parse arguments
    let command = match arguments::parse_command_line() {
        Ok(arguments) => arguments,
        Err(error) => fatal_error(Box::new(error)),
    };

    // Set defaults if nescessary
    let command = command.unwrap_or(config::DEFAULT_COMMAND);

    // Process command
    match match command {
        Command::Build => build::build(),
        Command::BuildImage => image::build_image(),
        Command::BuildISO => Err(Box::new(error::Error::NotImplemented(Command::BuildISO)).into()),
        Command::Clean => clean::clean(),
        Command::CleanUser => clean::clean_user(),
        Command::Debug => debug::debug(),
        Command::Help => help::display_help(),
        Command::Run => run::run(),
        Command::VBox => vbox::vbox(),
        Command::Version => version::display_version(),
    } {
        Ok(()) => {}
        Err(error) => fatal_error(error),
    }
}
