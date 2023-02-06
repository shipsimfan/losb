use super::{error::CreateImageError, file::File, Cluster};
use crate::args::Options;

#[repr(packed)]
#[allow(unused)]
struct FSInfo {
    lead_signature: u32,
    reserved_1: [u8; 480],
    struct_signature: u32,
    free_count: u32,
    next_free: u32,
    reserved_2: [u8; 12],
    trail_signature: u32,
}

pub const FS_INFO_SECTOR: usize = 1;

const FREE_COUNT_OFFSET: usize = 488;
const NEXT_FREE_OFFSET: usize = 492;

const LEAD_SIGNATURE: u32 = 0x41615252;
const STRUCT_SIGNATURE: u32 = 0x61417272;
const TRAIL_SIGNATURE: u32 = 0xAA550000;

pub fn write_fs_info(output: &mut File, options: &Options) -> Result<(), CreateImageError> {
    // Seek to the first sector
    output.seek(options.sector_size() as usize * FS_INFO_SECTOR)?;

    // Write fs_info
    let fs_info = FSInfo::new();
    output.write_struct(&fs_info)?;

    // Zero remaining space
    output.write_zeros(options.sector_size() as usize - std::mem::size_of::<FSInfo>())
}

pub fn write_free_cluster_info(
    output: &mut File,
    next_free_cluster: Cluster,
    free_clusters: usize,
    options: &Options,
) -> Result<(), CreateImageError> {
    output.seek(options.sector_size() as usize * FS_INFO_SECTOR + FREE_COUNT_OFFSET)?;
    output.write(&(free_clusters as u32).to_le_bytes())?;

    output.seek(options.sector_size() as usize * FS_INFO_SECTOR + NEXT_FREE_OFFSET)?;
    output.write(&next_free_cluster.to_le_bytes())
}

impl FSInfo {
    pub fn new() -> Self {
        FSInfo {
            lead_signature: LEAD_SIGNATURE,
            reserved_1: [0; 480],
            struct_signature: STRUCT_SIGNATURE,
            free_count: 0,
            next_free: 0,
            reserved_2: [0; 12],
            trail_signature: TRAIL_SIGNATURE,
        }
    }
}
