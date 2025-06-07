//!  Submodule provding the implementation of the `Display` trait for the
//! [`InstrumentCategory`] enum.

impl core::fmt::Display for crate::InstrumentCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            crate::InstrumentCategory::MassSpectrometer => write!(f, "MassSpectrometer"),
            crate::InstrumentCategory::WeightScale => write!(f, "WeightScale"),
            crate::InstrumentCategory::FreezeDryer => write!(f, "FreezeDryer"),
            crate::InstrumentCategory::BallMill => write!(f, "BallMill"),
            crate::InstrumentCategory::Centrifuge => write!(f, "Centrifuge"),
            crate::InstrumentCategory::Dispenser => write!(f, "Dispenser"),
            crate::InstrumentCategory::Shaker => write!(f, "Shaker"),
            crate::InstrumentCategory::Printer => write!(f, "Printer"),
        }
    }
}
