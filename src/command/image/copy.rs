use std::path::Path;

pub fn copy_directory(
    target_image: &Path,
    source_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    print!(
        "     \x1B[34;1mCopying\x1B[0m {} into {} . . .",
        source_path.to_string_lossy(),
        target_image.to_string_lossy()
    );

    Ok(println!(
        "\r    \x1B[32;1mFinished\x1B[0m copying {} into {}",
        source_path.to_string_lossy(),
        target_image.to_string_lossy()
    ))
}
