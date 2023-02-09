use super::create_image::create_image;
use crate::{
    args::Options,
    output::{Color, Finish, Initial},
    tools::{gdb, qemu},
};

fn common(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    create_image(options)?;
    options.output().log(
        "Running",
        "LOS",
        Initial::NewLineNotFirst,
        Color::Green,
        Finish::dots_newline(),
    );
    Ok(())
}

pub fn run(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    common(options)?;
    Ok(qemu::run(options)?)
}

pub fn debug(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    common(options)?;

    let emulator = qemu::debug(options)?;

    if options.execute_gdb() {
        gdb::run(options)?;
    }

    drop(emulator);
    Ok(())
}
