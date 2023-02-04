use crate::{
    args::{ArgumentError, Options},
    output::Output,
};

mod build;
mod create_image;
mod install;
mod names;

#[derive(Clone, Copy)]
pub enum Command {
    // Build Commands
    BuildAll,
    BuildBootloader,
    BuildKernel,

    // Install Commands
    InstallAll,
    InstallBootloader,
    InstallKernel,

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

            // Install Commands
            "install" | "install-all" => Command::InstallAll,
            "install-boot" | "install-bootloader" => Command::InstallBootloader,
            "install-kernel" => Command::InstallKernel,

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

    pub fn execute(
        &self,
        options: &Options,
        output: &Output,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(match self {
            // Build Commands
            Command::BuildAll => build::build_all(options, output)?,
            Command::BuildBootloader => build::build_bootloader(options, output)?,
            Command::BuildKernel => build::build_kernel(options, output)?,

            // Install Commands
            Command::InstallAll => install::install_all(options, output)?,
            Command::InstallBootloader => install::install_bootloader(options, output)?,
            Command::InstallKernel => install::install_kernel(options, output)?,

            // Create Image Commands
            Command::CreateIMG => create_image::create_image(options, output)?,

            _ => panic!("TODO: Implement"),
        })
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::BuildAll
    }
}
