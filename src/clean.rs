use std::{
    fs::remove_dir_all,
    path::Path,
    process::{Command, Stdio},
};

fn clean_cargo(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("cargo");
    command.arg("clean");
    command.current_dir(path);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.output()?;
    Ok(())
}

fn clean_build(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("build");
    command.arg("clean");
    command.current_dir(path);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.output()?;
    Ok(())
}

pub fn clean_user() -> Result<(), Box<dyn std::error::Error>> {
    // Remove sysroot
    println!("Cleaning sysroot . . .");
    if Path::new("./sysroot").exists() {
        remove_dir_all("./sysroot")?;
    }

    // Clean libraries
    println!("Cleaning libraries . . .");
    clean_build("./libraries")?;

    // Clean programs
    println!("Cleaning programs . . .");
    clean_build("./programs")?;

    Ok(())
}

pub fn clean() -> Result<(), Box<dyn std::error::Error>> {
    // Clean bootloader
    println!("Cleaning bootloader . . .");
    clean_cargo("./bootloader")?;

    // Clean kernel
    println!("Cleaning kernel . . .");
    clean_cargo("./kernel")?;

    // Clean user
    clean_user()
}
