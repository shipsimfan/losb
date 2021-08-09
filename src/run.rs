pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    crate::image::build_image()?;

    let mut emulator_command = std::process::Command::new(crate::config::EMULATOR);
    emulator_command.args(crate::config::EMULATOR_FLAGS);
    emulator_command.stdout(std::process::Stdio::inherit());
    emulator_command.stderr(std::process::Stdio::inherit());
    emulator_command.stdin(std::process::Stdio::inherit());
    emulator_command.output()?;

    Ok(())
}
