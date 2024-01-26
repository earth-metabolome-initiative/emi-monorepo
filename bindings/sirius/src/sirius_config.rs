use std::vec;

use crate::prelude::*;

/// Struct providing the configuration for Sirius.
///
/// # Implementative details
/// This struct MUST be a private struct. It is only used by the [`SiriusBuilder`](crate::builder::SiriusBuilder) to
/// build the [`Sirius`](crate::sirius::Sirius) struct, and through the builder we can evaluate all of the provided
/// parameters. If we make this struct public, we would allow the user to create a [`Sirius`](crate::sirius::Sirius)
/// struct with invalid parameters. DO NOT MAKE THIS STRUCT PUBLIC.
pub(crate) struct SiriusConfig<V: Version> {
    core_parameters: Vec<V::Core>,
    config_parameters: Vec<V::Config>,
    canopus_parameters: Vec<V::Canopus>,
}

impl<V: Version> Default for SiriusConfig<V> {
    fn default() -> Self {
        SiriusConfig {
            core_parameters: Vec::new(),
            config_parameters: Vec::new(),
            canopus_parameters: Vec::new(),
        }
    }
}

impl<V: Version> SiriusConfig<V> {
    /// Add a parameter to the core configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_core_parameter(&mut self, parameter: V::Core) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .core_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The core parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            self.core_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the config configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_config_parameter(&mut self, parameter: V::Config) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .config_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The config parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            self.config_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the canopus configuration.
    /// 
    /// # Arguments
    /// 
    /// * `parameter` - The parameter to add.
    /// 
    pub fn add_canopus_parameter(&mut self, parameter: V::Canopus) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .canopus_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The canopus parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            self.canopus_parameters.push(parameter);
            Ok(())
        }
    }

    pub fn args(&self) -> Vec<String> {
        let config_prefix = if self.config_parameters.is_empty() {
            vec![]
        } else {
            vec!["config".to_string()]
        };

        self.core_parameters
            .iter()
            .map(|p| p.to_string())
            .chain(
                config_prefix
                    .into_iter()
                    .chain(self.config_parameters.iter().map(|p| p.to_string())),
            )
            .collect::<Vec<String>>()
    }
}

impl<V: Version> ToString for SiriusConfig<V> {
    fn to_string(&self) -> String {
        self.args().join(" ")
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_sirius_config() {
        let mut config: SiriusConfig<Version5> = SiriusConfig::default();
        config
            .add_config_parameter(ConfigV5::IsotopeSettingsFilter(true))
            .unwrap();
        config
            .add_config_parameter(ConfigV5::FormulaSearchDB(FormulaSearchDB::Bio))
            .unwrap();

        assert!(config
            .add_config_parameter(ConfigV5::IsotopeSettingsFilter(true))
            .is_err());

        assert!(config
            .add_config_parameter(ConfigV5::FormulaSearchDB(FormulaSearchDB::Bio))
            .is_err());
    }
}
