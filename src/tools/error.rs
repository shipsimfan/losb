#[derive(Debug)]
pub enum ToolError {
    SpawnError(&'static str, std::io::Error),
    RuntimeError(&'static str),
}

impl std::error::Error for ToolError {}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::SpawnError(tool, error) => {
                write!(f, "Unable to launch {} - {}", tool, error)
            }
            ToolError::RuntimeError(tool) => write!(f, "{} did not complete successfully", tool),
        }
    }
}
