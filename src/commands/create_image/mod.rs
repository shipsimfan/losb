use self::writer::FileWriter;
use super::install::install_all;
use crate::{args::Options, output::Output};

mod bpb;
mod calculate;
mod directory;
mod error;
mod file;
mod fs_info;
mod writer;

type Cluster = u32;

pub fn create_image(options: &Options, output: &Output) -> Result<(), Box<dyn std::error::Error>> {
    install_all(options, output)?;

    output.log_custom("Creating", "image", true, true);

    // Calculate FAT size
    let fat_size = calculate::calculate_fat_size(options)?;

    // Construct filesystem
    let mut writer = FileWriter::new(fat_size, options)?;
    let root_cluster =
        directory::write_root_directory(&mut writer, options.sysroot().to_owned(), options)?;
    writer.finalize(root_cluster)?;

    Ok(())
}
