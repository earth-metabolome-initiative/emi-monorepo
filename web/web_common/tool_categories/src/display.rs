//! Submodule providing the implementation of the `Display` trait for the
//! [`ToolCategory`] enumeration.

impl core::fmt::Display for crate::ToolCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Camera => write!(f, "Camera"),
            Self::InsectTrap => write!(f, "InsectTrap"),
            Self::CuttingTool => write!(f, "CuttingTool"),
            Self::Wrapper => write!(f, "Wrapper"),
            Self::PreciseManipulationTool => write!(f, "PreciseManipulationTool"),
            Self::BreakingBeads => write!(f, "BreakingBeads"),
            Self::Gloves => write!(f, "Gloves"),
            Self::HandCleaningAgent => write!(f, "HandCleaningAgent"),
            Self::PaperTowels => write!(f, "PaperTowels"),
            Self::VolumeMeasuringTool => write!(f, "VolumeMeasuringTool"),
        }
    }
}
