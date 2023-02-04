use super::{error::CreateImageError, writer::FileWriter, Cluster};
use std::{
    fs::{DirEntry, ReadDir},
    path::PathBuf,
};

struct DirectoryBuilder {
    read_dir: ReadDir,
    path: PathBuf,
    entries: Vec<DirectoryEntry>,
}

struct DirectoryEntry {}

pub fn write_directory(
    writer: &mut FileWriter,
    path: PathBuf,
    root: bool,
) -> Result<Cluster, CreateImageError> {
    // Create the directory builder
    let builder = DirectoryBuilder::new(path)?;

    if !root {
        // Add "." and ".." entries
    }

    // Write each entry

    // Write the directory clusters
    writer.write(&builder.generate_clusters())
}

impl DirectoryBuilder {
    pub fn new(path: PathBuf) -> Result<Self, CreateImageError> {
        // Open the directory

        panic!("TODO: Implement")
    }

    pub fn next(&mut self) -> Result<Option<DirEntry>, CreateImageError> {
        panic!("TODO: Implement")
    }

    pub fn push_entry(&mut self, entry: DirectoryEntry) {
        self.entries.push(entry);
    }

    pub fn generate_clusters(self) -> Vec<u8> {
        panic!("TODO: Implement")
    }
}
