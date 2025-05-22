//!  Submodule provding implementations of the `TryFrom` trait for the
//! [`StepModelCategory`] enum.

impl TryFrom<&str> for crate::StepModelCategory {
    type Error = crate::errors::UnknownStepModelCategory;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "SamplePackaging" => Self::SamplePackaging,
            "FreezeDrying" => Self::FreezeDrying,
            "Labelling" => Self::Labelling,
            "ScanQRCode" => Self::ScanQRCode,
            "Photographing" => Self::Photographing,
            "Geotagging" => Self::Geotagging,
            "StoreSample" => Self::StoreSample,
            "Storage" => Self::Storage,
            "Weighing" => Self::Weighing,
            "Aliquoting" => Self::Aliquoting,
            "Sampling" => Self::Sampling,
            "Printing" => Self::Printing,
            "Cleaning" => Self::Cleaning,
            _ => {
                return Err(crate::errors::UnknownStepModelCategory::UnknownString(
                    value.to_string(),
                ));
            }
        })
    }
}

impl TryFrom<String> for crate::StepModelCategory {
    type Error = crate::errors::UnknownStepModelCategory;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
