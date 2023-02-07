use crate::{
    args::Options,
    commands::{BOOTLOADER_NAME, BOOTLOADER_PATH},
    tools::{cargo, ToolError},
};

pub fn clean_bootloader(options: &Options) -> Result<(), ToolError> {
    cargo::clean(BOOTLOADER_NAME, BOOTLOADER_PATH, options)
}
