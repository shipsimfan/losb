use std::{
    env::current_dir,
    fs::{copy, create_dir_all},
    path::Path,
    process::{Command, Stdio},
};

fn build_cargo(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("cargo");
    command.arg("build");
    command.current_dir(path);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.output()?;
    Ok(())
}

fn install_build(
    path: &str,
    prefix: &Path,
    sysroot: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("build");
    command.args([
        "install",
        "--sysroot",
        sysroot.to_str().unwrap(),
        "--prefix",
        prefix.to_str().unwrap(),
    ]);
    command.current_dir(path);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.output()?;
    Ok(())
}

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare sysroot
    create_dir_all("./sysroot/los/bin")?;
    create_dir_all("./sysroot/los/lib")?;
    create_dir_all("./sysroot/los/include")?;
    create_dir_all("./sysroot/EFI/BOOT")?;

    // Build bootloader
    println!("Building bootloader . . .");
    build_cargo("./bootloader")?;

    // Install bootloader
    println!("\nInstalling bootloader . . .");
    copy(
        "./bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi",
        "./sysroot/EFI/BOOT/BOOTX64.EFI",
    )?;

    // Build kernel
    println!("\nBuilding kernel . . .");
    build_cargo("./kernel")?;

    // Install kernel
    println!("\nInstalling kernel . . .");
    copy(
        "./kernel/target/x86_64-los/debug/kernel",
        "./sysroot/kernel.elf",
    )?;

    // Prepare prefix and sysroot
    let current_path = current_dir()?;
    let prefix = current_path.join("sysroot/los");
    let sysroot = current_path.join("sysroot");

    // Build and install libraries
    println!("\nBuilding libraries . . .");
    install_build("./libraries", &prefix, &sysroot)?;

    // Build and install programs
    println!("\nBuilding programs . . .");
    install_build("./programs", &prefix, &sysroot)?;

    Ok(())
}
