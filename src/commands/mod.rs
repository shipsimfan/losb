use crate::args::ArgumentError;

#[derive(Clone, Copy)]
pub enum Command {
    // Build Commands
    BuildAll,
    BuildBootloader,
    BuildKernel,

    // Create Image Commands
    CreateIMG,
    CreateISO,

    // Execute Commands
    Run,
    Debug,

    // Clean Commands
    CleanAll,
    CleanBootloader,
    CleanKernel,
}

impl Command {
    pub fn parse(command: &str) -> Result<Self, ArgumentError> {
        Ok(match command {
            // Build Commands
            "build" | "build-all" => Command::BuildAll,
            "build-boot" | "build-bootloader" => Command::BuildBootloader,
            "build-kernel" => Command::BuildKernel,

            // Create Image Commands
            "img" | "create-img" => Command::CreateIMG,
            "iso" | "create-iso" => Command::CreateISO,

            // Execute Commands
            "run" => Command::Run,
            "debug" => Command::Debug,

            // Clean Commands
            "clean" | "clean-all" => Command::CleanAll,
            "clean-boot" | "clean-bootloader" => Command::CleanBootloader,
            "clean-kernel" => Command::CleanKernel,
            _ => return Err(ArgumentError::UnknownCommand(command.to_owned())),
        })
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::BuildAll
    }
}
