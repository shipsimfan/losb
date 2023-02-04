use super::ToolError;
use crate::{args::Options, output::Output};
use std::{
    path::Path,
    process::{Child, Command},
};

const PROGRAM_NAME: &'static str = "cargo";
const BUILD_COMMAND: &'static str = "build";
const RELEASE_ARG: &'static str = "--release";

pub fn build<P: AsRef<Path>>(
    name: &str,
    path: P,
    options: &Options,
    output: &Output,
) -> Result<(), ToolError> {
    output.log_building(name);
    let command = create_command(path, options);
    run_command(command)
}

fn create_command<P: AsRef<Path>>(path: P, options: &Options) -> Command {
    let mut command = Command::new(PROGRAM_NAME);
    command
        .arg(BUILD_COMMAND)
        .current_dir(options.path().join(path.as_ref()));

    if options.is_release() {
        command.arg(RELEASE_ARG);
    }

    command
}

fn run_command(command: Command) -> Result<(), ToolError> {
    let child = spawn_command(command)?;
    wait_for_child(child)
}

fn spawn_command(mut command: Command) -> Result<Child, ToolError> {
    command
        .spawn()
        .map_err(|error| ToolError::SpawnError(PROGRAM_NAME, error))
}

fn wait_for_child(mut child: Child) -> Result<(), ToolError> {
    let status = child
        .wait()
        .map_err(|error| ToolError::SpawnError(PROGRAM_NAME, error))?;

    if !status.success() {
        Err(ToolError::RuntimeError(PROGRAM_NAME))
    } else {
        Ok(())
    }
}
