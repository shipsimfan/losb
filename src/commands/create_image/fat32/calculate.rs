use super::directory_entry::{calculate_short_name, DirectoryEntry};
use crate::{
    args::Options,
    commands::create_image::{error::CreateImageError, Cluster},
};
use std::path::Path;

const MINIMUM_CLUSTERS: usize = 65525;
const EXTRA_FREE_SPACE: usize = 32 * 1024 * 1024; // 32 MB of extra free space

const DIRECTORY_ENTRY_SIZE: usize = std::mem::size_of::<DirectoryEntry>();
const CLUSTER_SIZE: usize = std::mem::size_of::<Cluster>();
const RESERVED_CLUSTERS: usize = 2;

fn bytes_to_clusters(bytes: usize, options: &Options) -> usize {
    sectors_to_clusters(bytes.div_ceil(options.sector_size() as usize), options)
}

fn sectors_to_clusters(sectors: usize, options: &Options) -> usize {
    sectors.div_ceil(options.sectors_per_cluster() as usize)
}

pub fn calculate_size(fat_size: usize, options: &Options) -> usize {
    let mut size = 0;

    // Add the reserved sectors
    size += options.reserved_sectors() as usize * options.sector_size() as usize;

    // Add sectors for FATs
    size += fat_size * options.num_fats() as usize;

    // Add sectors for data section
    let num_clusters = (fat_size / CLUSTER_SIZE) - RESERVED_CLUSTERS;
    size += num_clusters * options.sectors_per_cluster() as usize * options.sector_size() as usize;

    size
}

pub fn calculate_fat_size(options: &Options) -> Result<usize, CreateImageError> {
    let extra_clusters = bytes_to_clusters(EXTRA_FREE_SPACE, options);

    let clusters = calculate_directory(options.sysroot(), true, options)? + extra_clusters + 2;

    Ok((if clusters < MINIMUM_CLUSTERS {
        MINIMUM_CLUSTERS
    } else {
        clusters
    } * CLUSTER_SIZE)
        .next_multiple_of(options.sector_size() as usize))
}

// Calculate the number of clusters a given directory requires
fn calculate_directory(
    path: &Path,
    root: bool,
    options: &Options,
) -> Result<usize, CreateImageError> {
    let mut bytes = 0;
    let mut clusters = 0;

    if !root {
        // Account for "." and ".." entries
        bytes += 2 * DIRECTORY_ENTRY_SIZE;
    }

    // Calculate entries
    for entry in std::fs::read_dir(path)
        .map_err(|error| CreateImageError::CalculateError(path.to_owned(), error))?
    {
        let entry =
            entry.map_err(|error| CreateImageError::CalculateError(path.to_owned(), error))?;

        let path = entry.path();

        let file_name = match path.file_name() {
            Some(file_name) => file_name.to_string_lossy(),
            None => {
                options.output().log_warning("Discovered an unnamed file");
                continue;
            }
        };

        bytes += DIRECTORY_ENTRY_SIZE
            * match calculate_short_name(file_name.as_bytes()) {
                Some(_) => 1,
                None => {
                    options.output().log_warning(&format!(
                        "\"{}\" is an invalid filename. It will not be in the output image",
                        file_name
                    ));
                    continue;
                }
            };

        // Calculate entry clusters
        clusters += if entry
            .file_type()
            .map_err(|error| CreateImageError::CalculateError(path.to_owned(), error))?
            .is_dir()
        {
            calculate_directory(&path, false, options)?
        } else {
            bytes_to_clusters(
                entry
                    .metadata()
                    .map_err(|error| CreateImageError::CalculateError(path.to_owned(), error))?
                    .len() as usize,
                options,
            )
        }
    }

    // Add clusters used by entries
    clusters += bytes_to_clusters(bytes, options);

    Ok(clusters)
}
