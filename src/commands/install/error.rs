#[derive(Debug)]
pub struct InstallError {
    file_name: String,
    error: std::io::Error,
}

impl InstallError {
    pub(super) fn new(file_name: &str, error: std::io::Error) -> Self {
        InstallError {
            file_name: file_name.to_owned(),
            error,
        }
    }
}

impl std::error::Error for InstallError {}

impl std::fmt::Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to install {} - {}", self.file_name, self.error)
    }
}
