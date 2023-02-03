use crate::{args::Options, output::Output, tools::ToolError};

mod bootloader;
mod kernel;

pub use bootloader::build_bootloader;
pub use kernel::build_kernel;

pub fn build_all(options: &Options, output: &Output) -> Result<(), ToolError> {
    build_bootloader(options, output)?;
    build_kernel(options, output)
}
