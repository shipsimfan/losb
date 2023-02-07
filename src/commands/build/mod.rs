use crate::{args::Options, tools::ToolError};

mod bootloader;
mod kernel;

pub use bootloader::build_bootloader;
pub use kernel::build_kernel;

pub fn build_all(options: &Options) -> Result<(), ToolError> {
    build_bootloader(options)?;
    build_kernel(options)
}
