#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod as_ref;
pub mod diesel_impls;
mod display;
pub mod errors;
mod try_from;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(
	feature = "diesel",
	diesel(sql_type = crate::diesel_impls::StepModelCategory)
)]
/// Enum representing different instrument categories
pub enum StepModelCategory {
    /// A step involving the packaging of samples for transport or storage.
    SamplePackaging,
    /// A step involving the removal of moisture from samples by sublimation.
    FreezeDrying,
    /// A step involving the application of labels to organisms, samples or
    /// containers.
    Labelling,
    /// A step involving the scanning of a QR code for identification or
    /// tracking purposes.
    ScanQRCode,
    /// A step involving the capture of images of organisms, samples or
    /// containers.
    Photographing,
    /// A step involving the recording of geographical coordinates for samples
    /// or organisms.
    Geotagging,
    /// A step involving the storage of a sample in a sample container.
    StoreSample,
    /// A step involving the storage of samples or organisms in a controlled
    /// environment.
    Storage,
    /// A step involving the measurement of the weight of samples or organisms.
    Weighing,
    /// A step involving the division of a sample into smaller portions for
    /// analysis or storage.
    Aliquoting,
    /// A step involving the collection of samples from organisms or
    /// environments.
    Sampling,
    /// A step involving the printing of labels or documents related to samples
    /// or organisms.
    Printing,
    /// A step involving the cleaning of samples, organisms or containers.
    Cleaning,
}

impl StepModelCategory {
    #[must_use]
    /// Returns the name of the instrument category
    pub fn name(&self) -> &'static str {
        match self {
            StepModelCategory::SamplePackaging => "Sample Packaging",
            StepModelCategory::FreezeDrying => "Freeze Drying",
            StepModelCategory::Labelling => "Labelling",
            StepModelCategory::ScanQRCode => "Scan QR Code",
            StepModelCategory::Photographing => "Photographing",
            StepModelCategory::Geotagging => "Geotagging",
            StepModelCategory::StoreSample => "Store Sample",
            StepModelCategory::Storage => "Storage",
            StepModelCategory::Weighing => "Weighing",
            StepModelCategory::Aliquoting => "Aliquoting",
            StepModelCategory::Sampling => "Sampling",
            StepModelCategory::Printing => "Printing",
            StepModelCategory::Cleaning => "Cleaning",
        }
    }

    #[must_use]
    /// Returns the description of the instrument category
    pub fn description(&self) -> &'static str {
        match self {
            StepModelCategory::SamplePackaging => {
                "A step involving the packaging of samples for transport or storage."
            }
            StepModelCategory::FreezeDrying => {
                "A step involving the removal of moisture from samples by sublimation."
            }
            StepModelCategory::Labelling => {
                "A step involving the application of labels to organisms, samples or containers."
            }
            StepModelCategory::ScanQRCode => {
                "A step involving the scanning of a QR code for identification or tracking purposes."
            }
            StepModelCategory::Photographing => {
                "A step involving the capture of images of organisms, samples or containers."
            }
            StepModelCategory::Geotagging => {
                "A step involving the recording of geographical coordinates for samples or organisms."
            }
            StepModelCategory::StoreSample => {
                "A step involving the storage of a sample in a sample container."
            }
            StepModelCategory::Storage => {
                "A step involving the storage of samples or organisms in a controlled environment."
            }
            StepModelCategory::Weighing => {
                "A step involving the measurement of the weight of samples or organisms."
            }
            StepModelCategory::Aliquoting => {
                "A step involving the division of a sample into smaller portions for analysis or storage."
            }
            StepModelCategory::Sampling => {
                "A step involving the collection of samples from organisms or environments."
            }
            StepModelCategory::Printing => {
                "A step involving the printing of labels or documents related to samples or organisms."
            }
            StepModelCategory::Cleaning => {
                "A step involving the removal of contaminants or impurities from tools, samples or organisms."
            }
        }
    }

    #[must_use]
    /// Returns the icon of the instrument category
    pub fn icon(&self) -> &'static str {
        match self {
            StepModelCategory::SamplePackaging => "sheet-plastic",
            StepModelCategory::FreezeDrying => "snowflake",
            StepModelCategory::Labelling => "tag",
            StepModelCategory::ScanQRCode => "qrcode",
            StepModelCategory::Photographing => "camera",
            StepModelCategory::Geotagging => "map-location-dot",
            StepModelCategory::StoreSample => "vial",
            StepModelCategory::Storage => "boxes-packing",
            StepModelCategory::Weighing => "weight-scale",
            StepModelCategory::Sampling | StepModelCategory::Aliquoting => "vials",
            StepModelCategory::Printing => "print",
            StepModelCategory::Cleaning => "broom",
        }
    }
}
