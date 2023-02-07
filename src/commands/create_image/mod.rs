use self::writer::FileWriter;
use super::install::install_all;
use crate::args::Options;

mod directory;
mod error;
mod fat32;
mod file;
mod writer;

type Cluster = u32;

pub fn create_image(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    install_all(options)?;

    options.output().log_custom("Creating", "image", true, true);

    // Calculate FAT size
    let fat_size = fat32::calculate_fat_size(options)?;

    // Construct filesystem
    let mut writer = FileWriter::new(fat_size, options)?;
    let root_cluster =
        directory::write_root_directory(&mut writer, options.sysroot().to_owned(), options)?;
    writer.finalize(root_cluster)?;

    Ok(())
}
