use crate::args::{ArgumentError, Options};

mod build;
mod clean;
mod create_image;
mod install;
mod run;

pub mod common;

pub(self) use common::*;

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

    // Execute Commands
    Run,
    Debug,

    // Clean Commands
    CleanAll,
    CleanSysroot,
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

            // Execute Commands
            "run" => Command::Run,
            "debug" => Command::Debug,

            // Clean Commands
            "clean" | "clean-all" => Command::CleanAll,
            "clean-sysroot" => Command::CleanSysroot,
            "clean-boot" | "clean-bootloader" => Command::CleanBootloader,
            "clean-kernel" => Command::CleanKernel,
            _ => return Err(ArgumentError::UnknownCommand(command.to_owned())),
        })
    }

    pub fn execute(&self, options: &Options) -> Result<(), Box<dyn std::error::Error>> {
        Ok(match self {
            // Build Commands
            Command::BuildAll => build::build_all(options)?,
            Command::BuildBootloader => build::build_bootloader(options)?,
            Command::BuildKernel => build::build_kernel(options)?,

            // Install Commands
            Command::InstallAll => install::install_all(options)?,
            Command::InstallBootloader => install::install_bootloader(options)?,
            Command::InstallKernel => install::install_kernel(options)?,

            // Create Image Commands
            Command::CreateIMG => create_image::create_image(options)?,

            // Run Commands
            Command::Run => run::run(options)?,
            Command::Debug => run::debug(options)?,

            // Clean Commands
            Command::CleanAll => clean::clean_all(options)?,
            Command::CleanSysroot => clean::clean_sysroot(options)?,
            Command::CleanBootloader => clean::clean_bootloader(options)?,
            Command::CleanKernel => clean::clean_kernel(options)?,
        })
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::BuildAll
    }
}
