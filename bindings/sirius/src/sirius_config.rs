use crate::prelude::*;
use crate::traits::Enablable;

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
    formula_parameters: Vec<V::Formula>,
    zodiac_parameters: Vec<V::Zodiac>,
    fingerprint_parameters: Vec<V::Fingerprint>,
    structure_parameters: Vec<V::Structure>,
    canopus_parameters: Vec<V::Canopus>,
    write_summaries_parameters: Vec<V::WriteSummaries>,
}

impl<V: Version> Default for SiriusConfig<V> {
    fn default() -> Self {
        SiriusConfig {
            core_parameters: Vec::new(),
            config_parameters: Vec::new(),
            formula_parameters: Vec::new(),
            zodiac_parameters: Vec::new(),
            fingerprint_parameters: Vec::new(),
            structure_parameters: Vec::new(),
            canopus_parameters: Vec::new(),
            write_summaries_parameters: Vec::new(),
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
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_config_parameter(V::Config::enabler());
            }
            self.config_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the formula configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_formula_parameter(&mut self, parameter: V::Formula) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .formula_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The formula parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_formula_parameter(V::Formula::enabler());
            }
            self.formula_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the zodiac configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_zodiac_parameter(&mut self, parameter: V::Zodiac) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .zodiac_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The zodiac parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_zodiac_parameter(V::Zodiac::enabler());
            }
            self.zodiac_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the fingerprint configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_fingerprint_parameter(&mut self, parameter: V::Fingerprint) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .fingerprint_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The fingerprint parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_fingerprint_parameter(V::Fingerprint::enabler());
            }
            self.fingerprint_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the structure configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_structure_parameter(&mut self, parameter: V::Structure) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .structure_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The structure parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_structure_parameter(V::Structure::enabler());
            }
            self.structure_parameters.push(parameter);
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
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_canopus_parameter(V::Canopus::enabler());
            }
            self.canopus_parameters.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the write_summaries configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_write_summaries_parameter(&mut self, parameter: V::WriteSummaries) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .write_summaries_parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The write_summaries parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler variant
                // is present in the vector by trying to insert it without checking if it is already
                // present.
                let _ = self.add_write_summaries_parameter(V::WriteSummaries::enabler());
            }
            self.write_summaries_parameters.push(parameter);
            Ok(())
        }
    }

    pub fn args(&self) -> Vec<String> {
        self.core_parameters
            .iter()
            .map(|p| p.to_string())
            .chain(self.config_parameters.iter().map(|p| p.to_string()))
            .chain(self.formula_parameters.iter().map(|p| p.to_string()))
            .chain(self.zodiac_parameters.iter().map(|p| p.to_string()))
            .chain(self.fingerprint_parameters.iter().map(|p| p.to_string()))
            .chain(self.structure_parameters.iter().map(|p| p.to_string()))
            .chain(self.canopus_parameters.iter().map(|p| p.to_string()))
            .chain(self.write_summaries_parameters.iter().map(|p| p.to_string()))
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
            .add_config_parameter(ConfigV5::FormulaSearchDB(SearchDB::Bio))
            .unwrap();

        assert!(config
            .add_config_parameter(ConfigV5::IsotopeSettingsFilter(true))
            .is_err());

        assert!(config
            .add_config_parameter(ConfigV5::FormulaSearchDB(SearchDB::Bio))
            .is_err());
    }
}
