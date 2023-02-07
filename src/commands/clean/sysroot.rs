use crate::args::Options;

#[derive(Debug)]
pub struct CleanError(std::io::Error);

pub fn clean_sysroot(options: &Options) -> Result<(), CleanError> {
    options.output().log_cleaning("sysroot");
    if options.sysroot().exists() {
        std::fs::remove_dir_all(options.sysroot()).map_err(|error| CleanError(error))?;
    }
    options.output().log_finished("cleaning", "sysroot");
    Ok(())
}

impl std::error::Error for CleanError {}

impl std::fmt::Display for CleanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to clean the sysroot - {}", self.0)
    }
}
