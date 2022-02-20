use std::{fs::remove_dir_all, path::Path, process::Command};

#[derive(Debug)]
pub struct CleanError(String, Option<std::io::Error>);

fn clean_cargo(path: &str) -> Result<(), CleanError> {
    let mut command = Command::new("cargo");
    command.arg("clean");
    command.current_dir(path);
    match command.status() {
        Ok(status) => match status.success() {
            true => Ok(()),
            false => Err(CleanError(path.to_owned(), None)),
        },
        Err(error) => Err(CleanError(path.to_owned(), Some(error))),
    }
}

fn clean_brew(path: &str) -> Result<(), CleanError> {
    let mut command = Command::new("brew");
    command.arg("clean");
    command.current_dir(path);

    match command.status() {
        Ok(status) => match status.success() {
            true => Ok(()),
            false => Err(CleanError(path.to_owned(), None)),
        },
        Err(error) => Err(CleanError(path.to_owned(), Some(error))),
    }
}

pub fn clean_user() -> Result<(), CleanError> {
    // Remove sysroot
    println!("    \x1B[32;1mCleaning\x1B[0m sysroot . . .");
    if Path::new("./sysroot").exists() {
        match remove_dir_all("./sysroot") {
            Ok(_) => {}
            Err(error) => return Err(CleanError("./sysroot".to_owned(), Some(error))),
        };
    }

    // Clean libraries
    println!("    \x1B[32;1mCleaning\x1B[0m libraries . . .");
    clean_brew("./libraries")?;

    // Clean programs
    println!("    \x1B[32;1mCleaning\x1B[0m programs . . .");
    clean_brew("./programs")?;

    Ok(())
}

pub fn clean() -> Result<(), CleanError> {
    // Clean bootloader
    println!("    \x1B[32;1mCleaning\x1B[0m bootloader . . .");
    clean_cargo("./bootloader")?;

    // Clean kernel
    println!("    \x1B[32;1mCleaning\x1B[0m kernel . . .");
    clean_cargo("./kernel")?;

    // Clean user
    clean_user()
}

impl std::error::Error for CleanError {}

impl std::fmt::Display for CleanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to clean {}{}",
            self.0,
            match &self.1 {
                Some(error) => format!(" ({})", error),
                None => String::new(),
            }
        )
    }
}
