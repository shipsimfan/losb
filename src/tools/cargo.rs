use super::ToolError;
use std::{path::Path, process::Command};

const PROGRAM_NAME: &'static str = "cargo";
const BUILD_COMMAND: &'static str = "build";

pub fn build<P: AsRef<Path>>(path: P) -> Result<(), ToolError> {
    let mut command = Command::new(PROGRAM_NAME);
    command.arg(BUILD_COMMAND).current_dir(path);

    let mut child = command
        .spawn()
        .map_err(|error| ToolError::SpawnError(PROGRAM_NAME, error))?;

    let status = child
        .wait()
        .map_err(|error| ToolError::SpawnError(PROGRAM_NAME, error))?;

    if !status.success() {
        Err(ToolError::RuntimeError(PROGRAM_NAME))
    } else {
        Ok(())
    }
}
