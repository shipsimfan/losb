use std::path::PathBuf;

#[derive(Debug)]
pub enum CreateImageError {
    CalculateError(PathBuf, std::io::Error),
    ReadError(PathBuf, std::io::Error),
    WriteError(std::io::Error),
}

impl std::error::Error for CreateImageError {}

impl std::fmt::Display for CreateImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateImageError::CalculateError(path, error) => write!(
                f,
                "Unable to calculate image size for \"{}\" - {}",
                path.display(),
                error
            ),
            CreateImageError::ReadError(path, error) => {
                write!(f, "Unable to read \"{}\" - {}", path.display(), error)
            }
            CreateImageError::WriteError(error) => {
                write!(f, "Unable to write to ouput image - {}", error)
            }
        }
    }
}
