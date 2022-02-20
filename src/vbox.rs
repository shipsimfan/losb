#[derive(Debug)]
pub enum VBoxError {
    BuildError(crate::image::BuildImageError),
    VBoxError(Option<std::io::Error>),
}

pub fn vbox() -> Result<(), VBoxError> {
    crate::image::build_image()?;

    let mut vbox_command = std::process::Command::new(crate::config::VBOX);
    vbox_command.args(crate::config::VBOX_FLAGS);
    let status = vbox_command.status()?;

    if !status.success() {
        Err(VBoxError::VBoxError(None))
    } else {
        Ok(())
    }
}

impl std::error::Error for VBoxError {}

impl std::fmt::Display for VBoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VBoxError::BuildError(error) => format!("{}", error),
                VBoxError::VBoxError(error) => format!(
                    "Unable to create VBox image{}",
                    match error {
                        Some(error) => format!(" ({})", error),
                        None => String::new(),
                    }
                ),
            }
        )
    }
}

impl From<crate::image::BuildImageError> for VBoxError {
    fn from(error: crate::image::BuildImageError) -> Self {
        VBoxError::BuildError(error)
    }
}

impl From<std::io::Error> for VBoxError {
    fn from(error: std::io::Error) -> Self {
        VBoxError::VBoxError(Some(error))
    }
}
