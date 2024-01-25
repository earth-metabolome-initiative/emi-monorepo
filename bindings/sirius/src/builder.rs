//! A builder is a type of struct that will collect configurations and once build, prodiuces a complete struct.
//! 

pub struct SiriusBuilder {
    /// This is the maximal m/z ratio on which Sirius calculation will be carried.
    maximal_mz: Option<f64>,
}

impl Default for SiriusBuilder {
    fn default() -> Self {
        SiriusBuilder {
            maximal_mz: Some(800),
        }
    }
}

impl SiriusBuilder {

    pub fn maximal_mz(self, maximal_mz: f64) -> Result<Self, String>{

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

        self.maximal_mz = Some(maximal_mz);
        Ok(self)
    }

}