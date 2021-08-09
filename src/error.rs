#[derive(Debug)]
pub enum Error {
    NotImplemented(crate::Command),
    TooManyArguments(String),
    InvalidCommand(String),
    BuildError(std::process::ExitStatus),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::NotImplemented(command) => format!("{} has not been implemented", command),
                Error::TooManyArguments(arg0) => format!(
                    "Too many arguments\n      \x1B[1mUsage:\x1B[0m {} [command] [configuration]",
                    arg0
                ),
                Error::InvalidCommand(command) => format!("Invalid command '{}'", command),
                Error::BuildError(status) =>
                    format!("Build failed with status {}", status.code().unwrap_or(1)),
            }
        )
    }
}
