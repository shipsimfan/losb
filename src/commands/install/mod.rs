use self::error::InstallError;
use crate::args::Options;
use std::path::{Path, PathBuf};

mod bootloader;
mod error;
mod kernel;

pub use bootloader::install_bootloader;
pub use kernel::install_kernel;

pub fn install_all(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    install_bootloader(options)?;
    install_kernel(options)
}

fn install_file<S: AsRef<Path>, D: AsRef<Path>, F: AsRef<str>>(
    source: S,
    destination: D,
    file_name: F,
    options: &Options,
) -> Result<(), InstallError> {
    options.output().log_installing_file(file_name.as_ref());
    let (full_source, full_destination) = prepare_install_paths(source, destination, options);
    make_directories(&full_destination, file_name.as_ref())?;
    copy_file(&full_source, &full_destination, file_name.as_ref())
}

fn prepare_install_paths<S: AsRef<Path>, D: AsRef<Path>>(
    source: S,
    destination: D,
    options: &Options,
) -> (PathBuf, PathBuf) {
    (
        options.path().join(source),
        options.sysroot().join(destination),
    )
}

fn make_directories(path: &Path, file_name: &str) -> Result<(), InstallError> {
    match path.parent() {
        Some(parent_path) => std::fs::create_dir_all(parent_path)
            .map_err(|error| InstallError::new(file_name, error)),
        None => Ok(()),
    }
}

fn copy_file(from: &Path, to: &Path, file_name: &str) -> Result<(), InstallError> {
    std::fs::copy(from, to)
        .map(|_| ())
        .map_err(|error| InstallError::new(file_name, error))
}
