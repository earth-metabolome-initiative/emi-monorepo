//! A builder is a type of struct that will collect configurations and once build, prodiuces a complete struct.
//! 
use crate::sirius_config::SiriusConfig;

#[derive(Default)]
pub struct SiriusBuilder {
    config: SiriusConfig,
}


impl SiriusBuilder {

    /// Set the maximal value of m/z ratio on which Sirius calculation will be carried.
    /// 
    /// # Arguments
    /// 
    /// * `maximal_mz` - The maximal m/z ratio.
    /// 
    /// # Example
    /// 
    /// ```
    /// use sirius::prelude::*;
    /// 
    /// let sirius = SiriusBuilder::default()
    ///    .maximal_mz(1000.0).unwrap()
    ///   .build();
    /// 
    /// assert!(SiriusBuilder::default().maximal_mz(-67.0).is_err()); 
    /// assert!(SiriusBuilder::default().maximal_mz(0.0).is_err());
    /// assert!(SiriusBuilder::default().maximal_mz(std::f64::NAN).is_err());
    /// assert!(SiriusBuilder::default().maximal_mz(std::f64::INFINITY).is_err());
    /// 
    /// ```
    /// 
    /// 
    pub fn maximal_mz(mut self, maximal_mz: f64) -> Result<Self, String>{

        if maximal_mz < 0.0 {
            return Err(format!(
                concat!(
                    "Maximal m/z ratio must be positive. ",
                    "You provided {}."
                ),
                maximal_mz
            ));
        }
        if maximal_mz == 0.0 {
            return Err("Maximal m/z ratio cannot be 0".to_string());
        }
        if maximal_mz.is_nan() {
            return Err("Maximal m/z ratio cannot be NaN".to_string());
        }
        if maximal_mz.is_infinite() {
            return Err("Maximal m/z ratio cannot be infinite".to_string());
        }


        self.maximal_mz = Some(maximal_mz);
        Ok(self)
    }

    pub fn isotope_settings_filter(mut self, isotope_settings_filter: bool) -> Self {
        self.isotope_settings_filter = Some(isotope_settings_filter);
        self
    }



}