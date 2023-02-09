use super::date_time::generate_date_time;
use crate::commands::create_image::Cluster;
use std::{fs::DirEntry, time::SystemTime};

pub enum Name {
    Long(Vec<u16>, [u8; SHORT_NAME_LENGTH]),
    Short([u8; SHORT_NAME_LENGTH]),
}

#[repr(packed)]
#[allow(unused)]
pub struct DirectoryEntry {
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

const ATTR_SYSTEM: u8 = 0x04;
const ATTR_DIRECTORY: u8 = 0x10;

const DOT_NAME: [u8; SHORT_NAME_LENGTH] = *b".          ";
const DOT_DOT_NAME: [u8; SHORT_NAME_LENGTH] = *b"..         ";

const SHORT_NAME_LENGTH: usize = 11;

pub const ILLEGAL_SHORT_CHARACTERS: &[u8] = &[
    0x22, 0x2A, 0x2B, 0x2C, 0x2E, 0x2F, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x5B, 0x5C, 0x5D, 0x7C,
];

// TODO: Allow long names
pub fn calculate_short_name(name: &[u8]) -> Option<[u8; 11]> {
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

    pub fn is_directory(&self) -> bool {
        self.attributes & ATTR_DIRECTORY == ATTR_DIRECTORY
    }

    pub fn creation(&self) -> (u16, u16, u8) {
        (
            self.creation_date,
            self.creation_time,
            self.creation_time_tenth,
        )
    }

    pub fn write(&self) -> (u16, u16) {
        (self.write_date, self.write_time)
    }

    pub fn set_root_cluster(&mut self, cluster: Cluster) {
        self.first_cluster_high = (cluster >> 16) as u16;
        self.first_cluster_low = (cluster & 0xFFFF) as u16;
    }
}
