use super::fat32::LongDirectoryEntry;

use super::fat32;
use std::{
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

struct Copier {
    file: std::fs::File,
    first_fat_sector: usize,
    first_data_sector: usize,
    fat_size: usize,
    num_fats: usize,
    next_cluster: u32,
}

pub fn copy_directory(target_image: &Path, source_path: &Path) -> Result<(), std::io::Error> {
    print!(
        "     \x1B[36;1mCopying\x1B[0m {} into {} . . .",
        source_path.to_string_lossy(),
        target_image.to_string_lossy()
    );

    let mut copier = Copier::new(target_image)?;
    copier.copy_directory(source_path, 2, 1)?;

    Ok(println!(
        "\r    \x1B[32;1mFinished\x1B[0m copying {} into {}",
        source_path.to_string_lossy(),
        target_image.to_string_lossy()
    ))
}

impl Copier {
    pub fn new(filepath: &Path) -> Result<Self, std::io::Error> {
        // Read the BPB
        let mut target_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(filepath)?;

        let mut bpb: fat32::BIOSParameterBlock = unsafe { std::mem::zeroed() };
        {
            let bpb_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    &mut bpb as *mut _ as *mut u8,
                    std::mem::size_of::<fat32::BIOSParameterBlock>(),
                )
            };

            target_file.read_exact(bpb_slice)?;
        }

        Ok(Copier {
            file: target_file,
            first_fat_sector: fat32::RESERVED_SECTOR_COUNT,
            first_data_sector: bpb.first_data_sector(),
            fat_size: bpb.fat_size(),
            num_fats: bpb.num_fats(),
            next_cluster: 3,
        })
    }

    pub fn copy_directory(
        &mut self,
        path: &Path,
        first_cluster: u32,
        first_index: usize,
    ) -> Result<(), std::io::Error> {
        let mut entry_index = first_index;
        let mut current_cluster = first_cluster;
        let mut buffer = [fat32::DirectoryEntry::zero();
            fat32::BYTES_PER_SECTOR / std::mem::size_of::<fat32::DirectoryEntry>()];

        self.read_cluster(current_cluster, unsafe {
            std::slice::from_raw_parts_mut(
                &mut buffer as *mut _ as *mut u8,
                fat32::BYTES_PER_SECTOR,
            )
        })?;

        let mut numeric_tail_value = 1;

        for child in std::fs::read_dir(path)? {
            let child = child?;

            // Insert child object
            let entry = if child.metadata()?.is_dir() {
                // Allocate directory entry cluster
                let directory_cluster = self.allocate_cluster(0)?;

                // Prepare empty directory
                let mut directory = [fat32::DirectoryEntry::zero();
                    fat32::BYTES_PER_SECTOR / std::mem::size_of::<fat32::DirectoryEntry>()];
                directory[0] = fat32::DirectoryEntry::new(
                    [
                        '.' as u8, ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8,
                        ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8,
                    ],
                    fat32::ATTR_DIRECTORY,
                    directory_cluster,
                    0,
                );
                directory[1] = fat32::DirectoryEntry::new(
                    [
                        '.' as u8, '.' as u8, ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8,
                        ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8, ' ' as u8,
                    ],
                    fat32::ATTR_DIRECTORY,
                    first_cluster,
                    0,
                );

                self.write_cluster(directory_cluster, unsafe {
                    std::slice::from_raw_parts(
                        &directory as *const _ as *const u8,
                        fat32::BYTES_PER_SECTOR,
                    )
                })?;

                // Recurse
                self.copy_directory(&child.path(), directory_cluster, 2)?;

                let mut name = [' ' as u8; 11];
                let mut i = 0;
                for c in child.file_name().to_string_lossy().as_bytes() {
                    if i >= 11 {
                        break;
                    }

                    name[i] = (*c).to_ascii_uppercase();
                    i += 1;
                }

                fat32::DirectoryEntry::new(name, fat32::ATTR_DIRECTORY, directory_cluster, 0)
            } else {
                // Copy file
                let (cluster, file_size) = self.copy_file(&child.path())?;

                // Generate basis name
                let child_path = child.path();
                let filename = child_path.file_name().unwrap().to_str().unwrap();

                let short_name = filename
                    .to_ascii_uppercase()
                    .replace(' ', "")
                    .trim_start_matches('.')
                    .to_string();

                let mut basis_name = String::new();
                for char in short_name.chars() {
                    if basis_name.len() >= 8 || char == '.' {
                        break;
                    }

                    basis_name.push(char);
                }

                match child_path.extension() {
                    Some(extension) => {
                        basis_name.push('.');
                        let mut i = 0;
                        for char in extension.to_str().unwrap().chars() {
                            if i >= 3 {
                                break;
                            }

                            basis_name.push(char.to_ascii_uppercase());
                            i += 1;
                        }
                    }
                    None => {}
                }

                // Check to see if long filenames are nescessary
                let mut name = [b' '; 11];
                if child_path.file_stem().unwrap().len() > 8
                    || match child_path.extension() {
                        None => false,
                        Some(extension) => extension.len() > 3,
                    }
                {
                    // Generate numeric tail
                    let numeric_tail = format!("~{}", numeric_tail_value);
                    numeric_tail_value += 1;
                    let mut i = 0;
                    let end = 8 - numeric_tail.len();
                    let mut iter = basis_name.chars();
                    let mut reached_extension = false;
                    while let Some(c) = iter.next() {
                        if c == '.' {
                            reached_extension = true;
                            break;
                        }

                        if i >= end {
                            break;
                        }

                        name[i] = c as u8;
                        i += 1;
                    }

                    for c in numeric_tail.chars() {
                        name[i] = c as u8;
                        i += 1;
                    }

                    while i < 8 {
                        name[i] = b' ';
                        i += 1;
                    }

                    if !reached_extension {
                        while let Some(c) = iter.next() {
                            if c == '.' {
                                break;
                            }
                        }
                    }

                    while let Some(c) = iter.next() {
                        name[i] = c as u8;
                        i += 1;
                    }

                    while i < 11 {
                        name[i] = b' ';
                        i += 1;
                    }

                    // Create long file name entries
                    let num_entries = (filename.len() + 12) / 13;

                    let checksum = {
                        let mut sum: u8 = 0;
                        for byte in name {
                            sum = if sum & 1 != 0 { 0x80 } else { 0 } + sum.wrapping_shr(1) + byte;
                        }

                        sum
                    };

                    let mut current_entry = num_entries;
                    let mut current_offset = (num_entries - 1) * 13;
                    while current_entry > 0 {
                        let long_entry = LongDirectoryEntry::new(
                            &filename[current_offset..],
                            if current_entry == num_entries {
                                current_entry | 0x40
                            } else {
                                current_entry
                            } as u8,
                            checksum,
                        );

                        let entry = long_entry.as_entry();

                        // Insert long filename entry
                        if entry_index
                            % (fat32::BYTES_PER_SECTOR
                                / std::mem::size_of::<fat32::DirectoryEntry>())
                            == 0
                        {
                            self.write_cluster(current_cluster, unsafe {
                                std::slice::from_raw_parts(
                                    &buffer as *const _ as *const u8,
                                    fat32::BYTES_PER_SECTOR,
                                )
                            })?;
                            current_cluster = self.allocate_cluster(current_cluster)?;
                            self.read_cluster(current_cluster, unsafe {
                                std::slice::from_raw_parts_mut(
                                    &mut buffer as *mut _ as *mut u8,
                                    fat32::BYTES_PER_SECTOR,
                                )
                            })?;
                        }

                        buffer[entry_index
                            % (fat32::BYTES_PER_SECTOR
                                / std::mem::size_of::<fat32::DirectoryEntry>())] = *entry;

                        entry_index += 1;

                        current_entry -= 1;
                        current_offset -= 13;
                    }
                } else {
                    let mut i = 0;
                    let mut iter = basis_name.chars();
                    while let Some(c) = iter.next() {
                        if c == '.' {
                            break;
                        }

                        name[i] = c as u8;
                        i += 1;
                    }

                    while i < 8 {
                        name[i] = b' ';
                        i += 1;
                    }

                    while let Some(c) = iter.next() {
                        name[i] = c as u8;
                        i += 1;
                    }

                    while i < 11 {
                        name[i] = b' ';
                        i += 1;
                    }
                };

                fat32::DirectoryEntry::new(name, 0, cluster, file_size as u32)
            };

            // Insert child entry
            if entry_index
                % (fat32::BYTES_PER_SECTOR / std::mem::size_of::<fat32::DirectoryEntry>())
                == 0
            {
                self.write_cluster(current_cluster, unsafe {
                    std::slice::from_raw_parts(
                        &buffer as *const _ as *const u8,
                        fat32::BYTES_PER_SECTOR,
                    )
                })?;
                current_cluster = self.allocate_cluster(current_cluster)?;
                self.read_cluster(current_cluster, unsafe {
                    std::slice::from_raw_parts_mut(
                        &mut buffer as *mut _ as *mut u8,
                        fat32::BYTES_PER_SECTOR,
                    )
                })?;
            }

            buffer[entry_index
                % (fat32::BYTES_PER_SECTOR / std::mem::size_of::<fat32::DirectoryEntry>())] = entry;

            entry_index += 1;
        }

        self.write_cluster(current_cluster, unsafe {
            std::slice::from_raw_parts(&buffer as *const _ as *const u8, fat32::BYTES_PER_SECTOR)
        })?;

        Ok(())
    }

    fn copy_file(&mut self, path: &Path) -> Result<(u32, usize), std::io::Error> {
        let file_data = std::fs::read(path)?;
        let num_clusters =
            (file_data.len() + fat32::BYTES_PER_SECTOR - 1) / fat32::BYTES_PER_SECTOR;
        let mut i = 0;
        let mut previous_cluster = 0;
        let mut first_cluster = 0;
        while i < num_clusters {
            let cluster = self.allocate_cluster(previous_cluster)?;

            if first_cluster == 0 {
                first_cluster = cluster;
            }

            if i + 1 != num_clusters {
                self.write_cluster(
                    cluster,
                    &file_data[(i * fat32::BYTES_PER_SECTOR)..((i + 1) * fat32::BYTES_PER_SECTOR)],
                )?;
            } else {
                self.write_cluster(cluster, &file_data[(i * fat32::BYTES_PER_SECTOR)..])?;
            }

            previous_cluster = cluster;
            i += 1;
        }

        Ok((first_cluster, file_data.len()))
    }

    fn write_cluster(&mut self, cluster: u32, buffer: &[u8]) -> Result<(), std::io::Error> {
        let sector = cluster - 2 + self.first_data_sector as u32;
        self.file.seek(SeekFrom::Start(
            (sector * fat32::BYTES_PER_SECTOR as u32) as u64,
        ))?;
        self.file.write(buffer)?;
        Ok(())
    }

    fn read_cluster(&mut self, cluster: u32, buffer: &mut [u8]) -> Result<(), std::io::Error> {
        let sector = cluster - 2 + self.first_data_sector as u32;
        self.file.seek(SeekFrom::Start(
            (sector * fat32::BYTES_PER_SECTOR as u32) as u64,
        ))?;
        self.file.read(buffer)?;
        Ok(())
    }

    fn allocate_cluster(&mut self, previous_cluster: u32) -> Result<u32, std::io::Error> {
        let mut i = 0;
        let mut buffer = [0u8; fat32::BYTES_PER_SECTOR];

        while i < self.num_fats {
            // Change previous clusters value
            if previous_cluster != 0 {
                let previous_offset = previous_cluster as usize * 4;
                let previous_sector = self.first_fat_sector
                    + (previous_offset / fat32::BYTES_PER_SECTOR)
                    + (self.fat_size * i);
                let previous_offset = previous_offset % fat32::BYTES_PER_SECTOR;

                self.file.seek(SeekFrom::Start(
                    (previous_sector * fat32::BYTES_PER_SECTOR) as u64,
                ))?;
                self.file.read(&mut buffer)?;

                buffer[previous_offset + 0] = ((self.next_cluster.wrapping_shr(0)) & 0xFF) as u8;
                buffer[previous_offset + 1] = ((self.next_cluster.wrapping_shr(8)) & 0xFF) as u8;
                buffer[previous_offset + 2] = ((self.next_cluster.wrapping_shr(16)) & 0xFF) as u8;
                buffer[previous_offset + 3] = ((self.next_cluster.wrapping_shr(24)) & 0xFF) as u8;

                self.file.seek(SeekFrom::Start(
                    (previous_sector * fat32::BYTES_PER_SECTOR) as u64,
                ))?;
                self.file.write(&buffer)?;
            }

            // Write new cluster
            let new_offset = self.next_cluster as usize * 4;
            let new_sector = self.first_fat_sector
                + (new_offset / fat32::BYTES_PER_SECTOR)
                + (self.fat_size * i);
            let new_offset = new_offset % fat32::BYTES_PER_SECTOR;

            self.file.seek(SeekFrom::Start(
                (new_sector * fat32::BYTES_PER_SECTOR) as u64,
            ))?;
            self.file.read(&mut buffer)?;

            buffer[new_offset + 0] = 0xFF;
            buffer[new_offset + 1] = 0xFF;
            buffer[new_offset + 2] = 0xFF;
            buffer[new_offset + 3] = 0x0F;

            self.file.seek(SeekFrom::Start(
                (new_sector * fat32::BYTES_PER_SECTOR) as u64,
            ))?;
            self.file.write(&buffer)?;

            i += 1;
        }

        // Set next cluster
        let ret = self.next_cluster;
        self.next_cluster += 1;
        Ok(ret)
    }
}
