use super::install_file;
use crate::{
    args::Options,
    commands::{build::build_kernel, kernel_path, KERNEL_NAME, KERNEL_SYSROOT_PATH},
    output::{Color, Finish, Initial},
};

const KERNEL_FILENAME: &'static str = "kernel.elf";

pub fn install_kernel(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    build_kernel(options)?;

    options.output().log(
        "Installing",
        KERNEL_NAME,
        Initial::NewLineNotFirst,
        Color::Green,
        Finish::dots_newline(),
    );
    install_file(
        kernel_path(options),
        KERNEL_SYSROOT_PATH,
        KERNEL_FILENAME,
        options,
    )?;
    options.output().log_complete("Installed", KERNEL_NAME);

    Ok(())
}
