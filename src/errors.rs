#[derive(Debug)]
pub enum WarpZipError {
    IoError(std::io::Error),
    InvalidFormat,
}

impl std::fmt::Display for WarpZipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WarpZipError::IoError(e) => write!(f, "I/O Error: {}", e),
            WarpZipError::InvalidFormat => write!(f, "Invalid file format"),
        }
    }
}
