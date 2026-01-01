//! Submodule providing a builder struct for the configuration of class diagrams
//! in Mermaid syntax.

use crate::{
    diagrams::class_diagram::configuration::ClassDiagramConfiguration, errors::ConfigError,
    shared::generic_configuration::GenericConfigurationBuilder, traits::ConfigurationBuilder,
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

impl TryFrom<ClassDiagramConfigurationBuilder> for ClassDiagramConfiguration {
    type Error = ConfigError;

    fn try_from(builder: ClassDiagramConfigurationBuilder) -> Result<Self, Self::Error> {
        Ok(ClassDiagramConfiguration {
            generic: builder.generic.try_into()?,
            hide_empty_members_box: builder.hide_empty_members_box,
        })
    }
}

impl ConfigurationBuilder for ClassDiagramConfigurationBuilder {
    type Configuration = ClassDiagramConfiguration;
    type Error = ConfigError;

    fn build(self) -> Result<Self::Configuration, Self::Error> {
        self.try_into()
    }

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
