use std::fmt::Debug;

#[derive(Debug)]
pub enum DebugError {
    BuildError(crate::image::BuildImageError),
    Emulator(std::io::Error),
    Debugger(std::io::Error),
}

pub fn debug() -> Result<(), DebugError> {
    crate::image::build_image()?;

    let mut emulator_command = std::process::Command::new(crate::config::EMULATOR);
    emulator_command.args(crate::config::EMULATOR_FLAGS);
    emulator_command.args(crate::config::EMULATOR_DEBUG_FLAGS);
    emulator_command.stdout(std::process::Stdio::inherit());
    emulator_command.stderr(std::process::Stdio::inherit());
    emulator_command.stdin(std::process::Stdio::inherit());
    let mut emulator = match emulator_command.spawn() {
        Ok(child) => child,
        Err(error) => return Err(DebugError::Emulator(error)),
    };

    let mut debugger_command = std::process::Command::new(crate::config::DEBUGGER);
    debugger_command.args(crate::config::DEBUGGER_FLAGS);
    match debugger_command.status() {
        Ok(_) => Ok(()),
        Err(error) => {
            emulator.kill().ok(); // Doesn't matter if it actually exits properly
            Err(DebugError::Debugger(error))
        }
    }
}

impl std::error::Error for DebugError {}

impl std::fmt::Display for DebugError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DebugError::BuildError(error) => format!("{}", error),
                DebugError::Emulator(error) => format!("Unable to launch emulator ({})", error),
                DebugError::Debugger(error) => format!("Unable to launch debugger ({})", error),
            }
        )
    }
}

impl From<crate::image::BuildImageError> for DebugError {
    fn from(error: crate::image::BuildImageError) -> Self {
        DebugError::BuildError(error)
    }
}
