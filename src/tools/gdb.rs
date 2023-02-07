use super::ToolError;
use crate::{args::Options, commands::common::KERNEL_SYSROOT_PATH};
use std::process::{Child, Command};

const PROGRAM_NAME: &'static str = "gdb";
const SOURCE_ARG: &'static str = "-s";
const EXECUTE_ARG: &'static str = "-ex";

pub fn run(options: &Options) -> Result<(), ToolError> {
    let command = create_command(options);
    run_command(command)
}

fn create_command(options: &Options) -> Command {
    let mut command = Command::new(PROGRAM_NAME);
    command
        .arg(SOURCE_ARG)
        .arg(options.sysroot().join(KERNEL_SYSROOT_PATH))
        .arg(EXECUTE_ARG)
        .arg(format!("target remote localhost:{}", options.debug_port()));

    println!("{:?}", command.get_args());

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
