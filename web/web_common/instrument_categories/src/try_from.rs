//!  Submodule provding implementations of the `TryFrom` trait for the
//! [`InstrumentCategory`] enum.

impl TryFrom<&str> for crate::InstrumentCategory {
    type Error = crate::errors::UnknownInstrumentCategory;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "MassSpectrometer" => Self::MassSpectrometer,
            "WeightScale" => Self::WeightScale,
            "FreezeDryer" => Self::FreezeDryer,
            "BallMill" => Self::BallMill,
            "Centrifuge" => Self::Centrifuge,
            "Dispenser" => Self::Dispenser,
            "Shaker" => Self::Shaker,
            _ => {
                return Err(crate::errors::UnknownInstrumentCategory::UnknownString(
                    value.to_string(),
                ));
            }
        })
    }
}

impl TryFrom<String> for crate::InstrumentCategory {
    type Error = crate::errors::UnknownInstrumentCategory;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
