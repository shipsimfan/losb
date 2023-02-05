use super::{bpb, error::CreateImageError, Cluster};
use crate::args::Options;
use std::fs::File;

pub struct FileWriter {
    output: File,
}

fn calculate_size(fat_size: usize, options: &Options) -> usize {
    let mut size = 0;

    // Add the reserved sectors
    size += options.reserved_sectors() as usize * options.sector_size() as usize;

    // Add sectors for FATs
    size += options.sector_size() as usize * fat_size * options.num_fats() as usize;

    // Add sectors for data section
    let clusters_per_fat_sector = options.sector_size() as usize / 4;
    let num_clusters = fat_size * clusters_per_fat_sector;
    size += num_clusters * options.sectors_per_cluster() as usize * options.sector_size() as usize;

    size
}

impl FileWriter {
    pub fn new(fat_size: usize, options: &Options) -> Result<Self, CreateImageError> {
        // Calculate full file size from fat size
        let full_size = calculate_size(fat_size, options);

        // Create the output image
        let mut output = std::fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(options.output_path())
            .map_err(|error| CreateImageError::WriteError(error))?;

        // Set the output images length
        output
            .set_len(full_size as u64)
            .map_err(|error| CreateImageError::WriteError(error))?;

        // Write the BPB
        bpb::write_bpb(full_size, fat_size, &mut output, options)?;

        // Write the FSInfo

        // Zero the FAT

        // Setup the writer

        Ok(FileWriter { output })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<Cluster, CreateImageError> {
        // Allocate a cluster chain

        // Write the data into the cluster chain

        // Return the start of the chain

        panic!("TODO: Implement")
    }

    pub fn set_root_cluster(&mut self, cluster: Cluster) -> Result<(), CreateImageError> {
        panic!("TODO: Implement")
    }
}
