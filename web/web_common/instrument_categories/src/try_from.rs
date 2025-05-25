//!  Submodule provding implementations of the `TryFrom` trait for the
//! [`InstrumentCategory`] enum.

use std::str::FromStr;

impl FromStr for crate::InstrumentCategory {
    type Err = crate::errors::UnknownInstrumentCategory;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "MassSpectrometer" => Self::MassSpectrometer,
            "WeightScale" => Self::WeightScale,
            "FreezeDryer" => Self::FreezeDryer,
            "BallMill" => Self::BallMill,
            "Centrifuge" => Self::Centrifuge,
            "Dispenser" => Self::Dispenser,
            "Shaker" => Self::Shaker,
            "Printer" => Self::Printer,
            _ => {
                return Err(crate::errors::UnknownInstrumentCategory::UnknownString(
                    value.to_string(),
                ));
            }
        })
    }
}

impl TryFrom<&str> for crate::InstrumentCategory {
    type Error = crate::errors::UnknownInstrumentCategory;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for crate::InstrumentCategory {
    type Error = crate::errors::UnknownInstrumentCategory;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for crate::InstrumentCategory {
    type Error = crate::errors::UnknownInstrumentCategory;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
