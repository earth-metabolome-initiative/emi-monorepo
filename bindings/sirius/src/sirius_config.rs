use crate::sirius_parameters::SiriusParameters;

#[derive(Default)]
pub(crate) struct SiriusConfig {
    parameters: Vec<SiriusParameters>,
}

impl SiriusConfig {
    /// Add a parameter to the configuration.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The parameter to add.
    ///

    pub fn add_parameter(&mut self, parameter: SiriusParameters) -> Result<(), String> {
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        if let Some(existing_parameter) = self
            .parameters
            .iter()
            .find(|&&p| std::mem::discriminant(&p) == std::mem::discriminant(&parameter))
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

impl ToString for SiriusConfig {
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
    use super::*;

    #[test]
    fn test_sirius_config() {
        let mut config = SiriusConfig::default();
        config
            .add_parameter(SiriusParameters::MaximalMz(1000.0))
            .unwrap();
        config
            .add_parameter(SiriusParameters::IsotopeSettingsFilter(true))
            .unwrap();

        assert!(config
            .add_parameter(SiriusParameters::MaximalMz(1002.0))
            .is_err());
    }
}
