use crate::prelude::*;

pub(crate) struct SiriusConfig<V: Version> {
    parameters: Vec<V::Parameters>,
}

impl<V: Version> Default for SiriusConfig<V> {
    fn default() -> Self {
        SiriusConfig {
            parameters: Vec::new(),
        }
    }
}

impl<V: Version> SiriusConfig<V> {
    /// Add a parameter to the configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///
    pub fn add_parameter(&mut self, parameter: V::Parameters) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .parameters
            .iter()
            .find(|&p| std::mem::discriminant(p) == std::mem::discriminant(&parameter))
        {
            Err(format!(
                concat!(
                    "The parameter {:?} cannot be added to the configuration. ",
                    "There is already an existing parameter which is {:?}. ",
                    "You cannot add it twice."
                ),
                parameter, existing_parameter
            ))
        } else {
            self.parameters.push(parameter);
            Ok(())
        }
    }

    pub fn args(&self) -> Vec<String> {
        self.parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
    }
}

impl<V: Version> ToString for SiriusConfig<V> {
    fn to_string(&self) -> String {
        self.parameters
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]

mod tests {
    use crate::sirius_parameters::SiriusParametersVersion5;

    use super::*;

    #[test]
    fn test_sirius_config() {
        let mut config: SiriusConfig<Version5> = SiriusConfig::default();
        config
            .add_parameter(SiriusParametersVersion5::MaximalMz(1000.0))
            .unwrap();
        config
            .add_parameter(SiriusParametersVersion5::IsotopeSettingsFilter(true))
            .unwrap();

        assert!(config
            .add_parameter(SiriusParametersVersion5::MaximalMz(1002.0))
            .is_err());
    }
}
