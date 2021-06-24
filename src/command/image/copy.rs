use super::fat32;
use std::{io::Read, path::Path};

struct Copier {
    file: std::fs::File,
    first_data_sector: usize,
    next_cluster: u32,
}

pub fn copy_directory(
    target_image: &Path,
    source_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    print!(
        "     \x1B[34;1mCopying\x1B[0m {} into {} . . .",
        source_path.to_string_lossy(),
        target_image.to_string_lossy()
    );

    let mut copier = Copier::new(target_image)?;
    copier.copy_directory(source_path, 2)?;

    Ok(println!(
        "\r    \x1B[32;1mFinished\x1B[0m copying {} into {}",
        source_path.to_string_lossy(),
        target_image.to_string_lossy()
    ))
}

impl Copier {
    pub fn new(filepath: &Path) -> Result<Self, Box<dyn std::error::Error>> {
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
            first_data_sector: bpb.first_data_sector(),
            next_cluster: 3,
        })
    }

    pub fn copy_directory(
        &mut self,
        path: &Path,
        first_cluster: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut entry_index = 2;
        for child in std::fs::read_dir(path)? {
            let child = child?;

            // Insert child object
            if child.metadata()?.is_dir() {
                // Allocate directory entry cluster

                // Prepare empty directory

                // Recurse
            } else {
                // Copy file
            }

            // Insert child entry

            entry_index += 1;
        }

        Ok(())
    }

    fn allocate_cluster(&mut self, previous_cluster: u32) -> u32 {
        // Change previous clusters value

        // Write new cluster

        // Return
        0
    }
}
