pub fn vbox() -> Result<(), Box<dyn std::error::Error>> {
    crate::image::build_image()?;

    let mut vbox_command = std::process::Command::new(crate::config::VBOX);
    vbox_command.args(crate::config::VBOX_FLAGS);
    vbox_command.stdout(std::process::Stdio::inherit());
    vbox_command.stderr(std::process::Stdio::inherit());
    vbox_command.stdin(std::process::Stdio::inherit());
    let output = vbox_command.output()?;

    if !output.status.success() {
        Err(Box::new(crate::error::Error::BuildError(output.status)))
    } else {
        Ok(())
    }
}
