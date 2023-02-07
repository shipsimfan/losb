use crate::args::Options;

mod bootloader;
mod kernel;
mod sysroot;

pub use bootloader::clean_bootloader;
pub use kernel::clean_kernel;
pub use sysroot::clean_sysroot;

pub fn clean_all(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    clean_sysroot(options)?;
    clean_bootloader(options)?;
    Ok(clean_kernel(options)?)
}
