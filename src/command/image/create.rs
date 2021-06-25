use super::fat32;
use std::{
    io::{Seek, SeekFrom, Write},
    path::Path,
};

fn write_boot_sector(
    file: &mut std::fs::File,
    bpb: &fat32::BIOSParameterBlock,
) -> Result<(), Box<dyn std::error::Error>> {
    file.write(unsafe {
        std::slice::from_raw_parts(
            bpb as *const _ as *const u8,
            std::mem::size_of::<fat32::BIOSParameterBlock>(),
        )
    })?;

    file.seek(SeekFrom::Current(
        510 - std::mem::size_of::<fat32::BIOSParameterBlock>() as i64,
    ))?;

    file.write(&[0x55, 0xAA])?;

    Ok(())
}

fn write_root_directory(
    file: &mut std::fs::File,
    bpb: &fat32::BIOSParameterBlock,
) -> Result<(), Box<dyn std::error::Error>> {
    // Write FAT entries
    let mut i = 0;
    while i < bpb.num_fats() {
        file.seek(SeekFrom::Start(
            (fat32::BYTES_PER_SECTOR * (fat32::RESERVED_SECTOR_COUNT + i * bpb.fat_size())) as u64,
        ))?;
        file.write(&[
            0xFF, 0xFF, 0xFF, 0xF, 0xFF, 0xFF, 0xFF, 0xF, 0xFF, 0xFF, 0xFF, 0xF,
        ])?;

        i += 1;
    }

    // Write directory entry
    file.seek(SeekFrom::Start(
        (fat32::BYTES_PER_SECTOR
            * (fat32::RESERVED_SECTOR_COUNT + (fat32::NUM_FATS * bpb.fat_size()))) as u64,
    ))?;

    let volume_id_entry =
        fat32::DirectoryEntry::new(bpb.volume_label().clone(), fat32::ATTR_VOLUME_ID, 0, 0);
    file.write(unsafe {
        std::slice::from_raw_parts(
            &volume_id_entry as *const _ as *const u8,
            std::mem::size_of::<fat32::DirectoryEntry>(),
        )
    })?;

    Ok(())
}

pub fn create_image(volume_size: usize, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    print!(
        "    \x1B[34;1mCreating\x1B[0m {} ({} MB) . . .",
        target.to_string_lossy(),
        volume_size / 1024 / 1024,
    );

    // Open the new image
    let mut target_file = std::fs::File::create(target)?;

    // Write the BPB
    let bpb = fat32::BIOSParameterBlock::new(volume_size);
    target_file.seek(SeekFrom::Start(0))?;
    write_boot_sector(&mut target_file, &bpb)?;
    target_file.seek(SeekFrom::Start(fat32::BYTES_PER_SECTOR as u64 * 6))?;
    write_boot_sector(&mut target_file, &bpb)?;

    // Write the FS info
    let fsinfo = fat32::FSInfo::new();
    target_file.seek(SeekFrom::Start(fat32::BYTES_PER_SECTOR as u64 * 1))?;
    target_file.write(unsafe {
        std::slice::from_raw_parts(
            &fsinfo as *const _ as *const u8,
            std::mem::size_of::<fat32::FSInfo>(),
        )
    })?;
    drop(fsinfo);

    // Write root directory
    write_root_directory(&mut target_file, &bpb)?;
    drop(bpb);

    // Set file size
    target_file.set_len(volume_size as u64)?;

    Ok(println!(
        "\r    \x1B[32;1mFinished\x1B[0m creating {} ({} MB)",
        target.to_string_lossy(),
        volume_size / 1024 / 1024,
    ))
}
