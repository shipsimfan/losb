mod arguments;
mod build;
mod clean;
mod command;
mod config;
mod debug;
mod help;
mod image;
mod run;
mod vbox;
mod version;

use command::Command;

#[derive(Debug)]
struct NotImplementedError(Command);

fn fatal_error(error: Box<dyn std::error::Error>) -> ! {
    println!("\x1B[31;1mFatal Error:\x1B[0m {}", error);

    std::process::exit(1);
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => fatal_error(error),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let command = arguments::parse_command_line()?;

    // Set defaults if nescessary
    let command = command.unwrap_or(config::DEFAULT_COMMAND);

    // Process command
    match command {
        Command::Build => build::build()?,
        Command::BuildImage => image::build_image()?,
        Command::BuildISO => return Err(Box::new(NotImplementedError(Command::BuildISO))),
        Command::Clean => clean::clean()?,
        Command::CleanUser => clean::clean_user()?,
        Command::Debug => debug::debug()?,
        Command::Help => help::display_help(),
        Command::Run => run::run()?,
        Command::VBox => vbox::vbox()?,
        Command::Version => version::display_version(),
    };

    Ok(())
}

impl std::error::Error for NotImplementedError {}

impl std::fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} has not been implemented", self.0)
    }
}
