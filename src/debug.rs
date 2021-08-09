pub fn debug() -> Result<(), Box<dyn std::error::Error>> {
    crate::image::build_image()?;

    let mut emulator_command = std::process::Command::new(crate::config::EMULATOR);
    emulator_command.args(crate::config::EMULATOR_FLAGS);
    emulator_command.args(crate::config::EMULATOR_DEBUG_FLAGS);
    emulator_command.stdout(std::process::Stdio::inherit());
    emulator_command.stderr(std::process::Stdio::inherit());
    emulator_command.stdin(std::process::Stdio::inherit());
    emulator_command.spawn()?;

    let mut debugger_command = std::process::Command::new(crate::config::DEBUGGER);
    debugger_command.args(crate::config::DEBUGGER_FLAGS);
    debugger_command.stdout(std::process::Stdio::inherit());
    debugger_command.stderr(std::process::Stdio::inherit());
    debugger_command.stdin(std::process::Stdio::inherit());
    debugger_command.output()?;

    Ok(())
}
