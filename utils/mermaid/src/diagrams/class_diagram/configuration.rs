//! Submodule defining configuration specifically for class diagrams in
//! Mermaid.

mod builder;
use std::fmt::Display;

pub use builder::ClassDiagramConfigurationBuilder;

use crate::{
    shared::{
        Direction, Renderer,
        generic_configuration::{GenericConfiguration, Look, Theme},
    },
    traits::Configuration,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Configuration for class diagrams in Mermaid syntax.
pub struct ClassDiagramConfiguration {
    /// Generic configuration options which apply to all Mermaid diagrams.
    generic: GenericConfiguration,
    /// Whether to hide empty members in the class diagram.
    hide_empty_members_box: bool,
}

impl Display for ClassDiagramConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---")?;
        if let Some(title) = &self.title() {
            writeln!(f, "title: {title}")?;
        }
        writeln!(f, "config:")?;
        writeln!(f, "  class:")?;
        writeln!(f, "    hideEmptyMembersBox: \"{}\"", self.hide_empty_members_box)?;
        writeln!(f, "---")?;

        Ok(())
    }
}

impl Configuration for ClassDiagramConfiguration {
    type Builder = ClassDiagramConfigurationBuilder;

    fn title(&self) -> Option<&str> {
        self.generic.title()
    }

    fn direction(&self) -> Direction {
        self.generic.direction()
    }

    fn renderer(&self) -> Renderer {
        self.generic.renderer()
    }

    fn theme(&self) -> Theme {
        self.generic.theme()
    }

    fn look(&self) -> Look {
        self.generic.look()
    }
}
