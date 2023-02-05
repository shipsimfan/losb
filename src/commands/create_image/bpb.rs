use super::error::CreateImageError;
use crate::args::Options;
use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
};

#[repr(packed)]
#[allow(unused)]
struct BIOSParameterBlock {
    // Base
    jmp_boot: [u8; 3],
    oem_name: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    root_entry_count: u16,
    total_sectors_16: u16,
    media: u8,
    fat_size_16: u16,
    sectors_per_track: u16,
    num_heads: u16,
    hidden_sectors: u32,
    total_sectors: u32,

    // FAT 32 Extension
    fat_size_32: u32,
    extended_flags: u16,
    fs_version: u16,
    root_cluster: u32,
    fs_info: u16,
    boot_sector_backup: u16,
    reserved: [u8; 12],
    drive_number: u8,
    reserved_1: u8,
    boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    file_system_type: [u8; 8],
}

const INITIAL_SPACE: usize = std::mem::size_of::<BIOSParameterBlock>();
const FINAL_SIGNATURE_OFFSET: usize = 510 - INITIAL_SPACE;

const JMP_BOOT: [u8; 3] = [0xEB, INITIAL_SPACE as u8, 0x90];
const OEM_NAME: [u8; 8] = *b"LOSB 1.0";
const FS_INFO_SECTOR: u16 = 1;
const BOOT_SIGNATURE: u8 = 0x29;
const VOLUME_LABEL: [u8; 11] = *b"LanceOS 1.0";
const FILE_SYSTEM_TYPE: [u8; 8] = *b"FAT32   ";

const FINAL_SIGNATURE: [u8; 2] = [0x55, 0xAA];

const FIXED_MEDIA: u8 = 0xF8;
const REMOVABLE_MEDIA: u8 = 0xF0;

pub fn write_bpb(
    image_size: usize,
    fat_size: usize,
    output: &mut File,
    options: &Options,
) -> Result<(), CreateImageError> {
    // Seek to the start
    output
        .seek(SeekFrom::Start(0))
        .map_err(|error| CreateImageError::WriteError(error))?;

    // Write bpb
    let bpb = BIOSParameterBlock::new(image_size, fat_size, options);
    output
        .write_all(struct_to_slice(&bpb))
        .map_err(|error| CreateImageError::WriteError(error))?;

    // TODO: Write the boot stub

    // Zero remaining space except for final signature
    let mut remaining = vec![0; options.sector_size() as usize - INITIAL_SPACE];
    remaining[FINAL_SIGNATURE_OFFSET..FINAL_SIGNATURE_OFFSET + 1].copy_from_slice(&FINAL_SIGNATURE);
    output
        .write_all(&remaining)
        .map_err(|error| CreateImageError::WriteError(error))
}

fn struct_to_slice<T: Sized>(value: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(value as *const T as *const u8, std::mem::size_of::<T>()) }
}

impl BIOSParameterBlock {
    pub fn new(image_size: usize, fat_size: usize, options: &Options) -> Self {
        BIOSParameterBlock {
            // Base
            jmp_boot: JMP_BOOT,
            oem_name: OEM_NAME,
            bytes_per_sector: options.sector_size(),
            sectors_per_cluster: options.sectors_per_cluster(),
            reserved_sectors: options.reserved_sectors(),
            root_entry_count: 0, // Must be zero for FAT 32 systems
            total_sectors_16: 0, // Must be zero for FAT 32 systems
            media: if options.is_fixed_media() {
                FIXED_MEDIA
            } else {
                REMOVABLE_MEDIA
            },
            fat_size_16: 0,       // Must be zero for FAT 32 systems
            sectors_per_track: 0, // Only used in real mode
            num_heads: 0,         // Only used in real mode
            hidden_sectors: 0,    // Only used in real mode
            total_sectors: image_size as u32 / options.sector_size() as u32,

            // FAT 32 Extension
            fat_size_32: fat_size as u32 / options.sector_size() as u32,
            extended_flags: 0, // Sets to use runtime mirroring
            fs_version: 0,
            root_cluster: 0, // To be set later once the root directory is written
            fs_info: FS_INFO_SECTOR,
            boot_sector_backup: 0, // Not supported
            reserved: [0; 12],
            drive_number: 0, // Only used in real mode
            reserved_1: 0,
            boot_signature: BOOT_SIGNATURE,
            volume_id: options.volume_id(),
            volume_label: VOLUME_LABEL,
            file_system_type: FILE_SYSTEM_TYPE,
        }
    }
}
