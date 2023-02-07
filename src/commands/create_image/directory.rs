use super::{error::CreateImageError, fat32::DirectoryEntry, writer::FileWriter, Cluster};
use crate::args::Options;
use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

pub fn write_root_directory(
    writer: &mut FileWriter,
    path: PathBuf,
    options: &Options,
) -> Result<Cluster, CreateImageError> {
    write_directory(writer, path, true, (0, 0, 0), (0, 0), 0, options)
}

fn struct_slice_to_slice<T: Sized>(value: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            value.as_ptr() as *const u8,
            std::mem::size_of::<T>() * value.len(),
        )
    }
}

fn write_directory(
    writer: &mut FileWriter,
    path: PathBuf,
    root: bool,
    creation: (u16, u16, u8),
    write: (u16, u16),
    parent_cluster: Cluster,
    options: &Options,
) -> Result<Cluster, CreateImageError> {
    // Collect the directory entries
    let mut dir_entries = collect_entries(&path)?;

    // Create the FAT entries
    let mut fat_entries = create_fat_entries(root, &mut dir_entries, creation, write);

    // Reserve our clusters
    let cluster_size = options.sector_size() as usize * options.sectors_per_cluster() as usize;
    let our_cluster = writer.allocate_clusters(
        (fat_entries.len() * std::mem::size_of::<DirectoryEntry>()).div_ceil(cluster_size),
    )?;

    // Write our children and update first cluster numbers
    let mut fat_i = if root {
        0
    } else {
        fat_entries[0].set_root_cluster(our_cluster);
        fat_entries[1].set_root_cluster(parent_cluster);
        2
    };

    for i in 0..dir_entries.len() {
        let dir_entry = match &dir_entries[i] {
            Some(dir_entry) => dir_entry,
            None => continue,
        };

        let path = dir_entry.path();
        let cluster = if fat_entries[fat_i].is_directory() {
            write_directory(
                writer,
                path,
                false,
                fat_entries[fat_i].creation(),
                fat_entries[fat_i].write(),
                our_cluster,
                options,
            )?
        } else {
            let file =
                std::fs::read(&path).map_err(|error| CreateImageError::ReadError(path, error))?;

            let cluster = writer.allocate_clusters(file.len().div_ceil(cluster_size))?;
            writer.write_data(&file, cluster)?;

            cluster
        };

        fat_entries[fat_i].set_root_cluster(cluster);

        fat_i += 1;
    }

    // Write our clusters
    writer.write_data(struct_slice_to_slice(fat_entries.as_slice()), our_cluster)?;

    Ok(our_cluster)
}

fn collect_entries(path: &Path) -> Result<Vec<Option<DirEntry>>, CreateImageError> {
    let read_dir = std::fs::read_dir(path)
        .map_err(|error| CreateImageError::ReadError(path.to_owned(), error))?;
    let entry_results: Vec<_> = read_dir.collect();
    let mut entries = Vec::with_capacity(entry_results.len());

    for entry in entry_results {
        entries.push(Some(entry.map_err(|error| {
            CreateImageError::ReadError(path.to_owned(), error)
        })?));
    }

    Ok(entries)
}

fn create_fat_entries(
    root: bool,
    dir_entries: &mut [Option<DirEntry>],
    creation: (u16, u16, u8),
    write: (u16, u16),
) -> Vec<DirectoryEntry> {
    let mut entries = Vec::with_capacity(dir_entries.len() + if root { 0 } else { 2 });

    if !root {
        entries.push(DirectoryEntry::new_dot(creation, write));
        entries.push(DirectoryEntry::new_dot_dot(creation, write));
    }

    for dir_entry in dir_entries {
        match dir_entry {
            Some(entry) => match DirectoryEntry::new(entry) {
                Some(entry) => entries.push(entry),
                None => *dir_entry = None,
            },
            None => continue,
        }
    }

    entries
}
