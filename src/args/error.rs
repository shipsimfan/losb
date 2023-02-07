#[derive(Debug)]
pub enum ArgumentError {
    UnknownCommand(String),
    InvalidSectorSize(String),
    InvalidClusterSize(String),
    InvalidReservedSectors,
    InvalidNumFats,
    InvalidVolumeID,
    InvalidDebugPort(String),
}

impl std::error::Error for ArgumentError {}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentError::UnknownCommand(command) => write!(f, "Unknown command \"{command}\""),
            ArgumentError::InvalidSectorSize(sector_size) => write!(
                f,
                "\"{}\" is an invalid sector size. Only 512, 1024, 2048, and 4096 are legal values",
                sector_size
            ),
            ArgumentError::InvalidClusterSize(cluster_size) => write!(
                f,
                "\"{}\" is an invalid cluster size. Only powers of two less than 255 are legal values",
                cluster_size
            ),
            ArgumentError::InvalidReservedSectors => write!(f, "Reserved sectors must be at least 2"),
            ArgumentError::InvalidNumFats => write!(f, "Number of FATs must be at least 1"),
            ArgumentError::InvalidVolumeID => write!(f, "Volume ID must be a number <= {}", u32::MAX),
            ArgumentError::InvalidDebugPort(port) => write!(f, "\"{}\" is an invalid port", port),
        }
    }
}
