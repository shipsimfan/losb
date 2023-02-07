use super::install_file;
use crate::{
    args::Options,
    commands::{build::build_bootloader, BOOTLOADER_NAME},
};

const BOOTLOADER_RELEASE_PATH: &'static str =
    "bootloader/target/x86_64-unknown-uefi/release/bootloader.efi";
const BOOTLOADER_DEBUG_PATH: &'static str =
    "bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi";

const BOOTLOADER_DESTINATION_PATH: &'static str = "EFI/BOOT/BOOTX64.EFI";

const BOOTLOADER_FILENAME: &'static str = "BOOTX64.EFI";

pub fn install_bootloader(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    build_bootloader(options)?;

    options.output().log_installing(BOOTLOADER_NAME);
    Ok(install_file(
        if options.is_release() {
            BOOTLOADER_RELEASE_PATH
        } else {
            BOOTLOADER_DEBUG_PATH
        },
        BOOTLOADER_DESTINATION_PATH,
        BOOTLOADER_FILENAME,
        options,
    )?)
}
