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
    println!("    \x1B[32;1mBuilding\x1B[0m bootloader . . .");
    build_cargo("./bootloader")?;

    // Install bootloader
    println!("\n  \x1B[32;1mInstalling\x1B[0m bootloader . . .");
    copy(
        "./bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi",
        "./sysroot/EFI/BOOT/BOOTX64.EFI",
    )?;

    // Build kernel
    println!("\n    \x1B[32;1mBuilding\x1B[0m kernel . . .");
    build_cargo("./kernel")?;

    // Install kernel
    println!("\n  \x1B[32;1mInstalling\x1B[0m kernel . . .");
    copy(
        "./kernel/target/x86_64-los/debug/kernel",
        "./sysroot/kernel.elf",
    )?;

    // Prepare prefix and sysroot
    let current_path = current_dir()?;
    let prefix = current_path.join("sysroot/los");
    let sysroot = current_path.join("sysroot");

    // Build and install libraries
    println!("\n    \x1B[32;1mBuilding\x1B[0m libraries . . .");
    install_build("./libraries", &prefix, &sysroot)?;

    // Build and install programs
    println!("\n    \x1B[32;1mBuilding\x1B[0m programs . . .");
    install_build("./programs", &prefix, &sysroot)?;

    // Copy core libraries to rustlib
    create_dir_all("./sysroot/los/lib/rustlib/x86_64-los/lib")?;
    copy("./sysroot/los/lib/crt0.o", "./sysroot/los/lib/rustlib/x86_64-los/lib/crt0.o")?;
    copy("./sysroot/los/lib/crti.o", "./sysroot/los/lib/rustlib/x86_64-los/lib/crti.o")?;
    copy("./sysroot/los/lib/crtn.o", "./sysroot/los/lib/rustlib/x86_64-los/lib/crtn.o")?;
    copy("./sysroot/los/lib/libkernel.a", "./sysroot/los/lib/rustlib/x86_64-los/lib/libkernel.a")?;
    copy("./sysroot/los/lib/libc.a", "./sysroot/los/lib/rustlib/x86_64-los/lib/libc.a")?;

    Ok(())
}
