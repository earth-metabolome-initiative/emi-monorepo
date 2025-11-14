//! Submodule providing an enumeration of possible errors that can occur in the
//! configuration of diagrams in Mermaid syntax.

use std::fmt::{Debug, Display};

use common_traits::prelude::BuilderError;

use crate::{
    diagrams::{
        class_diagram::ClassDiagramConfigurationAttribute,
        flowchart::FlowchartConfigurationAttribute,
    },
    shared::generic_configuration::GenericConfigurationAttribute,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Enum representing errors related to configuration in Mermaid diagrams.
pub enum ConfigError<ConfigAttr> {
    /// The provided diagram title is empty.
    EmptyTitle,
    /// An error occurred while building the configuration.
    Builder(BuilderError<ConfigAttr>),
}

impl<ConfigAttr> From<BuilderError<ConfigAttr>> for ConfigError<ConfigAttr> {
    fn from(error: BuilderError<ConfigAttr>) -> Self {
        ConfigError::Builder(error)
    }
}

impl<ConfigAttr: Display> std::fmt::Display for ConfigError<ConfigAttr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::EmptyTitle => write!(f, "Configuration title cannot be empty."),
            ConfigError::Builder(error) => write!(f, "Builder error: `{error}`"),
        }
    }
}

impl<ConfigAttr: Debug + Display + 'static> core::error::Error for ConfigError<ConfigAttr> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            ConfigError::Builder(error) => Some(error),
            ConfigError::EmptyTitle => None,
        }
    }
}

impl From<ConfigError<GenericConfigurationAttribute>>
    for ConfigError<ClassDiagramConfigurationAttribute>
{
    fn from(error: ConfigError<GenericConfigurationAttribute>) -> Self {
        match error {
            ConfigError::EmptyTitle => ConfigError::EmptyTitle,
            ConfigError::Builder(builder_error) => {
                ConfigError::Builder(builder_error.replace_field_name(From::from))
            }
        }
    }
}

impl From<ConfigError<GenericConfigurationAttribute>>
    for ConfigError<FlowchartConfigurationAttribute>
{
    fn from(error: ConfigError<GenericConfigurationAttribute>) -> Self {
        match error {
            ConfigError::EmptyTitle => ConfigError::EmptyTitle,
            ConfigError::Builder(builder_error) => {
                ConfigError::Builder(builder_error.replace_field_name(From::from))
            }
        }
    }
}
