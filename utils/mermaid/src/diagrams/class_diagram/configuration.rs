//! Submodule defining configuration specifically for class diagrams in
//! Mermaid.

mod builder;
pub use builder::{ClassDiagramConfigurationAttribute, ClassDiagramConfigurationBuilder};

use crate::{shared::generic_configuration::GenericConfiguration, traits::Configuration};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Configuration for class diagrams in Mermaid syntax.
pub struct ClassDiagramConfiguration {
    /// Generic configuration options which apply to all Mermaid diagrams.
    generic: GenericConfiguration,
    /// Whether to hide empty members in the class diagram.
    hide_empty_members_box: bool,
}

impl Configuration for ClassDiagramConfiguration {
    type Builder = ClassDiagramConfigurationBuilder;

    fn title(&self) -> Option<&str> {
        self.generic.title()
    }

    fn direction(&self) -> &crate::shared::generic_configuration::Direction {
        self.generic.direction()
    }

    fn markdown_auto_wrap(&self) -> bool {
        self.generic.markdown_auto_wrap()
    }

    fn renderer(&self) -> &crate::shared::generic_configuration::Renderer {
        self.generic.renderer()
    }
}
