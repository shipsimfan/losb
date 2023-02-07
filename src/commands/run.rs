use super::create_image::create_image;
use crate::{
    args::Options,
    tools::{gdb, qemu},
};

fn common(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    create_image(options)?;
    options.output().log_custom("Running", "LOS", false, true);
    Ok(())
}

pub fn run(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    common(options)?;
    Ok(qemu::run(options)?)
}

pub fn debug(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    common(options)?;

    let emulator = qemu::debug(options)?;

    gdb::run(options)?;

    drop(emulator);
    Ok(())
}
