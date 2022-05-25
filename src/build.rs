use std::{
    env::current_dir,
    fs::{copy, create_dir_all},
    path::Path,
    process::Command,
};

#[derive(Debug)]
pub enum BuildError {
    Cargo(String, Option<std::io::Error>),
    Brew(String, Option<std::io::Error>),
    PrepareSysroot(std::io::Error),
    InstallError(String, std::io::Error),
}

fn build_cargo(path: &str) -> Result<(), BuildError> {
    let mut command = Command::new("cargo");
    command.arg("build");
    command.current_dir(path);

    match command.status() {
        Ok(status) => match status.success() {
            true => Ok(()),
            false => Err(BuildError::Cargo(path.to_owned(), None)),
        },
        Err(error) => Err(BuildError::Cargo(path.to_owned(), Some(error))),
    }
}

fn install_build(path: &str, prefix: &Path, sysroot: &Path) -> Result<(), BuildError> {
    let mut command = Command::new("brew");
    command.args([
        "install",
        "--sysroot",
        sysroot.to_str().unwrap(),
        "--prefix",
        prefix.to_str().unwrap(),
    ]);
    command.current_dir(path);

    match command.status() {
        Ok(status) => match status.success() {
            true => Ok(()),
            false => Err(BuildError::Brew(path.to_string(), None)),
        },
        Err(error) => Err(BuildError::Brew(path.to_string(), Some(error))),
    }
}

fn prepare_sysroot() -> Result<(), std::io::Error> {
    create_dir_all("./sysroot/los/bin")?;
    create_dir_all("./sysroot/los/lib")?;
    create_dir_all("./sysroot/los/include")?;
    create_dir_all("./sysroot/EFI/BOOT")
}

fn install(from: &str, to: &str) -> Result<(), BuildError> {
    match copy(from, to) {
        Ok(_) => Ok(()),
        Err(error) => Err(BuildError::InstallError(from.to_string(), error)),
    }
}

pub fn build() -> Result<(), BuildError> {
    // Prepare sysroot
    match prepare_sysroot() {
        Ok(()) => {}
        Err(error) => return Err(BuildError::PrepareSysroot(error)),
    }

    // Build bootloader
    println!("    \x1B[32;1mBuilding\x1B[0m bootloader . . .");
    build_cargo("./bootloader")?;

    // Install bootloader
    println!("\n  \x1B[32;1mInstalling\x1B[0m bootloader . . .");
    install(
        "./bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi",
        "./sysroot/EFI/BOOT/BOOTX64.EFI",
    )?;

    // Build kernel
    println!("\n    \x1B[32;1mBuilding\x1B[0m kernel . . .");
    build_cargo("./kernel")?;

    // Install kernel
    println!("\n  \x1B[32;1mInstalling\x1B[0m kernel . . .");
    install(
        "./kernel/target/x86_64-los/debug/init",
        "./sysroot/kernel.elf",
    )?;

    // Prepare prefix and sysroot
    let current_path = current_dir().expect("No current working directory!");
    let prefix = current_path.join("sysroot/los");
    let sysroot = current_path.join("sysroot");

    // Build and install libraries
    println!("\n    \x1B[32;1mBuilding\x1B[0m libraries . . .");
    install_build("./libraries", &prefix, &sysroot)?;

    // Build and install programs
    println!("\n    \x1B[32;1mBuilding\x1B[0m programs . . .");
    install_build("./programs", &prefix, &sysroot)?;

    Ok(())
}

impl std::error::Error for BuildError {}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BuildError::Cargo(path, error) => format!(
                    "Unable to build {} using cargo{}",
                    path,
                    match error {
                        Some(error) => format!(" ({})", error),
                        None => String::new(),
                    }
                ),
                BuildError::Brew(path, error) => format!(
                    "Unable to build {} using brew{}",
                    path,
                    match error {
                        Some(error) => format!(" ({})", error),
                        None => String::new(),
                    }
                ),
                BuildError::PrepareSysroot(error) =>
                    format!("Unable to prepare sysroot ({})", error),
                BuildError::InstallError(path, error) =>
                    format!("Unable to install {} into sysroot ({})", path, error),
            }
        )
    }
}
