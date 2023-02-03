use crate::{
    args::Options,
    output::Output,
    tools::{cargo, ToolError},
};

const BOOTLOADER_PATH: &'static str = "bootloader";
const BOOTLOADER_NAME: &'static str = "bootloader";

pub fn build_bootloader(options: &Options, output: &Output) -> Result<(), ToolError> {
    output.log_building(BOOTLOADER_NAME);
    cargo::build(options.path().join(BOOTLOADER_PATH))
}
