#[derive(Debug)]
pub enum Error {
    BuildError(std::process::ExitStatus),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::BuildError(status) =>
                    format!("Build failed with status {}", status.code().unwrap_or(1)),
            }
        )
    }
}
