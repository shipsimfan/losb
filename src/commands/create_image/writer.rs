use super::{error::CreateImageError, Cluster};
use std::path::Path;

pub struct FileWriter {}

impl FileWriter {
    pub fn new<P: AsRef<Path>>(fat_size: usize, path: P) -> Result<Self, CreateImageError> {
        // Calculate full file size from fat size

        // Create the output image and set it's length

        // Write the BPB

        // Zero the FAT

        // Setup the writer

        panic!("TODO: Implement")
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
