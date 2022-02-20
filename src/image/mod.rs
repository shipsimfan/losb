use std::path::Path;

mod calculate;
mod copy;
mod create;
mod fat32;

#[derive(Debug)]
pub enum BuildImageError {
    BuildError(crate::build::BuildError),
    CreateImageError(std::io::Error),
    SysrootError(std::io::Error),
}

pub fn build_image() -> Result<(), BuildImageError> {
    crate::build::build()?;

    println!();
    let sysroot_path = Path::new(crate::config::SYSROOT_DIR);
    let target_path = Path::new(crate::config::TARGET_IMG);

    // Calculate image size
    let volume_size = calculate::volume_size(sysroot_path).unwrap();

    // Create blank FAT32 image
    match create::create_image(volume_size, target_path) {
        Ok(()) => {}
        Err(error) => return Err(BuildImageError::CreateImageError(error)),
    };

    // Copy sysroot into the image
    match copy::copy_directory(target_path, sysroot_path) {
        Ok(()) => Ok(()),
        Err(error) => Err(BuildImageError::SysrootError(error)),
    }
}

impl std::error::Error for BuildImageError {}

impl std::fmt::Display for BuildImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BuildImageError::BuildError(error) => format!("{}", error),
                BuildImageError::CreateImageError(error) =>
                    format!("Unable to create blank image ({})", error),
                BuildImageError::SysrootError(error) =>
                    format!("Unable to copy sysroot into image ({})", error),
            }
        )
    }
}

impl From<crate::build::BuildError> for BuildImageError {
    fn from(error: crate::build::BuildError) -> Self {
        BuildImageError::BuildError(error)
    }
}
