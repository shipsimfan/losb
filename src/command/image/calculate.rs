use std::path::Path;

// Calculates the volume size in bytes of the volume needed to hold the directory
pub fn volume_size(directory_path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    print!(
        " \x1B[36;1mCalculating\x1B[0m volume size for {} . . .",
        directory_path.to_string_lossy()
    );

    // TODO: Actually calculate the required size

    println!(
        "\r    \x1B[32;1mFinished\x1B[0m calculating volume size for {}",
        directory_path.to_string_lossy()
    );

    // TEMPORARY: returns the minimum volume size of 64 MB
    Ok(64 * 1024 * 1024)
}
