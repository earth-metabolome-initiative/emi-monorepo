//! Submodule handling `From` trait implementations relative to ER Diagram
//! errors.

use super::Error;

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
