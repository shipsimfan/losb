use crate::{commands::Command, output::Output};
use std::path::{Path, PathBuf};

pub struct Options<'a> {
    output: &'a Output,

    command: Command,

    // Common output options
    release: bool,
    path: PathBuf,
    sysroot: PathBuf,
    full_sysroot: PathBuf,
    output_path: PathBuf,

    // Image options
    sector_size: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    num_fats: u8,
    fixed_media: bool,
    volume_id: u32,

    // Run options
    ovmf_location: PathBuf,
    debug_port: u16,
    execute_gdb: bool,
}

const DEFAULT_SYSROOT: &'static str = "sysroot";
const DEFAULT_OUTPUT_PATH: &'static str = "os.img";

const DEFAULT_SECTOR_SIZE: u16 = 512;
const DEFAULT_SECTORS_PER_CLUSTER: u8 = 1;
const DEFAULT_RESERVED_SECTORS: u16 = 32;
const DEFAULT_NUM_FATS: u8 = 2;
const DEFAULT_VOLUME_ID: u32 = 0x0BADBEEF; // TODO: Replace this with a random number

const DEFAULT_OVMF_LOCATION: &'static str = "OVMF.fd";
const DEFAULT_DEBUG_PORT: u16 = 1234;

impl<'a> Options<'a> {
    pub fn new(output: &'a Output) -> Self {
        Options {
            output,

            command: Command::default(),

            // Common options
            path: PathBuf::new(),
            sysroot: PathBuf::from(DEFAULT_SYSROOT),
            full_sysroot: PathBuf::from(DEFAULT_SYSROOT),
            output_path: PathBuf::from(DEFAULT_OUTPUT_PATH),
            release: false,

            // Image options
            sector_size: DEFAULT_SECTOR_SIZE,
            sectors_per_cluster: DEFAULT_SECTORS_PER_CLUSTER,
            reserved_sectors: DEFAULT_RESERVED_SECTORS,
            num_fats: DEFAULT_NUM_FATS,
            fixed_media: true,
            volume_id: DEFAULT_VOLUME_ID,

            // Run options
            ovmf_location: PathBuf::from(DEFAULT_OVMF_LOCATION),
            debug_port: DEFAULT_DEBUG_PORT,
            execute_gdb: true,
        }
    }

    pub fn output(&self) -> &Output {
        &self.output
    }

    pub fn command(&self) -> Command {
        self.command
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn sysroot(&self) -> &Path {
        &self.full_sysroot
    }

    pub fn is_release(&self) -> bool {
        self.release
    }

    pub fn output_path(&self) -> &Path {
        &self.output_path
    }

    pub fn sector_size(&self) -> u16 {
        self.sector_size
    }

    pub fn sectors_per_cluster(&self) -> u8 {
        self.sectors_per_cluster
    }

    pub fn reserved_sectors(&self) -> u16 {
        self.reserved_sectors
    }

    pub fn is_fixed_media(&self) -> bool {
        self.fixed_media
    }

    pub fn num_fats(&self) -> u8 {
        self.num_fats
    }

    pub fn volume_id(&self) -> u32 {
        self.volume_id
    }

    pub fn ovmf_location(&self) -> &Path {
        &self.ovmf_location
    }

    pub fn debug_port(&self) -> u16 {
        self.debug_port
    }

    pub fn execute_gdb(&self) -> bool {
        self.execute_gdb
    }

    pub(super) fn set_command(&mut self, command: Command) {
        self.command = command;
    }

    pub(super) fn set_path(&mut self, path: PathBuf) {
        self.path = path;
        self.update_sysroot();
    }

    pub(super) fn set_sysroot(&mut self, sysroot: PathBuf) {
        self.sysroot = sysroot;
        self.update_sysroot();
    }

    pub(super) fn set_debug(&mut self) {
        self.release = false;
    }

    pub(super) fn set_release(&mut self) {
        self.release = true;
    }

    pub(super) fn set_output_path(&mut self, output_path: PathBuf) {
        self.output_path = output_path;
    }

    pub(super) fn set_sector_size(&mut self, sector_size: u16) {
        assert!([512, 1024, 2048, 4096].contains(&sector_size));
        self.sector_size = sector_size;
    }

    pub(super) fn set_sectors_per_cluster(&mut self, sectors_per_cluster: u8) {
        assert_ne!(sectors_per_cluster, 0);
        assert!(sectors_per_cluster.is_power_of_two());

        self.sectors_per_cluster = sectors_per_cluster;
        // TODO: Add a warning if the sectors_per_cluster * bytes_per_sector > 32 * 1024
    }

    pub(super) fn set_reserved_sectors(&mut self, reserved_sectors: u16) {
        assert!(reserved_sectors >= 2);
        self.reserved_sectors = reserved_sectors;
    }

    pub(super) fn set_num_fats(&mut self, num_fats: u8) {
        assert_ne!(num_fats, 0);
        // TODO: Add warning if the FAT number is not 2
        self.num_fats = num_fats;
    }

    pub(super) fn set_fixed_media(&mut self) {
        self.fixed_media = true;
    }

    pub(super) fn set_removable_media(&mut self) {
        self.fixed_media = false;
    }

    pub(super) fn set_volume_id(&mut self, volume_id: u32) {
        self.volume_id = volume_id;
    }

    pub(super) fn set_ovmf_location(&mut self, location: PathBuf) {
        self.ovmf_location = location;
    }

    pub(super) fn set_debug_port(&mut self, port: u16) {
        self.debug_port = port;
    }

    pub(super) fn set_no_gdb(&mut self) {
        self.execute_gdb = false;
    }

    fn update_sysroot(&mut self) {
        if self.sysroot.is_absolute() {
            self.full_sysroot = self.sysroot.clone();
            return;
        }

        self.full_sysroot = self.path.join(&self.sysroot);
    }
}
