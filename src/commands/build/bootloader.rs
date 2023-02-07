use crate::{
    args::Options,
    commands::{BOOTLOADER_NAME, BOOTLOADER_PATH},
    tools::{cargo, ToolError},
};

pub fn build_bootloader(options: &Options) -> Result<(), ToolError> {
    cargo::build(BOOTLOADER_NAME, BOOTLOADER_PATH, options)
}
