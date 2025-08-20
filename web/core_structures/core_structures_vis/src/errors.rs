//! Submodule providing errors which may occur in the core structures
//! visualization module.

#[derive(Debug)]
/// Error type for the core structures visualization module.
pub enum Error {
    /// Error related to Mermaid Entity Relationship Diagrams.
    MermaidERDError(mermaid::ERDiagramError),
    /// Error related to Diesel database operations.
    Diesel(diesel::result::Error),
}

impl From<mermaid::ERDiagramError> for Error {
    fn from(err: mermaid::ERDiagramError) -> Self {
        Error::MermaidERDError(err)
    }
}

impl From<mermaid::ERDiagramConfigError> for Error {
    fn from(err: mermaid::ERDiagramConfigError) -> Self {
        Error::MermaidERDError(err.into())
    }
}

impl From<mermaid::ERDiagramNodeError> for Error {
    fn from(err: mermaid::ERDiagramNodeError) -> Self {
        Error::MermaidERDError(err.into())
    }
}

impl From<mermaid::ERDiagramEdgeError> for Error {
    fn from(err: mermaid::ERDiagramEdgeError) -> Self {
        Error::MermaidERDError(err.into())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Diesel(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MermaidERDError(err) => write!(f, "Mermaid ERD error: {}", err),
            Error::Diesel(err) => write!(f, "Diesel error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::MermaidERDError(err) => Some(err),
            Error::Diesel(err) => Some(err),
        }
    }
}
