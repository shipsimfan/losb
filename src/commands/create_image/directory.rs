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
    let mut builder = DirectoryBuilder::new(path)?;

    if !root {
        // Add "." and ".." entries
    }

    // Write each entry
    while let Some(entry) = builder.next()? {
        let path = entry.path();

        let (cluster, directory) = if entry
            .file_type()
            .map_err(|error| CreateImageError::ReadError(path.clone(), error))?
            .is_dir()
        {
            (write_directory(writer, path, false)?, true)
        } else {
            let cluster = writer.write(
                &std::fs::read(&path).map_err(|error| CreateImageError::ReadError(path, error))?,
            )?;
            (cluster, false)
        };

        // Create entry

        // Push entry
    }

    // Write the directory clusters
    writer.write(&builder.generate_clusters())
}

impl DirectoryBuilder {
    pub fn new(path: PathBuf) -> Result<Self, CreateImageError> {
        // Open the directory

        panic!("TODO: Implement")
    }

    pub fn next(&mut self) -> Result<Option<DirEntry>, CreateImageError> {
        match self.read_dir.next() {
            Some(result) => match result {
                Ok(entry) => Ok(Some(entry)),
                Err(error) => Err(CreateImageError::ReadError(self.path.clone(), error)),
            },
            None => Ok(None),
        }
    }

    pub fn push_entry(&mut self, entry: DirectoryEntry) {
        self.entries.push(entry);
    }

    pub fn generate_clusters(self) -> Vec<u8> {
        panic!("TODO: Implement")
    }
}
