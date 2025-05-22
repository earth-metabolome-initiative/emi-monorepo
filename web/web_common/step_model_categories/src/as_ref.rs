//! Submodule providing implementations of the `AsRef` trait
//! for the `StepModelCategory` enumeration.

impl AsRef<str> for crate::StepModelCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::SamplePackaging => "SamplePackaging",
            Self::FreezeDrying => "FreezeDrying",
            Self::Labelling => "Labelling",
            Self::ScanQRCode => "ScanQRCode",
            Self::Photographing => "Photographing",
            Self::Geotagging => "Geotagging",
            Self::StoreSample => "StoreSample",
            Self::Storage => "Storage",
            Self::Weighing => "Weighing",
            Self::Aliquoting => "Aliquoting",
            Self::Sampling => "Sampling",
            Self::Printing => "Printing",
            Self::Cleaning => "Cleaning",
        }
    }
}
