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

fn clean_brew(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("brew");
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
    println!("    \x1B[32;1mCleaning\x1B[0m sysroot . . .");
    if Path::new("./sysroot").exists() {
        remove_dir_all("./sysroot")?;
    }

    // Clean libraries
    println!("    \x1B[32;1mCleaning\x1B[0m libraries . . .");
    clean_brew("./libraries")?;

    // Clean programs
    println!("    \x1B[32;1mCleaning\x1B[0m programs . . .");
    clean_brew("./programs")?;

    Ok(())
}

pub fn clean() -> Result<(), Box<dyn std::error::Error>> {
    // Clean bootloader
    println!("    \x1B[32;1mCleaning\x1B[0m bootloader . . .");
    clean_cargo("./bootloader")?;

    // Clean kernel
    println!("    \x1B[32;1mCleaning\x1B[0m kernel . . .");
    clean_cargo("./kernel")?;

    // Clean user
    clean_user()
}
