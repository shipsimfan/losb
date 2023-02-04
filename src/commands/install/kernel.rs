use super::{error::InstallError, install_file};
use crate::{
    args::Options,
    commands::{build::build_kernel, names::KERNEL_NAME},
    output::Output,
};

const KERNEL_RELEASE_PATH: &'static str = "kernel/target/release/kernel.elf";
const KERNEL_DEBUG_PATH: &'static str = "kernel/target/debug/kernel.elf";

const KERNEL_DESTINATION_PATH: &'static str = "los/kernel.elf";

const KERNEL_FILENAME: &'static str = "kernel.elf";

pub fn install_kernel(
    options: &Options,
    output: &Output,
) -> Result<(), Box<dyn std::error::Error>> {
    build_kernel(options, output)?;

    output.log_installing(KERNEL_NAME);
    Ok(install_file(
        if options.is_release() {
            KERNEL_RELEASE_PATH
        } else {
            KERNEL_DEBUG_PATH
        },
        KERNEL_DESTINATION_PATH,
        KERNEL_FILENAME,
        options,
        output,
    )?)
}
