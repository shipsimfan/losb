pub fn clean_user() -> Result<(), Box<dyn std::error::Error>> {
    // Remove sysroot

    // Clean libraries

    // Clean programs

    Ok(())
}

pub fn clean() -> Result<(), Box<dyn std::error::Error>> {
    // Clean kernel and bootloader

    // Clean user
    clean_user()
}
