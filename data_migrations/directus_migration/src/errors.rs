#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    SampleID(SampleIDError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SampleID(e) => write!(f, "Sample ID error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SampleID(e) => Some(e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SampleIDError {
    Empty,
    InvalidFormat,
}

impl std::fmt::Display for SampleIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleIDError::Empty => write!(f, "Sample ID is empty"),
            SampleIDError::InvalidFormat => write!(f, "Sample ID has an invalid format"),
        }
    }
}

impl std::error::Error for SampleIDError {}
