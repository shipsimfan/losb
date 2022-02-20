#[derive(Debug)]
pub enum RunError {
    BuildError(crate::image::BuildImageError),
    Emulator(std::io::Error),
}

pub fn run() -> Result<(), RunError> {
    crate::image::build_image()?;

    let mut emulator_command = std::process::Command::new(crate::config::EMULATOR);
    emulator_command.args(crate::config::EMULATOR_FLAGS);
    emulator_command.stdout(std::process::Stdio::inherit());
    emulator_command.stderr(std::process::Stdio::inherit());
    emulator_command.stdin(std::process::Stdio::inherit());
    emulator_command.output()?;

    Ok(())
}

impl std::error::Error for RunError {}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RunError::BuildError(error) => format!("{}", error),
                RunError::Emulator(error) => format!("Unable to launch emulator ({})", error),
            }
        )
    }
}

impl From<crate::image::BuildImageError> for RunError {
    fn from(error: crate::image::BuildImageError) -> Self {
        RunError::BuildError(error)
    }
}

impl From<std::io::Error> for RunError {
    fn from(error: std::io::Error) -> Self {
        RunError::Emulator(error)
    }
}
