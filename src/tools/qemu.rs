use super::ToolError;
use crate::args::Options;
use std::process::{Child, Command};

pub struct RunningEmulator {
    child: Option<Child>,
}

const PROGRAM_NAME: &'static str = "qemu-system-x86_64";

const BIOS_ARG: &'static str = "-bios";
const DRIVE_ARG: &'static str = "-drive";

const DEBUG_ARGS: &[&'static str] = &["-S", "-gdb"];

pub fn run(options: &Options) -> Result<(), ToolError> {
    let command = create_command(options);
    run_command(command)
}

pub fn debug(options: &Options) -> Result<RunningEmulator, ToolError> {
    let mut command = create_command(options);
    add_debug_arguments(&mut command, options);

    spawn_command(command).map(|child| RunningEmulator { child: Some(child) })
}

fn create_command(options: &Options) -> Command {
    let mut command = Command::new(PROGRAM_NAME);
    command
        .arg(BIOS_ARG)
        .arg(options.ovmf_location().display().to_string())
        .arg(DRIVE_ARG)
        .arg(format!(
            "format=raw,file={}",
            options.output_path().display()
        ));

    command
}

fn add_debug_arguments(command: &mut Command, options: &Options) {
    command
        .args(DEBUG_ARGS)
        .arg(format!("tcp::{}", options.debug_port()));
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

impl Drop for RunningEmulator {
    fn drop(&mut self) {
        self.child.take().map(|child| wait_for_child(child));
    }
}
