//!  Submodule provding the implementation of the `Display` trait for the
//! [`StepModelCategory`] enum.

impl core::fmt::Display for crate::StepModelCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SamplePackaging => write!(f, "SamplePackaging"),
            Self::FreezeDrying => write!(f, "FreezeDrying"),
            Self::Labelling => write!(f, "Labelling"),
            Self::ScanQRCode => write!(f, "ScanQRCode"),
            Self::Photographing => write!(f, "Photographing"),
            Self::Geotagging => write!(f, "Geotagging"),
            Self::StoreSample => write!(f, "StoreSample"),
            Self::Storage => write!(f, "Storage"),
            Self::Weighing => write!(f, "Weighing"),
            Self::Aliquoting => write!(f, "Aliquoting"),
            Self::Sampling => write!(f, "Sampling"),
            Self::Printing => write!(f, "Printing"),
            Self::Cleaning => write!(f, "Cleaning"),
        }
    }
}
