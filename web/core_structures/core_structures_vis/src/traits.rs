//! Submodule defining visualization traits for core structures.

use mermaid::traits::Diagram;

/// A trait for DB types that can be converted into a Mermaid diagram.
pub trait MermaidDB<C> {
    /// The type of the diagram.
    type Diagram: Diagram;
    /// The error type that can be returned.
    type Error: From<diesel::result::Error>;

    /// Converts the implementing type into a Mermaid diagram string.
    ///
    /// # Arguments
    ///
    /// * `conn`: The database connection to use for fetching data.
    ///
    /// # Errors
    ///
    /// * If the conversion to a Mermaid diagram fails.
    /// * If the database connection fails.
    fn to_mermaid(&self, conn: &mut C) -> Result<Self::Diagram, Self::Error>;
}
