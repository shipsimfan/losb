use crate::{
    args::Options,
    commands::names::BOOTLOADER_NAME,
    output::Output,
    tools::{cargo, ToolError},
};

const BOOTLOADER_PATH: &'static str = "bootloader";

pub fn build_bootloader(options: &Options, output: &Output) -> Result<(), ToolError> {
    cargo::build(BOOTLOADER_NAME, BOOTLOADER_PATH, options, output)
}
