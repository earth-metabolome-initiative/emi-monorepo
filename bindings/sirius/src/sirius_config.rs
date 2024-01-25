
use crate::sirius_parameters::SiriusParameters;

pub(crate) struct SiriusConfig {
    parameters: Vec<SiriusParameters>,
}

impl Default for SiriusConfig {
    fn default() -> Self {
        SiriusConfig {
            parameters: Vec::new(),
        }
    }
}

impl SiriusConfig {

    pub fn add_parameter(&mut self, parameter: SiriusParameters) -> Result<(), String>{
        // We check if the parameter is already present in the vector
        // If it is, we return an error
        self.parameters.iter().find(|&&p| std::mem::discriminant(&p) == std::mem::discriminant(&parameter))
            .map_or_else(|| {
                self.parameters.push(parameter);
                Ok(())
            }, |existing_parameter| {
                Err(format!(
                    concat!(
                        "The parameter {:?} cannot be added to the configuration. ",
                        "There is already an existing parameter which is {:?}. ",
                        "You cannot add it twice."
                    ),
                    parameter,
                    existing_parameter
                ))
            })
    }

    pub fn get_parameters(&self) -> &Vec<SiriusParameters> {
        &self.parameters
    }
}