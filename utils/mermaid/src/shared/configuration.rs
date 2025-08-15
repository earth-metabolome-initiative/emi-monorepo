//! Submodule defining the configuration options which are applied at the top
//! level of a Mermaid diagram.

mod renderers;
pub use renderers::Renderer;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents the configuration options for a Mermaid diagram.
pub struct Configuration {
    /// The title of the diagram.
    title: Option<String>,
    /// Whether to automatically wrap markdown labels.
    markdown_auto_wrap: bool,
    /// The renderer to use for the diagram.
    renderer: Renderer,
}
