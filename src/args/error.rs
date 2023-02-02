#[derive(Debug)]
pub enum ArgumentError {
    UnknownCommand(String),
}

impl std::error::Error for ArgumentError {}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentError::UnknownCommand(command) => write!(f, "Unknown command \"{command}\""),
        }
    }
}
