//! Submodule providing implementations of the `AsRef` trait
//! for the `InstrumentCategory` enumeration.

impl AsRef<str> for crate::InstrumentCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::MassSpectrometer => "MassSpectrometer",
            Self::WeightScale => "WeightScale",
            Self::FreezeDryer => "FreezeDryer",
            Self::BallMill => "BallMill",
            Self::Centrifuge => "Centrifuge",
            Self::Dispenser => "Dispenser",
            Self::Shaker => "Shaker",
            Self::Printer => "Printer",
        }
    }
}
