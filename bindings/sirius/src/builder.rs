//! A builder is a type of struct that will collect configurations and once build, prodiuces a complete struct.
//!
use crate::prelude::{Version, Version5};
use crate::sirius::Sirius;
use crate::sirius_config::SiriusConfig;
use crate::sirius_parameters::SiriusParametersVersion5;

#[derive(Default)]
pub struct SiriusBuilder<V: Version> {
    config: SiriusConfig<V>,
}

impl SiriusBuilder<Version5> {
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
    pub fn maximal_mz(mut self, maximal_mz: f64) -> Result<Self, String> {
        if maximal_mz < 0.0 {
            return Err(format!(
                concat!("Maximal m/z ratio must be positive. ", "You provided {}."),
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

        self.config
            .add_parameter(SiriusParametersVersion5::MaximalMz(maximal_mz))?;
        Ok(self)
    }

    pub fn isotope_settings_filter(
        mut self,
        isotope_settings_filter: bool,
    ) -> Result<Self, String> {
        self.config
            .add_parameter(SiriusParametersVersion5::IsotopeSettingsFilter(
                isotope_settings_filter,
            ))?;
        Ok(self)
    }

    pub fn formula_search_db(
        mut self,
        formula_search_db: crate::sirius_types::FormulaSearchDB,
    ) -> Result<Self, String> {
        self.config
            .add_parameter(SiriusParametersVersion5::FormulaSearchDB(formula_search_db))?;
        Ok(self)
    }
}
impl<V: Version> SiriusBuilder<V> {
    pub fn build(self) -> Sirius<V> {
        Sirius::from(self.config)
    }
}

impl SiriusBuilder<Version5> {
    /// Set the default maximal value of m/z ratio on which Sirius calculation will be carried.
    ///
    /// # Example
    ///
    /// ```
    /// use sirius::prelude::*;
    ///
    /// let sirius = SiriusBuilder::default()
    ///    .maximal_mz_default().unwrap()
    ///  .build();
    ///
    /// assert!(SiriusBuilder::default().maximal_mz_default().is_ok());
    ///
    /// assert!(SiriusBuilder::default().maximal_mz_default().unwrap().maximal_mz_default().is_err());
    ///
    ///
    /// ```
    pub fn maximal_mz_default(mut self) -> Result<Self, String> {
        self.config
            .add_parameter(SiriusParametersVersion5::MaximalMz(f64::default()).to_default())?;
        Ok(self)
    }

    pub fn isotope_settings_filter_default(mut self) -> Result<Self, String> {
        self.config.add_parameter(
            SiriusParametersVersion5::IsotopeSettingsFilter(bool::default()).to_default(),
        )?;
        Ok(self)
    }

    pub fn formula_search_db_default(mut self) -> Result<Self, String> {
        self.config.add_parameter(
            SiriusParametersVersion5::FormulaSearchDB(crate::sirius_types::FormulaSearchDB::default()).to_default(),
        )?;
        Ok(self)
    }
}
