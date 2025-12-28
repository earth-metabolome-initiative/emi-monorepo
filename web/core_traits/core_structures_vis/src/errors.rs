//! Submodule providing errors which may occur in the core structures
//! visualization module.

mod er_diagram;
mod flowchart;

#[derive(Debug)]
/// Error type for the core structures visualization module.
pub enum Error {
    /// Error related to Mermaid Entity Relationship Diagrams.
    MermaidERDError(mermaid::ERDiagramError),
    /// Error related to Mermaid Flowchart Diagrams.
    MermaidFlowchartError(mermaid::FlowchartError),
    /// Error related to Diesel database operations.
    Diesel(diesel::result::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MermaidERDError(err) => write!(f, "Mermaid ERD error: {err}"),
            Error::MermaidFlowchartError(err) => write!(f, "Mermaid Flowchart error: {err}"),
            Error::Diesel(err) => write!(f, "Diesel error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::MermaidERDError(err) => Some(err),
            Error::MermaidFlowchartError(err) => Some(err),
            Error::Diesel(err) => Some(err),
        }
    }
}
