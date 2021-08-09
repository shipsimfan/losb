#[derive(Debug)]
pub enum Command {
    Build,
    BuildImage,
    BuildISO,
    Clean,
    CleanUser,
    Debug,
    Help,
    Run,
    VBox,
    Version,
}

impl Command {
    pub fn parse(command: &str) -> Result<Self, crate::Error> {
        match command.to_lowercase().as_str() {
            "build" => Ok(Command::Build),
            "build-image" => Ok(Command::BuildImage),
            "build-iso" => Ok(Command::BuildISO),
            "clean" => Ok(Command::Clean),
            "clean-user" => Ok(Command::CleanUser),
            "debug" => Ok(Command::Debug),
            "help" => Ok(Command::Help),
            "run" => Ok(Command::Run),
            "vbox" => Ok(Command::VBox),
            "version" => Ok(Command::Version),
            _ => Err(crate::Error::InvalidCommand(command.to_string())),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Build => "build",
                Command::BuildImage => "build-image",
                Command::BuildISO => "build-iso",
                Command::Clean => "clean",
                Command::CleanUser => "clean-user",
                Command::Debug => "debug",
                Command::Help => "help",
                Command::Run => "run",
                Command::VBox => "vbox",
                Command::Version => "version",
            }
        )
    }
}
