use super::ToolError;
use crate::args::Options;
use std::{
    path::Path,
    process::{Child, Command},
};

#[derive(PartialEq, Eq)]
enum CargoCommand {
    Build,
    Clean,
}

const PROGRAM_NAME: &'static str = "cargo";
const RELEASE_ARG: &'static str = "--release";

pub fn build<P: AsRef<Path>>(name: &str, path: P, options: &Options) -> Result<(), ToolError> {
    options.output().log_building(name);
    let command = create_command(path, options, CargoCommand::Build);
    run_command(command)
}

pub fn clean<P: AsRef<Path>>(name: &str, path: P, options: &Options) -> Result<(), ToolError> {
    options.output().log_cleaning(name);
    let command = create_command(path, options, CargoCommand::Clean);
    run_command(command)
}

fn create_command<P: AsRef<Path>>(
    path: P,
    options: &Options,
    cargo_command: CargoCommand,
) -> Command {
    let mut command = Command::new(PROGRAM_NAME);
    command
        .arg(cargo_command.to_string())
        .current_dir(options.path().join(path.as_ref()));

    if options.is_release() && cargo_command == CargoCommand::Build {
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

impl CargoCommand {
    pub fn to_string(&self) -> &str {
        match self {
            CargoCommand::Build => "build",
            CargoCommand::Clean => "clean",
        }
    }
}
