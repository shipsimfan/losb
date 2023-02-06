use super::error::CreateImageError;
use crate::args::Options;
use std::path::Path;

const MINIMUM_CLUSTERS: usize = 65525;
const EXTRA_FREE_SPACE: usize = 32 * 1024 * 1024; // 32 MB of extra free space

const DIRECTORY_ENTRY_SIZE: usize = 32;

pub const ILLEGAL_SHORT_CHARACTERS: &[u8] = &[
    0x22, 0x2A, 0x2B, 0x2C, 0x2E, 0x2F, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x5B, 0x5C, 0x5D, 0x7C,
];

// Returns the number of sectors the FAT occupies
pub fn calculate_fat_size(options: &Options) -> Result<usize, CreateImageError> {
    let extra_clusters = bytes_to_clusters(EXTRA_FREE_SPACE, options);

    let clusters = calculate_directory(options.sysroot(), true, options)? + extra_clusters + 2;

    let clusters_per_fat_sector = options.sector_size() as usize / 4;

    Ok(if clusters < MINIMUM_CLUSTERS {
        MINIMUM_CLUSTERS
    } else {
        clusters
    }
    .div_ceil(clusters_per_fat_sector))
}

fn bytes_to_clusters(bytes: usize, options: &Options) -> usize {
    sectors_to_clusters(bytes.div_ceil(options.sector_size() as usize), options)
}

fn sectors_to_clusters(sectors: usize, options: &Options) -> usize {
    sectors.div_ceil(options.sectors_per_cluster() as usize)
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
                // TODO: Add warning for unnamed file
                continue;
            }
        };

        bytes += DIRECTORY_ENTRY_SIZE
            * match calculate_name_entries(file_name.as_bytes()) {
                Some(entries) => entries,
                None => {
                    // TODO: Add warning for invalid filename
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

// TODO: Allow long filenames
//  This will also allow filenames with different cases
fn calculate_name_entries(name: &[u8]) -> Option<usize> {
    let mut stem_length = 0;
    let mut extension_length = 0;
    let mut extension = false;

    for c in name {
        if stem_length == 0 && (*c == b'.' || *c == b'.') {
            return None;
        }

        if !extension && *c == b'.' {
            extension = true;
            continue;
        }

        if ILLEGAL_SHORT_CHARACTERS.contains(c) {
            return None;
        }

        if extension {
            extension_length += 1;
        } else {
            stem_length += 1;
        }
    }

    if stem_length == 0 || stem_length > 8 || extension_length > 3 {
        None
    } else {
        Some(1)
    }
}
