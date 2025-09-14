//! Submodule providing a builder struct for the configuration of class diagrams
//! in Mermaid syntax.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::Builder,
};

use crate::{
    diagrams::class_diagram::configuration::ClassDiagramConfiguration,
    errors::ConfigError,
    shared::generic_configuration::{GenericConfigurationAttribute, GenericConfigurationBuilder},
    traits::ConfigurationBuilder,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDiagramConfigurationBuilder {
    /// Generic configuration options which apply to all Mermaid diagrams.
    generic: GenericConfigurationBuilder,
    /// Whether to hide empty members in the class diagram.
    hide_empty_members_box: bool,
}

impl ClassDiagramConfigurationBuilder {
    /// Sets whether to hide empty members in the class diagram.
    pub fn hide_empty_members_box(mut self, hide: bool) -> Self {
        self.hide_empty_members_box = hide;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An enumeration of configuration attributes specific to class diagrams.
pub enum ClassDiagramConfigurationAttribute {
    /// Generic configuration attribute.
    Generic(GenericConfigurationAttribute),
    /// Hide empty members box attribute.
    HideEmptyMembersBox,
}

impl From<GenericConfigurationAttribute> for ClassDiagramConfigurationAttribute {
    fn from(attr: GenericConfigurationAttribute) -> Self {
        ClassDiagramConfigurationAttribute::Generic(attr)
    }
}

impl Display for ClassDiagramConfigurationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassDiagramConfigurationAttribute::Generic(attr) => write!(f, "{attr}"),
            ClassDiagramConfigurationAttribute::HideEmptyMembersBox => {
                write!(f, "hide_empty_members_box")
            }
        }
    }
}

impl IsCompleteBuilder for ClassDiagramConfigurationBuilder {
    fn is_complete(&self) -> bool {
        self.generic.is_complete()
    }
}

impl Attributed for ClassDiagramConfigurationBuilder {
    type Attribute = ClassDiagramConfigurationAttribute;
}

impl Builder for ClassDiagramConfigurationBuilder {
    type Object = ClassDiagramConfiguration;
    type Error = ConfigError<Self::Attribute>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ClassDiagramConfiguration {
            generic: self.generic.build()?,
            hide_empty_members_box: self.hide_empty_members_box,
        })
    }
}

impl ConfigurationBuilder for ClassDiagramConfigurationBuilder {
    type Configuration = ClassDiagramConfiguration;

    fn title<S: ToString>(mut self, title: S) -> Result<Self, Self::Error> {
        self.generic = self.generic.title(title)?;
        Ok(self)
    }

    fn direction(mut self, direction: crate::shared::generic_configuration::Direction) -> Self {
        self.generic = self.generic.direction(direction);
        self
    }

    fn renderer(mut self, renderer: crate::shared::generic_configuration::Renderer) -> Self {
        self.generic = self.generic.renderer(renderer);
        self
    }
}
