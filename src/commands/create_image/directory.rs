use super::{
    calculate::ILLEGAL_SHORT_CHARACTERS, error::CreateImageError, writer::FileWriter, Cluster,
};
use crate::args::Options;
use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[repr(packed)]
#[allow(unused)]
struct DirectoryEntry {
    name: [u8; 11],
    attributes: u8,
    reserved: u8,
    creation_time_tenth: u8,
    creation_time: u16,
    creation_date: u16,
    last_access_date: u16,
    first_cluster_high: u16,
    write_time: u16,
    write_date: u16,
    first_cluster_low: u16,
    file_size: u32,
}

const DOT_NAME: [u8; 11] = *b".          ";
const DOT_DOT_NAME: [u8; 11] = *b"..         ";

const ATTR_SYSTEM: u8 = 0x04;
const ATTR_DIRECTORY: u8 = 0x10;

const MS_DOS_CONVERSION: u128 = 8 * YEAR_MS + 2 * LEAP_YEAR_MS;

const YEAR_MS: u128 = 365 * DAY_MS;
const LEAP_YEAR_MS: u128 = 366 * DAY_MS;
const DAY_MS: u128 = 24 * HOUR_MS;
const HOUR_MS: u128 = 60 * MINUTE_MS;
const MINUTE_MS: u128 = 60 * SECOND_MS;
const SECOND_MS: u128 = 1000;

const YEAR_MONTH_LENGTHS: [u128; 12] = [
    31 * DAY_MS,
    28 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
];
const LEAP_YEAR_MONTH_LENGTHS: [u128; 12] = [
    31 * DAY_MS,
    29 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
];

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
        let cluster = if fat_entries[fat_i].attributes & ATTR_DIRECTORY == ATTR_DIRECTORY {
            write_directory(
                writer,
                path,
                false,
                (
                    fat_entries[fat_i].creation_date,
                    fat_entries[fat_i].creation_time,
                    fat_entries[fat_i].creation_time_tenth,
                ),
                (fat_entries[fat_i].write_date, fat_entries[fat_i].write_time),
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

// TODO: Allow long names
fn calculate_short_name(name: &[u8]) -> Option<[u8; 11]> {
    let mut output = [0x20; 11];
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

        if stem_length == 8 && !extension {
            return None;
        }

        if extension_length == 3 {
            return None;
        }

        if ILLEGAL_SHORT_CHARACTERS.contains(c) {
            return None;
        }

        if extension {
            output[8 + extension_length] = *c;

            extension_length += 1;
        } else {
            output[stem_length] = *c;

            stem_length += 1;
        }
    }

    if stem_length == 0 {
        None
    } else {
        Some(output)
    }
}

fn generate_date_time(time: SystemTime) -> Option<(u16, u16, u8)> {
    let mut millis = match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() - MS_DOS_CONVERSION,
        Err(_) => return None,
    };

    let year = calculate_year(&mut millis);
    let month = calculate_month(&mut millis, year);
    let day = calculate_simple(&mut millis, DAY_MS) + 1;

    let hour = calculate_simple(&mut millis, HOUR_MS);
    let minute = calculate_simple(&mut millis, MINUTE_MS);
    let second = calculate_simple(&mut millis, 2 * SECOND_MS);

    let tenth = calculate_simple(&mut millis, SECOND_MS / 10) as u8;

    let date =
        (day as u16 & 0x1F) | (month as u16 & 0x0F) << 5 | ((year - 1980) as u16 & 0x7F) << 9;
    let time = (second as u16 & 0x1F) | (minute as u16 & 0x1F) << 5 | (hour as u16 & 0x1F) << 10;

    Some((date, time, tenth))
}

fn calculate_year(millis: &mut u128) -> usize {
    let mut year = 1980;
    loop {
        let leap_year = is_leap_year(year);

        let year_millis = if leap_year { LEAP_YEAR_MS } else { YEAR_MS };

        if *millis < year_millis {
            return year;
        }

        *millis -= year_millis;
        year += 1;
    }
}

fn calculate_month(millis: &mut u128, year: usize) -> usize {
    let month_lengths = if is_leap_year(year) {
        LEAP_YEAR_MONTH_LENGTHS
    } else {
        YEAR_MONTH_LENGTHS
    };

    let mut month = 0;
    loop {
        if *millis < month_lengths[month] {
            return month + 1;
        }

        *millis -= month_lengths[month];
        month += 1;
    }
}

fn calculate_simple(millis: &mut u128, unit: u128) -> usize {
    let value = (*millis / unit) as usize;
    *millis = *millis % unit;
    value
}

fn is_leap_year(year: usize) -> bool {
    if year % 4 != 0 {
        return false;
    }

    if year % 400 == 0 {
        return true;
    }

    year % 100 != 0
}

impl DirectoryEntry {
    pub fn new(dir_entry: &DirEntry) -> Option<Self> {
        let name = match dir_entry.path().file_name() {
            Some(file_name) => match calculate_short_name(file_name.to_string_lossy().as_bytes()) {
                Some(name) => name,
                None => return None,
            },
            None => return None,
        };

        let metadata = match dir_entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return None,
        };

        let mut attributes = 0;
        if metadata.is_dir() {
            attributes |= ATTR_DIRECTORY;
        }

        let (creation_date, creation_time, creation_time_tenth) = match metadata.created() {
            Ok(time) => generate_date_time(time).unwrap_or((0, 0, 0)),
            Err(_) => (0, 0, 0),
        };

        let now = SystemTime::now();
        let (now_date, now_time, _) = generate_date_time(now).unwrap_or((0, 0, 0));

        Some(DirectoryEntry {
            name,
            attributes,
            reserved: 0,
            creation_time_tenth,
            creation_time,
            creation_date,
            last_access_date: now_date,
            first_cluster_high: 0,
            write_time: now_time,
            write_date: now_date,
            first_cluster_low: 0,
            file_size: if metadata.is_dir() {
                0
            } else {
                metadata.len() as u32
            },
        })
    }

    pub fn new_dot(creation: (u16, u16, u8), write: (u16, u16)) -> Self {
        DirectoryEntry {
            name: DOT_NAME,
            attributes: ATTR_DIRECTORY | ATTR_SYSTEM,
            reserved: 0,
            creation_time_tenth: creation.2,
            creation_time: creation.1,
            creation_date: creation.0,
            last_access_date: write.0,
            first_cluster_high: 0,
            write_time: write.1,
            write_date: write.0,
            first_cluster_low: 0,
            file_size: 0,
        }
    }

    pub fn new_dot_dot(creation: (u16, u16, u8), write: (u16, u16)) -> Self {
        DirectoryEntry {
            name: DOT_DOT_NAME,
            attributes: ATTR_DIRECTORY | ATTR_SYSTEM,
            reserved: 0,
            creation_time_tenth: creation.2,
            creation_time: creation.1,
            creation_date: creation.0,
            last_access_date: write.0,
            first_cluster_high: 0,
            write_time: write.1,
            write_date: write.0,
            first_cluster_low: 0,
            file_size: 0,
        }
    }

    pub fn set_root_cluster(&mut self, cluster: Cluster) {
        self.first_cluster_high = (cluster >> 16) as u16;
        self.first_cluster_low = (cluster & 0xFFFF) as u16;
    }
}
