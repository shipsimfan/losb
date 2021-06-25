#![allow(dead_code)]

#[repr(packed(1))]
pub struct BIOSParameterBlock {
    bs_jump_boot: [u8; 3],
    bs_oem_name: [u8; 8],
    bpb_bytes_per_sector: u16,
    bpb_sectors_per_cluster: u8,
    bpb_reserved_sector_count: u16,
    bpb_num_fats: u8,
    bpb_root_entry_count: u16,
    bpb_total_sectors_16: u16,
    bpb_media: u8,
    bpb_fat_size_16: u16,
    bpb_sectors_per_track: u16,
    bpb_number_of_heads: u16,
    bpb_hidden_sector: u32,
    bpb_total_sectors_32: u32,
    bpb_fat_size_32: u32,
    bpb_extended_flags: u16,
    bpb_fs_version: u16,
    bpb_root_cluster: u32,
    bpb_fs_info: u16,
    bpb_backup_boot_sector: u16,
    bpb_reserved: [u8; 12],
    bs_drive_number: u8,
    bs_reserved: u8,
    bs_boot_signature: u8,
    bs_volume_id: u32,
    bs_volume_label: [u8; 11],
    bs_filesystem_type: [u8; 8],
}

#[repr(packed(1))]
pub struct FSInfo {
    fsi_lead_signature: u32,
    fsi_reserved_1: [u8; 480],
    fsi_structure_signature: u32,
    fsi_free_count: u32,
    fsi_next_free: u32,
    fsi_reserved_2: [u8; 12],
    fsi_trail_signature: u32,
}

#[repr(packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct DirectoryEntry {
    name: [u8; 11],
    attribute: u8,
    nt_reserved: u8,
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

pub const BYTES_PER_SECTOR: usize = 512;

pub const RESERVED_SECTOR_COUNT: usize = 32;
const SECTORS_PER_CLUSTER: usize = 1;
pub const NUM_FATS: usize = 2;

pub const ATTR_READ_ONLY: u8 = 0x01;
pub const ATTR_HIDDEN: u8 = 0x02;
pub const ATTR_SYSTEM: u8 = 0x04;
pub const ATTR_VOLUME_ID: u8 = 0x08;
pub const ATTR_DIRECTORY: u8 = 0x10;
pub const ATTR_ARCHIVE: u8 = 0x20;
pub const ATTR_LONG_NAME: u8 = ATTR_READ_ONLY | ATTR_HIDDEN | ATTR_SYSTEM | ATTR_VOLUME_ID;

impl BIOSParameterBlock {
    pub fn new(volume_size: usize) -> Self {
        let num_sectors = volume_size / BYTES_PER_SECTOR;
        let tmp_val_1 = num_sectors - RESERVED_SECTOR_COUNT;
        let tmp_val_2 = (256 * SECTORS_PER_CLUSTER) + NUM_FATS;
        let tmp_val_2 = tmp_val_2 / 2;
        let fat_size = (tmp_val_1 + (tmp_val_2 - 1)) / tmp_val_2;

        BIOSParameterBlock {
            bs_jump_boot: [0xEB, 0xFC, 0x90],
            bs_oem_name: [
                'M' as u8, 'S' as u8, 'W' as u8, 'I' as u8, 'N' as u8, '4' as u8, '.' as u8,
                '1' as u8,
            ],
            bpb_bytes_per_sector: BYTES_PER_SECTOR as u16,
            bpb_sectors_per_cluster: SECTORS_PER_CLUSTER as u8,
            bpb_reserved_sector_count: RESERVED_SECTOR_COUNT as u16,
            bpb_num_fats: NUM_FATS as u8,
            bpb_root_entry_count: 0,
            bpb_total_sectors_16: 0,
            bpb_media: 0xF0,
            bpb_fat_size_16: 0,
            bpb_sectors_per_track: (num_sectors / 16) as u16,
            bpb_number_of_heads: 16,
            bpb_hidden_sector: 0,
            bpb_total_sectors_32: num_sectors as u32,
            bpb_fat_size_32: fat_size as u32,
            bpb_extended_flags: 0,
            bpb_fs_version: 0,
            bpb_root_cluster: 2,
            bpb_fs_info: 1,
            bpb_backup_boot_sector: 6,
            bpb_reserved: [0; 12],
            bs_drive_number: 0,
            bs_reserved: 0,
            bs_boot_signature: 0x29,
            bs_volume_id: 0x0BADC0DE,
            bs_volume_label: [
                'L' as u8, 'a' as u8, 'n' as u8, 'c' as u8, 'e' as u8, ' ' as u8, 'O' as u8,
                'S' as u8, ' ' as u8, ' ' as u8, ' ' as u8,
            ],
            bs_filesystem_type: [
                'F' as u8, 'A' as u8, 'T' as u8, '3' as u8, '2' as u8, ' ' as u8, ' ' as u8,
                ' ' as u8,
            ],
        }
    }

    pub fn fat_size(&self) -> usize {
        if self.bpb_fat_size_32 != 0 {
            self.bpb_fat_size_32 as usize
        } else {
            self.bpb_fat_size_16 as usize
        }
    }

    pub fn volume_label(&self) -> &[u8; 11] {
        &self.bs_volume_label
    }

    pub fn first_data_sector(&self) -> usize {
        (self.bpb_reserved_sector_count as u32 + self.bpb_num_fats as u32 * self.bpb_fat_size_32)
            as usize
    }

    pub fn num_fats(&self) -> usize {
        self.bpb_num_fats as usize
    }

    pub fn reserved_sectors(&self) -> usize {
        self.bpb_reserved_sector_count as usize
    }
}

impl FSInfo {
    pub fn new() -> Self {
        FSInfo {
            fsi_lead_signature: 0x41615252,
            fsi_reserved_1: [0; 480],
            fsi_structure_signature: 0x61417272,
            fsi_free_count: 0xFFFFFFFF,
            fsi_next_free: 3,
            fsi_reserved_2: [0; 12],
            fsi_trail_signature: 0xAA550000,
        }
    }
}

impl DirectoryEntry {
    pub fn new(name: [u8; 11], attribute: u8, first_cluster: u32, file_size: u32) -> Self {
        let first_cluster_low = (first_cluster & 0xFFFF) as u16;
        let first_cluster_high = (first_cluster.wrapping_shr(16) & 0xFFFF) as u16;

        DirectoryEntry {
            name: name,
            attribute: attribute,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: first_cluster_high,
            first_cluster_low: first_cluster_low,
            write_time: 0,
            write_date: 0,
            file_size: file_size,
        }
    }

    pub const fn zero() -> Self {
        DirectoryEntry {
            name: [0; 11],
            attribute: 0,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: 0,
            first_cluster_low: 0,
            write_time: 0,
            write_date: 0,
            file_size: 0,
        }
    }
}
