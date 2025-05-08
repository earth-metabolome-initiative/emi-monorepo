use std::fmt::Display;

use crate::{prelude::*, traits::Enablable};

/// Struct providing the configuration for Sirius.
///
/// # Implementation details
/// This struct MUST be a private struct. It is only used by the
/// [`SiriusBuilder`](crate::builder::SiriusBuilder) to
/// build the [`Sirius`](crate::sirius::Sirius) struct, and through the builder
/// we can evaluate all of the provided parameters. If we make this struct
/// public, we would allow the user to create a
/// [`Sirius`](crate::sirius::Sirius) struct with invalid parameters. DO NOT
/// MAKE THIS STRUCT PUBLIC.
pub(crate) struct SiriusConfig<V: Version> {
    core: Vec<V::Core>,
    config: Vec<V::Config>,
    formula: Vec<V::Formula>,
    zodiac: Vec<V::Zodiac>,
    fingerprint: Vec<V::Fingerprint>,
    structure: Vec<V::Structure>,
    canopus: Vec<V::Canopus>,
    write_summaries: Vec<V::WriteSummaries>,
}

impl<V: Version> Default for SiriusConfig<V> {
    fn default() -> Self {
        SiriusConfig {
            core: Vec::new(),
            config: Vec::new(),
            formula: Vec::new(),
            zodiac: Vec::new(),
            fingerprint: Vec::new(),
            structure: Vec::new(),
            canopus: Vec::new(),
            write_summaries: Vec::new(),
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
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration.
    pub fn add_core_parameter(&mut self, parameter: V::Core) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .core
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The core parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            self.core.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the config configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration. If the parameter is not an enabler, it will try to
    /// insert the enabler variant first.
    pub fn add_config_parameter(&mut self, parameter: V::Config) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .config
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The config parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler
                // variant is present in the vector by trying to insert it
                // without checking if it is already present.
                let _ = self.add_config_parameter(V::Config::enabler());
            }
            self.config.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the formula configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration. If the parameter is not an enabler, it will try to
    /// insert the enabler variant first.
    pub fn add_formula_parameter(&mut self, parameter: V::Formula) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .formula
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The formula parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler
                // variant is present in the vector by trying to insert it
                // without checking if it is already present.
                let _ = self.add_formula_parameter(V::Formula::enabler());
            }
            self.formula.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the zodiac configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration. If the parameter is not an enabler, it will try to
    /// insert the enabler variant first.
    pub fn add_zodiac_parameter(&mut self, parameter: V::Zodiac) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .zodiac
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The zodiac parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler
                // variant is present in the vector by trying to insert it
                // without checking if it is already present.
                let _ = self.add_zodiac_parameter(V::Zodiac::enabler());
            }
            self.zodiac.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the fingerprint configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration. If the parameter is not an enabler, it will try to
    /// insert the enabler variant first.
    pub fn add_fingerprint_parameter(&mut self, parameter: V::Fingerprint) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .fingerprint
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The fingerprint parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler
                // variant is present in the vector by trying to insert it
                // without checking if it is already present.
                let _ = self.add_fingerprint_parameter(V::Fingerprint::enabler());
            }
            self.fingerprint.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the structure configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration. If the parameter is not an enabler, it will try to
    /// insert the enabler variant first.
    pub fn add_structure_parameter(&mut self, parameter: V::Structure) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .structure
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The structure parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler
                // variant is present in the vector by trying to insert it
                // without checking if it is already present.
                let _ = self.add_structure_parameter(V::Structure::enabler());
            }
            self.structure.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the canopus configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration. If the parameter is not an enabler, it will try to
    /// insert the enabler variant first.
    pub fn add_canopus_parameter(&mut self, parameter: V::Canopus) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .canopus
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The canopus parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler
                // variant is present in the vector by trying to insert it
                // without checking if it is already present.
                let _ = self.add_canopus_parameter(V::Canopus::enabler());
            }
            self.canopus.push(parameter);
            Ok(())
        }
    }

    /// Add a parameter to the `write_summaries` configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    /// # Errors
    /// Returns an error if the parameter has already been added to the
    /// configuration. If the parameter is not an enabler, it will try to
    /// insert the enabler variant first.
    pub fn add_write_summaries_parameter(
        &mut self,
        parameter: V::WriteSummaries,
    ) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .write_summaries
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                "The write_summaries parameter {parameter:?} cannot be added to the configuration. There is already an existing parameter which is {existing_parameter:?}. You cannot add it twice.",
            ))
        } else {
            if !parameter.is_enabler() {
                // If the current parameter is not an enabler, we make sure that the enabler
                // variant is present in the vector by trying to insert it
                // without checking if it is already present.
                let _ = self.add_write_summaries_parameter(V::WriteSummaries::enabler());
            }
            self.write_summaries.push(parameter);
            Ok(())
        }
    }

    #[must_use]
    pub fn args(&self) -> Vec<String> {
        self.core
            .iter()
            .map(ToString::to_string)
            .chain(self.config.iter().map(ToString::to_string))
            .chain(self.formula.iter().map(ToString::to_string))
            .chain(self.zodiac.iter().map(ToString::to_string))
            .chain(self.fingerprint.iter().map(ToString::to_string))
            .chain(self.structure.iter().map(ToString::to_string))
            .chain(self.canopus.iter().map(ToString::to_string))
            .chain(self.write_summaries.iter().map(ToString::to_string))
            .collect::<Vec<String>>()
    }
}

impl<V: Version> Display for SiriusConfig<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.args().iter().try_for_each(|arg| Display::fmt(arg, f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sirius_config() {
        let mut config: SiriusConfig<Version5> = SiriusConfig::default();
        config.add_config_parameter(ConfigV5::IsotopeSettingsFilter(true)).unwrap();
        config
            .add_config_parameter(ConfigV5::FormulaSearchDB(DBVector::from(vec![SearchDB::Bio])))
            .unwrap();

        assert!(config.add_config_parameter(ConfigV5::IsotopeSettingsFilter(true)).is_err());

        assert!(
            config
                .add_config_parameter(ConfigV5::FormulaSearchDB(DBVector::from(vec![
                    SearchDB::Bio
                ])))
                .is_err()
        );
    }
}
