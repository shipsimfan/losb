use std::path::Path;

pub fn create_image(volume_size: usize, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    print!(
        "    \x1B[34;1mCreating\x1B[0m {} ({} MB) . . .",
        target.to_string_lossy(),
        volume_size / 1024 / 1024,
    );

    Ok(println!(
        "\r    \x1B[32;1mFinished\x1B[0m creating {} ({} MB)",
        target.to_string_lossy(),
        volume_size / 1024 / 1024,
    ))
}
