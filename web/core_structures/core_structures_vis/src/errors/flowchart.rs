//! Submodule handling `From` trait implementations relative to Flowchart
//! errors.

use super::Error;

impl From<mermaid::FlowchartError> for Error {
    fn from(err: mermaid::FlowchartError) -> Self {
        Error::MermaidFlowchartError(err)
    }
}

impl From<mermaid::FlowchartConfigError> for Error {
    fn from(err: mermaid::FlowchartConfigError) -> Self {
        Error::MermaidFlowchartError(err.into())
    }
}

impl From<mermaid::FlowchartNodeError> for Error {
    fn from(err: mermaid::FlowchartNodeError) -> Self {
        Error::MermaidFlowchartError(err.into())
    }
}

impl From<mermaid::FlowchartEdgeError> for Error {
    fn from(err: mermaid::FlowchartEdgeError) -> Self {
        Error::MermaidFlowchartError(err.into())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Diesel(err)
    }
}
