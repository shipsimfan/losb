pub fn display_version() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    Ok(())
}
