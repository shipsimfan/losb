use std::path::Path;

mod calculate;
mod copy;
mod create;
mod fat32;

pub fn build_image() -> Result<(), Box<dyn std::error::Error>> {
    crate::build::build()?;

    println!();
    let sysroot_path = Path::new(crate::config::SYSROOT_DIR);
    let target_path = Path::new(crate::config::TARGET_IMG);

    // Calculate image size
    let volume_size = calculate::volume_size(sysroot_path)?;

    // Create blank FAT32 image
    create::create_image(volume_size, target_path)?;

    // Copy sysroot into the image
    copy::copy_directory(target_path, sysroot_path)
}
