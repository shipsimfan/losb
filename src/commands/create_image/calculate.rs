use super::error::CreateImageError;
use crate::args::Options;
use std::path::Path;

const MINIMUM_CLUSTERS: usize = 65525;
const EXTRA_FREE_SPACE: usize = 32 * 1024 * 1024; // 32 MB of extra free space

const DIRECTORY_ENTRY_SIZE: usize = 32;

// Returns the number of sectors the FAT occupies
pub fn calculate_fat_size(options: &Options) -> Result<usize, CreateImageError> {
    let extra_clusters = bytes_to_clusters(EXTRA_FREE_SPACE, options);

    let clusters = calculate_directory(options.sysroot(), true, options)? + extra_clusters + 2;

    let clusters_per_fat_sector = options.sector_size() / 4;

    Ok(if clusters < MINIMUM_CLUSTERS {
        MINIMUM_CLUSTERS
    } else {
        clusters
    }
    .div_ceil(clusters_per_fat_sector))
}

fn bytes_to_clusters(bytes: usize, options: &Options) -> usize {
    sectors_to_clusters(bytes.div_ceil(options.sector_size()), options)
}

fn sectors_to_clusters(sectors: usize, options: &Options) -> usize {
    sectors.div_ceil(options.sectors_per_cluster())
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

        // TODO: Calculate if the entry needs a long name
        bytes += DIRECTORY_ENTRY_SIZE;

        let path = entry.path();

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
    clusters += (((bytes + options.sector_size() - 1) / options.sector_size())
        + options.sectors_per_cluster()
        - 1)
        / options.sectors_per_cluster();

    Ok(clusters)
}
