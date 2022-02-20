#[derive(Debug)]
pub enum ArgumentParseError {
    TooManyArguments(String),
    InvalidCommand(crate::command::InvalidCommand),
}

pub fn parse_command_line() -> Result<Option<crate::Command>, ArgumentParseError> {
    parse(std::env::args().collect())
}

fn parse(arguments: Vec<String>) -> Result<Option<crate::Command>, ArgumentParseError> {
    match arguments.len() {
        1 => Ok(None),
        2 => Ok(Some(crate::Command::parse(&arguments[1])?)),
        _ => Err(ArgumentParseError::TooManyArguments(
            arguments[0].to_string(),
        )),
    }
}

impl std::error::Error for ArgumentParseError {}

impl std::fmt::Display for ArgumentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArgumentParseError::InvalidCommand(error) => format!("{}", error),
                ArgumentParseError::TooManyArguments(arg0) => format!(
                    "Too many arguments\n      \x1B[1mUsage:\x1B[0m {} [command] [configuration]",
                    arg0
                ),
            }
        )
    }
}

impl From<crate::command::InvalidCommand> for ArgumentParseError {
    fn from(error: crate::command::InvalidCommand) -> Self {
        ArgumentParseError::InvalidCommand(error)
    }
}
