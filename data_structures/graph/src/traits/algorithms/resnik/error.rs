//! Submodule providing `Resnik` Errors for working with Resnik

use std::fmt::Display;


/// Resnik Enum for Errors that may occur during Resnik process
#[derive(Debug)]
pub enum ResnikError {
    /// Error for when a graph is not a DAG / contains a cycle
    NotDag,
}

impl Display for ResnikError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotDag => write!(f,"The graph is not a DAG")
        }
    }
}

impl core::error::Error for ResnikError {
    
}